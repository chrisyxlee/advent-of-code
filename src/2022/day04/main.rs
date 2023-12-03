use advent_of_code::utils::input::read_lines;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file.
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let lines = &read_lines(args.input);

    let mut count = 0;

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if parse_line(line) {
            count += 1
        }
    }
    println!("Total overlapping: {}", count);
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
