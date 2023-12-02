use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day01/input.txt";
    let lines = &read_lines(file_path);
    let pt1: i64 = lines.iter().map(|line| parse_line_pt1(line)).sum();
    println!("Part 1: {}", pt1);
}

fn parse_line_pt1(s: &str) -> i64 {
    return s.chars().fold(0, |sum: i64, x| match x {
        '(' => sum + 1,
        ')' => sum - 1,
        _ => sum,
    });
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
    fn test_parsing_pt1() {
        let tests = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, want) in tests {
            assert_eq!(parse_line_pt1(input), want, "for input {}", input);
        }
    }
}
