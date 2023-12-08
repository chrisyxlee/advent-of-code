use advent_of_code::utils::input::read_lines;
use clap::Parser;
use regex::Regex;
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
    let lines = read_lines(args.input);

    let pt1: i32 = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
    let pt2: i64 = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
}

fn parse_guide(line: &String) -> (String, (String, String)) {
    let line_re = Regex::new(r"(\w+)\s+=\s+\((\w+),\s+(\w+)\)").unwrap();

    let mut source = String::from("");
    let mut left = String::from("");
    let mut right = String::from("");

    for m in line_re.captures_iter(line) {
        for (i, capt) in m.iter().enumerate() {
            if let Some(sub) = capt {
                match i {
                    1 => source = sub.as_str().to_string(),
                    2 => left = sub.as_str().to_string(),
                    3 => right = sub.as_str().to_string(),
                    _ => {}
                }
            }
        }
    }

    (source, (left, right))
}

fn handle_pt1(lines: &Vec<String>) -> i32 {
    let directions = lines[0].chars().collect::<Vec<char>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut current: String = String::from("AAA");
    for i in 2..lines.len() {
        let (source, (left, right)) = parse_guide(&lines[i]);
        map.insert(source.clone(), (left.clone(), right.clone()));
    }

    let mut steps = 0;
    loop {
        for dir in &directions {
            let (left, right) = map.get(&current).unwrap();
            current = match dir {
                'L' => left,
                'R' => right,
                _ => todo!(),
            }
            .clone();

            steps += 1;
            if current == "ZZZ" {
                return steps;
            }
        }
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn handle_pt2(lines: &Vec<String>) -> i64 {
    let directions = lines[0].chars().collect::<Vec<char>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut current: Vec<String> = Vec::new();
    for i in 2..lines.len() {
        let (source, (left, right)) = parse_guide(&lines[i]);
        map.insert(source.clone(), (left.clone(), right.clone()));
        if source.ends_with('A') {
            current.push(source.clone());
        }
    }

    let mut first_zs: Vec<i64> = vec![0; current.len()];

    for i in 0..current.len() {
        let mut c = current[i].clone();
        let mut steps = 0;
        loop {
            let mut break_outer = false;
            for dir in &directions {
                let (left, right) = map.get(&c).unwrap();
                c = match dir {
                    'L' => left,
                    'R' => right,
                    _ => todo!(),
                }
                .clone();

                steps += 1;
                if c.ends_with('Z') {
                    first_zs[i] = steps;
                    break_outer = true;
                    break;
                }
            }
            if break_outer {
                break;
            }
        }
    }

    first_zs.iter().fold(1, |acc: i64, x| lcm(acc, *x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [
            (
                vec![
                    String::from("RL"),
                    String::from(""),
                    String::from("AAA = (BBB, CCC)"),
                    String::from("BBB = (DDD, EEE)"),
                    String::from("CCC = (ZZZ, GGG)"),
                    String::from("DDD = (DDD, DDD)"),
                    String::from("EEE = (EEE, EEE)"),
                    String::from("GGG = (GGG, GGG)"),
                    String::from("ZZZ = (ZZZ, ZZZ)"),
                ],
                2,
            ),
            (
                vec![
                    String::from("LLR"),
                    String::from(""),
                    String::from("AAA = (BBB, BBB)"),
                    String::from("BBB = (AAA, ZZZ)"),
                    String::from("ZZZ = (ZZZ, ZZZ)"),
                ],
                6,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [(
            vec![
                String::from("LR"),
                String::from(""),
                String::from("11A = (11B, XXX)"),
                String::from("11B = (XXX, 11Z)"),
                String::from("11Z = (11B, XXX)"),
                String::from("22A = (22B, XXX)"),
                String::from("22B = (22C, 22C)"),
                String::from("22C = (22Z, 22Z)"),
                String::from("22Z = (22B, 22B)"),
                String::from("XXX = (XXX, XXX)"),
            ],
            6,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
