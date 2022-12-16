use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    let num_elves = args[2].parse::<i32>().unwrap();
    println!("In file {}", file_path);

    let mut v: Vec<i32> = Vec::new();
    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        let mut max = 0;
        let mut elf = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    if elf > max {
                        max = elf;
                    }
                    v.push(elf);
                    elf = 0;
                } else {
                    let calories = ip.parse::<i32>().unwrap();
                    elf += calories;
                }
            }
        }
        println!("Most calories: {}", max);
    }
    v.sort();
    let mut total = 0;
    for i in v.len() - 3..v.len() {
        total += v[i];
    }
    println!("Sum of {} elves: {} calories", num_elves, total);
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
