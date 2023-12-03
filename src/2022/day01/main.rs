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
    let mut max: [usize; 3] = [0, 0, 0];
    let mut elf = 0;
    for line in lines {
        if line.len() == 0 {
            let i = max
                .iter()
                .position(|u| u == max.iter().min().unwrap())
                .unwrap();
            if elf > max[i] {
                max[i] = elf;
            }
            elf = 0;
            continue;
        }
        elf += line.parse::<usize>().unwrap();
    }
    println!("Part 1: {}", max.iter().max().unwrap());
    println!("Part 2: {}", max.iter().sum::<usize>());
}
