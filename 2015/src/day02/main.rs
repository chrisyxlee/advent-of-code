use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day02/input.txt";
    let lines = &read_lines(file_path);
    let pt1: i64 = lines.iter().map(|line| parse_line_pt1(line)).sum();
    println!("Part 1: {}", pt1);
    //  let pt2: i64 = lines.iter().map(|line| parse_line_pt2(line)).sum();
    //  println!("Part 2: {}", pt2);
}

fn parse_line_pt1(s: &str) -> i64 {
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

    let sides = [x * y, x * z, y * z];
    return 2 * sides.iter().sum::<i64>() + sides.iter().min().unwrap();
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
            assert_eq!(parse_line_pt1(input), want, "for input {}", input);
        }
    }
}
