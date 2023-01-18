use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day01/input.txt";
    let mut max: [usize; 3] = [0, 0, 0];
    let mut elf = 0;
    for line in &read_lines(file_path) {
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

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect::<Vec<String>>();
}
