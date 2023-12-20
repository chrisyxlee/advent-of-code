use std::collections::HashMap;

use advent_of_code::utils::input::read_lines;
use clap::Parser;
use regex::Regex;
use std::fmt;

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

    println!("Part 1: {}", handle_pt1(&lines));
    //  println!("Part 2: {}", handle_pt2(&lines));
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Operation {
    GT,
    LT,
    LE,
    GE,
}

pub struct Rule {
    var: char,
    op: Operation,
    val: i64,
    dst: String,
}

impl Rule {
    fn run(&self, value: Value) -> Option<String> {
        let var = match self.var {
            'x' => value.x,
            'm' => value.m,
            'a' => value.a,
            's' => value.s,
            _ => unreachable!(),
        };

        let result = match self.op {
            Operation::GT => var > self.val,
            Operation::LT => var < self.val,
            Operation::GE => var >= self.val,
            Operation::LE => var <= self.val,
        };

        if result {
            return Some(self.dst.clone());
        }

        None
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}:{}",
            self.var,
            match self.op {
                Operation::GT => ">",
                Operation::LT => "<",
                Operation::GE => ">=",
                Operation::LE => "<=",
            },
            self.val,
            self.dst
        )
    }
}

pub struct Checker {
    rules: Vec<Rule>,
    otherwise: String,
}

impl Checker {
    fn check(&self, value: Value) -> String {
        for rule in &self.rules {
            // print!(" --> rule {} --> ", rule);
            if let Some(dst) = rule.run(value) {
                // println!("ok");
                return dst;
            }
            // println!("nope");
        }

        //   println!(" --> therefore, {}", self.otherwise);
        self.otherwise.clone()
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Value {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct ValueRange {
    x_min: i64,
    x_max: i64,
    m_min: i64,
    m_max: i64,
    a_min: i64,
    a_max: i64,
    s_min: i64,
    s_max: i64,
}

impl ValueRange {
    fn new() -> Self {
        Self {
            x_min: 1,
            x_max: 4000,
            m_min: 1,
            m_max: 4000,
            a_min: 1,
            a_max: 4000,
            s_min: 1,
            s_max: 4000,
        }
    }

    fn update(&mut self, rule: &Rule, pass: bool) {
        let op = match (rule.op, pass) {
            (_, true) => rule.op,
            (Operation::GT, false) => Operation::LE,
            (Operation::LT, false) => Operation::GE,
            _ => unreachable!(),
        };
        match op {
            Operation::GT => match rule.var {
                'x' => self.x_min = rule.val + 1,
                'm' => self.m_min = rule.val + 1,
                'a' => self.a_min = rule.val + 1,
                's' => self.s_min = rule.val + 1,
                _ => unreachable!(),
            },
            Operation::LT => match rule.var {
                'x' => self.x_max = rule.val - 1,
                'm' => self.m_max = rule.val - 1,
                'a' => self.a_max = rule.val - 1,
                's' => self.s_max = rule.val - 1,
                _ => unreachable!(),
            },
            Operation::LE => match rule.var {
                'x' => self.x_max = rule.val,
                'm' => self.m_max = rule.val,
                'a' => self.a_max = rule.val,
                's' => self.s_max = rule.val,
                _ => unreachable!(),
            },
            Operation::GE => match rule.var {
                'x' => self.x_min = rule.val,
                'm' => self.m_min = rule.val,
                'a' => self.a_min = rule.val,
                's' => self.s_min = rule.val,
                _ => unreachable!(),
            },
        }
    }

    fn combinations(&self) -> i64 {
        *vec![
            (self.x_max - self.x_min)
                * (self.m_max - self.m_min)
                * (self.a_max - self.a_min)
                * (self.s_max - self.s_min),
            0,
        ]
        .iter()
        .max()
        .unwrap()
    }
}

impl Ord for ValueRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x_min
            .cmp(&other.x_min)
            .then(self.x_max.cmp(&other.x_max))
            .then(self.m_min.cmp(&other.m_min))
            .then(self.m_max.cmp(&other.m_max))
            .then(self.a_min.cmp(&other.a_min))
            .then(self.a_max.cmp(&other.a_max))
            .then(self.s_min.cmp(&other.s_min))
            .then(self.s_max.cmp(&other.s_max))
    }
}

impl PartialOrd for ValueRange {
   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       Some(self.cmp(other))
   }
}

impl fmt::Display for ValueRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(x=[{},{}],m=[{},{}],a=[{},{}],s=[{},{}])",
            self.x_min,
            self.x_max,
            self.m_min,
            self.m_max,
            self.a_min,
            self.a_max,
            self.s_min,
            self.s_max
        )
    }
}

impl Value {
    fn accepted(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={},m={},a={},s={}", self.x, self.m, self.a, self.s)
    }
}

fn parse_checker(line: &str) -> (String, Checker) {
    let line_re = Regex::new(r"(.*)\{(.*)\}").unwrap();
    let rule_re = Regex::new(r"(\w)(<|>)(\d+):(\w+)").unwrap();

    let mut name = "".to_string();
    let mut checker = Checker {
        rules: Vec::new(),
        otherwise: String::from(""),
    };

    for m in line_re.captures_iter(line) {
        for (i, capt) in m.iter().enumerate() {
            if let Some(sub) = capt {
                match i {
                    1 => name = sub.as_str().to_string(),
                    2 => {
                        let parts = sub.as_str().split(",").collect::<Vec<&str>>();
                        for (i, part) in parts.iter().enumerate() {
                            if i == parts.len() - 1 {
                                checker.otherwise = part.to_string();
                                continue;
                            }

                            let mut rule = Rule {
                                var: '?',
                                op: Operation::LT,
                                val: 0,
                                dst: String::from(""),
                            };
                            for rule_m in rule_re.captures_iter(part) {
                                for (rule_i, rule_capt) in rule_m.iter().enumerate() {
                                    if let Some(rule_sub) = rule_capt {
                                        match rule_i {
                                            1 => {
                                                rule.var = *rule_sub
                                                    .as_str()
                                                    .chars()
                                                    .collect::<Vec<char>>()
                                                    .first()
                                                    .unwrap()
                                            }
                                            2 => {
                                                rule.op = match rule_sub.as_str() {
                                                    ">" => Operation::GT,
                                                    "<" => Operation::LT,
                                                    _ => unreachable!(),
                                                }
                                            }
                                            3 => {
                                                rule.val = rule_sub.as_str().parse::<i64>().unwrap()
                                            }
                                            4 => rule.dst = rule_sub.as_str().to_string(),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            checker.rules.push(rule);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    (name, checker)
}

fn parse_values(line: &str) -> Value {
    let value_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let mut value = Value {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };
    for m in value_re.captures_iter(line) {
        for (i, capt) in m.iter().enumerate() {
            if let Some(sub) = capt {
                match i {
                    1 => value.x = sub.as_str().parse::<i64>().unwrap(),
                    2 => value.m = sub.as_str().parse::<i64>().unwrap(),
                    3 => value.a = sub.as_str().parse::<i64>().unwrap(),
                    4 => value.s = sub.as_str().parse::<i64>().unwrap(),
                    _ => {}
                }
            }
        }
    }

    value
}

fn handle_pt1(lines: &Vec<String>) -> i64 {
    let binding = lines
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_empty())
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let split_idx = binding.first().unwrap();

    let mut checkers = HashMap::new();
    for l in 0..*split_idx {
        let (name, checker) = parse_checker(&lines[l]);
        checkers.insert(name, checker);
    }

    let mut total = 0;
    for l in (*split_idx + 1)..lines.len() {
        let value = parse_values(&lines[l]);
        let mut workflow = "in".to_string();
        loop {
            // println!("{} -- workflow = {}", value, workflow);
            if workflow == "A" || workflow == "R" {
                break;
            }

            workflow = checkers.get(&workflow).unwrap().check(value);
        }

        if workflow == "A" {
            // println!("{} is accepted ({})", value, value.accepted());
            total += value.accepted();
        }
    }

    total
}

fn rec_find_accepted_rule_paths(
    checker: &HashMap<String, Checker>,
    name: &str,
    vrange: ValueRange,
) -> Vec<ValueRange> {
    match name {
        "A" => return vec![vrange],
        "R" => return vec![],
        _ => {}
    }

    let mut res = Vec::new();
    let check = checker.get(name).unwrap();
    let mut vr = vrange;
    for rule in &check.rules {
        let mut pass_vr = vr.clone();
        pass_vr.update(rule, true);
        res.append(&mut rec_find_accepted_rule_paths(
            checker, &rule.dst, pass_vr,
        ));
        vr.update(&rule, false);
    }

    res.append(&mut rec_find_accepted_rule_paths(
        checker,
        &check.otherwise,
        vr,
    ));

    res
}

fn find_accepted_rule_paths(checkers: &HashMap<String, Checker>) -> Vec<ValueRange> {
    rec_find_accepted_rule_paths(checkers, "in", ValueRange::new())
}

fn handle_pt2(lines: &Vec<String>) -> i64 {
    let binding = lines
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_empty())
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let split_idx = binding.first().unwrap();

    let mut checkers = HashMap::new();
    for l in 0..*split_idx {
        let (name, checker) = parse_checker(&lines[l]);
        checkers.insert(name, checker);
    }

    let mut possibles = find_accepted_rule_paths(&checkers);
    possibles.sort();
    println!(
        "possibles
{}",
        possibles
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );

    possibles.iter().map(|x| x.combinations()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let tests = [(
            vec![
                String::from("px{a<2006:qkq,m>2090:A,rfg}"),
                String::from("pv{a>1716:R,A}"),
                String::from("lnx{m>1548:A,A}"),
                String::from("rfg{s<537:gd,x>2440:R,A}"),
                String::from("qs{s>3448:A,lnx}"),
                String::from("qkq{x<1416:A,crn}"),
                String::from("crn{x>2662:A,R}"),
                String::from("in{s<1351:px,qqz}"),
                String::from("qqz{s>2770:qs,m<1801:hdj,R}"),
                String::from("gd{a>3333:R,R}"),
                String::from("hdj{m>838:A,pv}"),
                String::from(""),
                String::from("{x=787,m=2655,a=1222,s=2876}"),
                String::from("{x=1679,m=44,a=2067,s=496}"),
                String::from("{x=2036,m=264,a=79,s=2244}"),
                String::from("{x=2461,m=1339,a=466,s=291}"),
                String::from("{x=2127,m=1623,a=2188,s=1013}"),
            ],
            (19114, 167409079868000),
        )];

        for (input, (want1, want2)) in tests {
            assert_eq!(
                handle_pt1(&input),
                want1,
                "with input\n{}",
                input.join("\n")
            );
            assert_eq!(
                handle_pt2(&input),
                want2,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
}
