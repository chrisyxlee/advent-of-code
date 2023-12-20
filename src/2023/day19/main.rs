use std::collections::HashMap;

use advent_of_code::utils::input::read_lines;
use clap::Parser;
use regex::Regex;
use std::fmt;
use std::time::Instant;

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

    let mut start = Instant::now();
    let (checkers, values) = parse_lines(&lines);
    println!("Part 1: {}", handle_pt1(&checkers, &values));
    println!("Elapsed: {:.2?}", start.elapsed());

    start = Instant::now();
    println!("Part 2: {}", handle_pt2(&checkers));
    println!("Elapsed: {:.2?}", start.elapsed());
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
            if let Some(dst) = rule.run(value) {
                return dst;
            }
        }

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
            (self.x_max - self.x_min + 1)
                * (self.m_max - self.m_min + 1)
                * (self.a_max - self.a_min + 1)
                * (self.s_max - self.s_min + 1),
            0,
        ]
        .iter()
        .max()
        .unwrap()
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

fn parse_rule(line: &Vec<char>) -> Rule {
    let mut stage = 0;
    let mut val = Vec::new();
    let mut dst = Vec::new();
    let mut rule = Rule {
        var: '?',
        op: Operation::LT,
        val: 0,
        dst: String::from(""),
    };
    for c in line {
        match stage {
            0 => {
                rule.var = *c;
                stage += 1;
            }
            1 => {
                rule.op = match c {
                    '<' => Operation::LT,
                    '>' => Operation::GT,
                    _ => unreachable!(),
                };
                stage += 1;
            }
            2 => {
                if c.is_numeric() {
                    val.push(*c);
                } else {
                    rule.val = val.iter().collect::<String>().parse::<i64>().unwrap();
                    stage += 1;
                }
            }
            3 => {
                dst.push(*c);
            }
            _ => unreachable!(),
        }
    }

    rule.dst = dst.iter().collect::<String>();

    rule
}

fn parse_checker(line: &str) -> (String, Checker) {
    let mut name = Vec::new();
    let mut checker = Checker {
        rules: Vec::new(),
        otherwise: String::from(""),
    };

    let mut rule_start = false;
    let mut rule = Vec::new();
    for c in line.chars() {
        match c {
            '}' => checker.otherwise = rule.iter().collect::<String>(),
            '{' => rule_start = true,
            ',' => {
                checker.rules.push(parse_rule(&rule));
                rule.clear();
            }
            _ => {
                if rule_start {
                    rule.push(c);
                } else {
                    name.push(c);
                }
            }
        }
    }

    (name.iter().collect::<String>(), checker)
}

fn parse_lines(lines: &Vec<String>) -> (HashMap<String, Checker>, Vec<Value>) {
    let start = Instant::now();
    let mut checkers = HashMap::new();
    let mut i = 0;
    while !lines[i].is_empty() {
        let (name, checker) = parse_checker(&lines[i]);
        checkers.insert(name, checker);
        i += 1;
    }

    let mut values = Vec::new();
    while i < lines.len() {
        values.push(parse_value(&lines[i]));
        i += 1;
    }

    println!("Parse Lines: elapsed: {:.2?}", start.elapsed());
    (checkers, values)
}

fn parse_value(line: &str) -> Value {
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

fn handle_pt1(checkers: &HashMap<String, Checker>, values: &Vec<Value>) -> i64 {
    let mut total = 0;
    for value in values {
        let mut workflow = "in".to_string();
        loop {
            if workflow == "A" || workflow == "R" {
                break;
            }

            workflow = checkers.get(&workflow).unwrap().check(*value);
        }

        if workflow == "A" {
            total += value.accepted();
        }
    }

    total
}

fn find_accepted_rule_paths(
    checker: &HashMap<String, Checker>,
    name: &str,
    value_range: ValueRange,
) -> Vec<ValueRange> {
    match name {
        "A" => return vec![value_range],
        "R" => return vec![],
        _ => {}
    }

    let mut res = Vec::new();
    let check = checker.get(name).unwrap();
    let mut curr = value_range;
    for rule in &check.rules {
        let mut pass = curr.clone();
        pass.update(rule, true);
        res.append(&mut find_accepted_rule_paths(checker, &rule.dst, pass));

        curr.update(&rule, false);
    }

    res.append(&mut find_accepted_rule_paths(
        checker,
        &check.otherwise,
        curr,
    ));

    res
}

fn handle_pt2(checkers: &HashMap<String, Checker>) -> i64 {
    find_accepted_rule_paths(checkers, "in", ValueRange::new())
        .into_iter()
        .map(|x| x.combinations())
        .sum()
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
            let (checker, values) = parse_lines(&input);
            assert_eq!(
                handle_pt1(&checker, &values),
                want1,
                "with input\n{}",
                input.join("\n")
            );
            assert_eq!(
                handle_pt2(&checker),
                want2,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
}
