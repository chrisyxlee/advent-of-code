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
    //  let pt2: i32 = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
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

            // println!("current is {}", current);

            steps += 1;
            if current == "ZZZ" {
                return steps;
            }
        }
    }
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
}
