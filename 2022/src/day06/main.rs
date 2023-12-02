use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day06/input.txt";
    println!("In file {}", file_path);

    let mut uniques: HashMap<char, i32> = HashMap::new();
    let size = 14;
    let mut circular: Vec<char> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        circular.push(0 as char);
    }
    let mut next_idx = 0;

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                for (i, c) in ip.chars().enumerate() {
                    if i >= size {
                        let key = circular[next_idx];
                        if uniques.contains_key(&key) {
                            let count = uniques.get_mut(&key).unwrap();
                            *count -= 1;
                            if *count <= 0 {
                                uniques.remove(&key);
                            }
                        }
                    }

                    circular[next_idx] = c;
                    next_idx = (next_idx + 1) % size;
                    let count = uniques.entry(c).or_insert(0);
                    *count = *count + 1;

                    if i >= size {
                        if uniques.len() == size {
                            println!("Parsed {} characters", i + 1);
                            return;
                        }
                    }
                }
            }
        }
    }
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
