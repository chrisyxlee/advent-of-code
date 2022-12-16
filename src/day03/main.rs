use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("In file {}", file_path);

    let mut priority = 0;
    let mut left_freqs: HashSet<char> = HashSet::new();
    let mut middle_freqs: HashSet<char> = HashSet::new();
    let mut right_freqs: HashSet<char> = HashSet::new();

    // File must exist in current path before this produces output
    let mut i = 0;
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                for c in ip.chars() {
                    match i {
                        0 => left_freqs.insert(c),
                        1 => middle_freqs.insert(c),
                        2 => right_freqs.insert(c),
                        _ => true,
                    };
                }
                i = (i + 1) % 3;
                if i == 0 {
                    let cpy = left_freqs.clone();
                    let inter = cpy
                        .into_iter()
                        .filter(|&k| middle_freqs.contains(&k))
                        .filter(|&k| right_freqs.contains(&k))
                        .collect::<Vec<char>>();
                    assert!(inter.len() == 1);
                    let mut dup: char = 0 as char;
                    for c in inter.into_iter() {
                        dup = c;
                    }
                    priority += to_priority(dup);

                    left_freqs.clear();
                    middle_freqs.clear();
                    right_freqs.clear();
                }
            }
        }
        println!("Total priority: {}", priority);
    }
}

fn to_priority(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as u32 - 'a' as u32) as i32 + 1;
    }

    return (c as u32 - 'A' as u32) as i32 + 27;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
