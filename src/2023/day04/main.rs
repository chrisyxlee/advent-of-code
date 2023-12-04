use advent_of_code::utils::input::read_lines;
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
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
    let lines = read_lines(args.input);

    let pt1: i32 = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
    let pt2: i32 = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
}

fn parse_line_pt1(line: &str) -> i32 {
    let line_re = Regex::new(r"Card\s+(\d+):\s+(.*)\s+\|\s+(.*)").unwrap();
    let mut winning_set: HashSet<i32> = HashSet::new();

    let mut match_count = 0;
    for m in line_re.captures_iter(line) {
        for (i, capt) in m.iter().enumerate() {
            if let Some(sub) = capt {
                match i {
                    2 => {
                        for n in sub
                            .as_str()
                            .split(" ")
                            .filter(|x| x.len() > 0)
                            .map(|x| x.parse::<i32>().unwrap())
                        {
                            winning_set.insert(n);
                        }
                    }
                    3 => {
                        for n in sub
                            .as_str()
                            .split(" ")
                            .filter(|x| x.len() > 0)
                            .map(|x| x.parse::<i32>().unwrap())
                        {
                            if winning_set.contains(&n) {
                                match_count += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if match_count == 0 {
        return 0;
    }

    let base: i32 = 2;
    base.pow(match_count - 1)
}

fn handle_pt1(lines: &Vec<String>) -> i32 {
    lines.iter().map(|l| parse_line_pt1(l)).sum()
}

fn parse_line_pt2(line: &str) -> (i32, Vec<i32>) {
    let line_re = Regex::new(r"Card\s+(\d+):\s+(.*)\s+\|\s+(.*)").unwrap();
    let mut winning_set: HashSet<i32> = HashSet::new();

    let mut card_id = 0;
    let mut match_count = 0;
    for m in line_re.captures_iter(line) {
        for (i, capt) in m.iter().enumerate() {
            if let Some(sub) = capt {
                match i {
                    1 => card_id = sub.as_str().parse::<i32>().unwrap(),
                    2 => {
                        for n in sub
                            .as_str()
                            .split(" ")
                            .filter(|x| x.len() > 0)
                            .map(|x| x.parse::<i32>().unwrap())
                        {
                            winning_set.insert(n);
                        }
                    }
                    3 => {
                        for n in sub
                            .as_str()
                            .split(" ")
                            .filter(|x| x.len() > 0)
                            .map(|x| x.parse::<i32>().unwrap())
                        {
                            if winning_set.contains(&n) {
                                match_count += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    (
        card_id,
        (1..=match_count)
            .into_iter()
            .map(|x| card_id + x)
            .collect::<Vec<i32>>(),
    )
}

fn handle_pt2(lines: &Vec<String>) -> i32 {
    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    let mut max_id = 0;
    for line in lines {
        let (id, copies) = parse_line_pt2(line);
        *frequencies.entry(id).or_insert(0) += 1;
        if id > max_id {
            max_id = id;
        }

        let copy_times = *frequencies.get(&id).unwrap();
        for new_copy in copies.iter() {
            *frequencies.entry(*new_copy).or_insert(0) += copy_times;
        }
    }

    (1..=max_id)
        .into_iter()
        .map(|id| frequencies.get(&id).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(
            vec![
                String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
                String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
                String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
                String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
                String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            ],
            13,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [(
            vec![
                String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
                String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
                String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
                String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
                String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            ],
            30,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
