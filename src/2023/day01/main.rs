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
    let lines = read_lines(args.input);

    let pt1: i64 = lines.iter().map(|line| parse_line_pt1(line)).sum();
    let pt2: i64 = lines.iter().map(|line| parse_line_pt2(line)).sum();
    println!("Part 1: {}", pt1);
    println!("Part 1: {}", pt2);
}

fn parse_line_pt1(s: &str) -> i64 {
    // TODO: better error checking
    let first = s.find(char::is_numeric).unwrap();
    let last = s.rfind(char::is_numeric).unwrap();
    return (s.chars().nth(first).expect("index").to_digit(10).unwrap() as i64) * 10
        + (s.chars().nth(last).expect("index").to_digit(10).unwrap() as i64);
}

fn found_to_digit(s: &str, options: &[(Option<usize>, i32)], find_min: bool) -> i64 {
    let mut mindex: Option<usize> = None;
    let mut mval: i64 = -1;
    for (index, value) in options {
        if let Some(i) = index {
            if mindex == None
                || (find_min && *i < mindex.unwrap())
                || (!find_min && *i > mindex.unwrap())
            {
                mindex = Some(*i);
                mval = *value as i64;
            }
        }
    }
    if mval >= 0 {
        return mval;
    }

    return s
        .chars()
        .nth(mindex.unwrap())
        .expect("index")
        .to_digit(10)
        .unwrap() as i64;
}

fn parse_line_pt2(s: &str) -> i64 {
    let first = [
        (s.find(char::is_numeric), -1),
        (s.find("zero"), 0),
        (s.find("one"), 1),
        (s.find("two"), 2),
        (s.find("three"), 3),
        (s.find("four"), 4),
        (s.find("five"), 5),
        (s.find("six"), 6),
        (s.find("seven"), 7),
        (s.find("eight"), 8),
        (s.find("nine"), 9),
    ];
    let first_digit = found_to_digit(s, &first, true);
    let last = [
        (s.rfind(char::is_numeric), -1),
        (s.rfind("zero"), 0),
        (s.rfind("one"), 1),
        (s.rfind("two"), 2),
        (s.rfind("three"), 3),
        (s.rfind("four"), 4),
        (s.rfind("five"), 5),
        (s.rfind("six"), 6),
        (s.rfind("seven"), 7),
        (s.rfind("eight"), 8),
        (s.rfind("nine"), 9),
    ];
    let last_digit = found_to_digit(s, &last, false);
    return first_digit * 10 + last_digit;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for (input, want) in tests {
            assert_eq!(parse_line_pt1(input), want);
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
        ];

        for (input, want) in tests {
            assert_eq!(parse_line_pt2(input), want);
        }
    }
}
