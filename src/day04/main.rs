use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("In file {}", file_path);

    let mut count = 0;

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if parse_line(&ip) {
                    count += 1
                }
            }
        }
        println!("Total overlapping: {}", count);
    }
}

fn range_overlaps(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    // not overlapping
    return !(end1 < start2 || end2 < start1);
}

fn parse_range(rng: &str) -> (i32, i32) {
    let parts = rng.split("-").collect::<Vec<&str>>();
    return (
        parts[0].parse::<i32>().unwrap(),
        parts[1].parse::<i32>().unwrap(),
    );
}

fn parse_line(line: &String) -> bool {
    let parts = line.split(",").collect::<Vec<&str>>();

    let (r1_start, r1_end) = parse_range(parts[0]);
    let (r2_start, r2_end) = parse_range(parts[1]);

    return range_overlaps(r1_start, r1_end, r2_start, r2_end);
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
