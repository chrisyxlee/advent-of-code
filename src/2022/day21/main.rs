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

    {
        let (mut known, mut unknown) = parse_input_pt1(&lines);
        solve_pt1(&mut known, &mut unknown);
        println!("Part 1: {}", known[&String::from("root")]);
    }
    {
        let (mut known, mut unknown) = parse_input_pt2(&lines);
        solve_pt2(&mut known, &mut unknown);
        println!("Part 2: {}", known[&String::from("humn")]);
    }
}

#[derive(Debug)]
pub struct Operation {
    tok1: String,
    op: char,
    tok2: String,
    is_val: bool,
}

fn parse_input_pt1(lines: &Vec<String>) -> (HashMap<String, i64>, HashMap<String, Operation>) {
    let mut known_values: HashMap<String, i64> = HashMap::new();
    let mut unknown_values: HashMap<String, Operation> = HashMap::new();

    let number_re = Regex::new(r"(\w+): (\d+)").unwrap();
    let operation_re = Regex::new(r"(\w+): (\w+) (.) (\w+)").unwrap();

    for line in lines {
        let mut name = String::from("");
        if number_re.is_match(&line) {
            let mut val: i64 = 0;
            for m in number_re.captures_iter(&line) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        match i {
                            1 => name = String::from(sub.as_str()),
                            2 => val = sub.as_str().parse::<i64>().unwrap(),
                            _ => (),
                        }
                    }
                }
            }
            known_values.entry(name).or_insert(val);
        } else if operation_re.is_match(&line) {
            let mut tok1 = String::from("");
            let mut tok2 = String::from("");
            let mut op = ' ';
            for m in operation_re.captures_iter(&line) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        match i {
                            1 => name = String::from(sub.as_str()),
                            2 => tok1 = String::from(sub.as_str()),
                            3 => op = sub.as_str().chars().collect::<Vec<char>>()[0],
                            4 => tok2 = String::from(sub.as_str()),
                            _ => (),
                        }
                    }
                }
            }
            unknown_values.entry(name).or_insert(Operation {
                tok1: tok1,
                op: op,
                tok2: tok2,
                is_val: false,
            });
        } else {
            assert!(false, "Line did not match: {}", line);
        }
    }

    return (known_values, unknown_values);
}

fn solve_pt1(known: &mut HashMap<String, i64>, unknown: &mut HashMap<String, Operation>) {
    while unknown.len() > 0 {
        let mut to_remove: Vec<String> = Vec::new();
        for key in unknown.keys() {
            let op: &Operation = &unknown[key];
            if known.contains_key(&op.tok1) && known.contains_key(&op.tok2) {
                let val = match op.op {
                    '-' => known[&op.tok1] - known[&op.tok2],
                    '*' => known[&op.tok1] * known[&op.tok2],
                    '/' => known[&op.tok1] / known[&op.tok2],
                    '+' => known[&op.tok1] + known[&op.tok2],
                    _ => {
                        assert!(false, "Invalid op: {}", op.op);
                        0
                    }
                };
                *known.entry(key.clone()).or_insert(val) = val;
                to_remove.push(key.clone());
            }
        }
        for del in to_remove {
            unknown.remove(&del);
        }
    }
}

fn parse_input_pt2(lines: &Vec<String>) -> (HashMap<String, i64>, HashMap<String, Operation>) {
    let mut known_values: HashMap<String, i64> = HashMap::new();
    let mut unknown_values: HashMap<String, Operation> = HashMap::new();

    let number_re = Regex::new(r"(\w+): (\d+)").unwrap();
    let operation_re = Regex::new(r"(\w+): (\w+) (.) (\w+)").unwrap();

    for line in lines {
        let mut name = String::from("");
        if number_re.is_match(&line) {
            let mut val: i64 = 0;
            for m in number_re.captures_iter(&line) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        match i {
                            1 => name = String::from(sub.as_str()),
                            2 => val = sub.as_str().parse::<i64>().unwrap(),
                            _ => (),
                        }
                    }
                }
            }
            match name.as_str() {
                "humn" => {
                    unknown_values.entry(name).or_insert(Operation {
                        is_val: true,
                        tok1: String::from(""),
                        tok2: String::from(""),
                        op: ' ',
                    });
                    ()
                }
                _ => *known_values.entry(name).or_insert(val) = val,
            }
        } else if operation_re.is_match(&line) {
            let mut tok1 = String::from("");
            let mut tok2 = String::from("");
            let mut op = ' ';
            for m in operation_re.captures_iter(&line) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        match i {
                            1 => name = String::from(sub.as_str()),
                            2 => tok1 = String::from(sub.as_str()),
                            3 => match name.as_str() {
                                "root" => op = '=',
                                _ => op = sub.as_str().chars().collect::<Vec<char>>()[0],
                            },
                            4 => tok2 = String::from(sub.as_str()),
                            _ => (),
                        }
                    }
                }
            }
            unknown_values.entry(name).or_insert(Operation {
                tok1: tok1,
                op: op,
                tok2: tok2,
                is_val: false,
            });
        } else {
            assert!(false, "Line did not match: {}", line);
        }
    }

    return (known_values, unknown_values);
}

fn solve_pt2(known: &mut HashMap<String, i64>, unknown: &mut HashMap<String, Operation>) {
    while unknown.len() > 0 {
        let mut to_remove: Vec<String> = Vec::new();
        let mut did_something = false;
        for key in unknown.keys() {
            let op = &unknown[key];
            if !op.is_val {
                if op.op == '=' {
                    if known.contains_key(&op.tok1) {
                        let val = known[&op.tok1];
                        *known.entry(op.tok2.clone()).or_insert(val) = val;
                        to_remove.push(key.clone());
                    } else if known.contains_key(&op.tok2) {
                        let val = known[&op.tok2];
                        *known.entry(op.tok1.clone()).or_insert(val) = val;
                        to_remove.push(key.clone());
                    }
                } else if known.contains_key(&op.tok1) && known.contains_key(&op.tok2) {
                    let val = match op.op {
                        '-' => known[&op.tok1] - known[&op.tok2],
                        '*' => known[&op.tok1] * known[&op.tok2],
                        '/' => known[&op.tok1] / known[&op.tok2],
                        '+' => known[&op.tok1] + known[&op.tok2],
                        _ => {
                            assert!(false, "Invalid op: {}", op.op);
                            0
                        }
                    };
                    *known.entry(key.clone()).or_insert(val) = val;
                    to_remove.push(key.clone());
                }
            }
        }
        for del in to_remove {
            unknown.remove(&del);
            did_something = true;
        }
        if !did_something {
            break;
        }
    }

    let mut did_something = true;
    while unknown.len() > 0 && did_something {
        let mut to_remove: Vec<String> = Vec::new();
        did_something = false;
        for key in unknown.keys() {
            let op = &unknown[key];
            if !op.is_val {
                if known.contains_key(key) {
                    if known.contains_key(&op.tok1) {
                        let val = match op.op {
                            // tok2 = tok1 - NAME
                            '-' => known[&op.tok1] - known[key],
                            // NAME = tok 1 * tok2
                            '*' => known[key] / known[&op.tok1],
                            // NAME = tok1 / tok2
                            '/' => known[&op.tok1] / known[key],
                            // NAME = tok 1 + tok 2
                            '+' => known[key] - known[&op.tok1],
                            _ => {
                                assert!(false, "Invalid op: {}", op.op);
                                0
                            }
                        };
                        *known.entry(op.tok2.clone()).or_insert(val) = val;
                        to_remove.push(key.clone());
                    } else if known.contains_key(&op.tok2) {
                        let val = match op.op {
                            // NAME = tok1 - tok2
                            '-' => known[key] + known[&op.tok2],
                            // NAME = tok 1 * tok2
                            '*' => known[key] / known[&op.tok2],
                            // NAME = tok1 / tok2
                            '/' => known[key] * known[&op.tok2],
                            // NAME = tok 1 + tok 2
                            '+' => known[key] - known[&op.tok2],
                            _ => {
                                assert!(false, "Invalid op: {}", op.op);
                                0
                            }
                        };
                        *known.entry(op.tok1.clone()).or_insert(val) = val;
                        to_remove.push(key.clone());
                    }
                }
            } else {
                if known.contains_key(key) {
                    to_remove.push(key.clone());
                }
            }
        }
        for del in to_remove {
            unknown.remove(&del);
            did_something = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = Vec::from([
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ])
        .iter()
        .map(|&x| String::from(x))
        .collect::<Vec<String>>();

        {
            let (mut known, mut unknown) = parse_input_pt1(&input);
            solve_pt1(&mut known, &mut unknown);
            assert_eq!(152, known[&String::from("root")]);
        }
        {
            let (mut known, mut unknown) = parse_input_pt2(&input);
            solve_pt2(&mut known, &mut unknown);
            println!("known\n  {:?}\n\nunknown\n  {:?}", known, unknown);
            assert_eq!(301, known[&String::from("humn")]);
        }
    }
}
