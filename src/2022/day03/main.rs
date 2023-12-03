use advent_of_code::utils::input::read_lines;
use clap::Parser;
use std::collections::HashSet;

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

    let mut priority = 0;
    let mut left_freqs: HashSet<char> = HashSet::new();
    let mut middle_freqs: HashSet<char> = HashSet::new();
    let mut right_freqs: HashSet<char> = HashSet::new();

    // File must exist in current path before this produces output
    let mut i = 0;
    for line in lines {
        for c in line.chars() {
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
    println!("Total priority: {}", priority);
}

fn to_priority(c: char) -> i32 {
    if c.is_lowercase() {
        return (c as u32 - 'a' as u32) as i32 + 1;
    }

    return (c as u32 - 'A' as u32) as i32 + 27;
}
