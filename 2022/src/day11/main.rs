use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day11/input.txt";
    println!("In file {}", file_path);

    let lines: Vec<String> = read_lines(file_path)
        .expect("where is the file")
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect();

    let monkeys: Vec<RefCell<Monkey>> = split_lines(lines)
        .iter()
        .map(|section| Monkey::from_section(section))
        .collect();
    // Bless modulo arithmetic:
    // https://www.reddit.com/r/adventofcode/comments/zizi43/comment/iztt8mx/
    let divisor = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.borrow().condition.divisor);
    println!("Divisor is {}", divisor);
    for (i, m) in monkeys.iter().enumerate() {
        println!("M{} = {}", i, m.borrow().condition.divisor);
    }

    for _round in 0..10000 {
        for (_i, monkey) in monkeys.iter().enumerate() {
            let mut monkey = monkey.borrow_mut();
            loop {
                match monkey.perform_action(divisor) {
                    Some((item, dst)) => monkeys[dst as usize].borrow_mut().receive_item(item),
                    _ => break,
                }
            }
        }
    }

    let mut inspections: Vec<i64> = Vec::new();
    for monkey in monkeys.iter() {
        inspections.push(monkey.borrow().inspection_count);
    }
    inspections.sort();
    inspections.reverse();

    println!("{}", inspections[0] * inspections[1]);
}

#[derive(Debug)]
pub struct Calc {
    tok1: Tok,
    tok2: Tok,
    op: char,
}

#[derive(Debug)]
pub struct Tok {
    var: Option<bool>,
    val: Option<i64>,
}

#[derive(Debug)]
pub struct ThrowCondition {
    divisor: i64,
    dst_true: i64,
    dst_false: i64,
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    condition: ThrowCondition,
    calc: Calc,
    inspection_count: i64,
}

impl Tok {
    pub fn from_str(s: &str) -> Self {
        match s.parse::<i64>() {
            Ok(n) => {
                return Self {
                    var: None,
                    val: Some(n),
                }
            }
            _ => {
                return Self {
                    var: Some(true),
                    val: None,
                }
            }
        }
    }

    pub fn get_value(&self, old: i64) -> i64 {
        if let Some(val) = self.val {
            return val;
        }

        self.var.expect("no val, must have var");
        return old;
    }
}

impl Calc {
    pub fn with_equation(tok1: Tok, op: char, tok2: Tok) -> Self {
        Self {
            tok1: tok1,
            tok2: tok2,
            op: op,
        }
    }

    pub fn calculate(&self, old: i64) -> i64 {
        let tok1 = self.tok1.get_value(old);
        let tok2 = self.tok2.get_value(old);
        match self.op {
            '*' => return tok1 * tok2,
            '-' => return tok1 - tok2,
            '+' => return tok1 + tok2,
            _ => (),
        }

        assert!(false);
        return -1;
    }
}

impl ThrowCondition {
    pub fn from_section(section: &Vec<String>) -> Self {
        return Self {
            divisor: section[3]
                .trim_start()
                .strip_prefix("Test: divisible by ")
                .expect("divisor mismatch")
                .parse::<i64>()
                .unwrap(),
            dst_true: section[4]
                .trim_start()
                .strip_prefix("If true: throw to monkey ")
                .expect("throw true mismatch")
                .parse::<i64>()
                .unwrap(),
            dst_false: section[5]
                .trim_start()
                .strip_prefix("If false: throw to monkey ")
                .expect("throw true mismatch")
                .parse::<i64>()
                .unwrap(),
        };
    }

    pub fn get_destination(&self, val: i64) -> i64 {
        if val % self.divisor == 0 {
            return self.dst_true;
        }
        return self.dst_false;
    }
}

impl Monkey {
    pub fn from_section(section: &Vec<String>) -> RefCell<Self> {
        return RefCell::new(Self {
            items: Monkey::parse_starting_items(section[1].as_str()),
            condition: ThrowCondition::from_section(section),
            calc: Monkey::parse_equation(section[2].as_str()),
            inspection_count: 0,
        });
    }

    fn parse_starting_items(line: &str) -> Vec<i64> {
        let items_str = line
            .trim_start()
            .strip_prefix("Starting items: ")
            .expect("starting items mismatch");
        let mut res: Vec<i64> = Vec::new();
        let parts: Vec<&str> = items_str.split(", ").collect();
        for part in parts.iter() {
            res.push(part.parse::<i64>().unwrap());
        }
        return res;
    }

    fn parse_equation(line: &str) -> Calc {
        let equation = line
            .trim_start()
            .strip_prefix("Operation: new = ")
            .expect("mismatch");
        let eq_parts: Vec<&str> = equation.split(" ").collect();
        assert!(eq_parts.len() == 3);
        let op_chars: Vec<char> = eq_parts[1].chars().collect();
        assert!(op_chars.len() == 1);
        return Calc::with_equation(
            Tok::from_str(eq_parts[0]),
            op_chars[0],
            Tok::from_str(eq_parts[2]),
        );
    }

    fn perform_action(&mut self, divisor: i64) -> Option<(i64, i64)> {
        if self.items.len() == 0 {
            return None;
        }
        let item = self.items.pop().expect("just checked array length");
        let new_val = self.calc.calculate(item) % divisor;
        self.inspection_count += 1;
        return Some((new_val, self.condition.get_destination(new_val)));
    }

    fn receive_item(&mut self, item: i64) {
        self.items.push(item);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn split_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut split_lines: Vec<Vec<String>> = Vec::new();
    split_lines.push(Vec::new());
    let mut section_idx = 0;
    for line in lines.iter() {
        if line.len() == 0 {
            section_idx += 1;
            split_lines.push(Vec::new());
            continue;
        }

        split_lines[section_idx].push(line.clone());
    }
    return split_lines;
}
