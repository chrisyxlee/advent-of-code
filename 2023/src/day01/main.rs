use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day01/input.txt";
    let total: i64 = read_lines(file_path)
        .iter()
        .map(|line| parse_line(line))
        .sum();
    println!("Part 1: {}", total);
    //  println!("Part 2: {}", max.iter().sum::<usize>());
}

fn parse_line(s: &str) -> i64 {
    let first = s.find(char::is_numeric).unwrap();
    let last = s.rfind(char::is_numeric).unwrap();
    return (s.chars().nth(first).expect("index").to_digit(10).unwrap() as i64) * 10
        + (s.chars().nth(last).expect("index").to_digit(10).unwrap() as i64);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let tests = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (input, want) in tests {
            assert!(parse_line(input) == want);
        }
    }
}
