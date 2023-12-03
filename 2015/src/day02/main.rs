use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day02/input.txt";
    let lines = &read_lines(file_path);
    let pt1: i64 = lines
        .iter()
        .map(|s| parse_line(s))
        .map(|x| compute_pt1(x))
        .sum();
    println!("Part 1: {}", pt1);
    let pt2: i64 = lines
        .iter()
        .map(|s| parse_line(s))
        .map(|x| compute_pt2(x))
        .sum();
    println!("Part 2: {}", pt2);
}

fn parse_line(s: &str) -> (i64, i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let dim_re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    for m in dim_re.captures_iter(s) {
        for (i, capt) in m.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if let Some(sub) = capt {
                let val = sub.as_str().parse::<i64>().unwrap();
                match i {
                    1 => {
                        x = val;
                    }
                    2 => {
                        y = val;
                    }
                    3 => {
                        z = val;
                    }
                    _ => {}
                }
            }
        }
    }

    return (x, y, z);
}

fn compute_pt1((x, y, z): (i64, i64, i64)) -> i64 {
    let sides = [x * y, x * z, y * z];
    return 2 * sides.iter().sum::<i64>() + sides.iter().min().unwrap();
}

fn compute_pt2((x, y, z): (i64, i64, i64)) -> i64 {
    let mut ordered = [x, y, z];
    ordered.sort();

    return 2 * (ordered[0] + ordered[1]) + x * y * z;
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
        let tests = [("2x3x4", 58), ("1x1x10", 43)];

        for (input, want) in tests {
            assert_eq!(compute_pt1(parse_line(input)), want, "for input {}", input);
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [("2x3x4", 34), ("1x1x10", 14)];

        for (input, want) in tests {
            assert_eq!(compute_pt2(parse_line(input)), want, "for input {}", input);
        }
    }
}
