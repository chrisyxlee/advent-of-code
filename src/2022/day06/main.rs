use advent_of_code::utils::input::read_lines;
use clap::Parser;
use std::collections::HashMap;

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

    let mut uniques: HashMap<char, i32> = HashMap::new();
    let size = 14;
    let mut circular: Vec<char> = Vec::with_capacity(size as usize);
    for _i in 0..size {
        circular.push(0 as char);
    }
    let mut next_idx = 0;

    for line in lines {
        for (i, c) in line.chars().enumerate() {
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
