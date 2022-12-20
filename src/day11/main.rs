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

    let mut monkeys: Vec<RefCell<Monkey>> = Vec::new();
    let sections = split_lines(lines);
    for section in sections.iter() {
        monkeys.push(Monkey::from_section(section));
    }

    let num_rounds_to_print = 1;

    for round in 0..10000 {
        if round % num_rounds_to_print == 0 {
            println!("Round {}:", round);
        }
        for (i, monkey) in monkeys.iter().enumerate() {
            let mut monkey = monkey.borrow_mut();
            loop {
                match monkey.perform_action() {
                    Some((item, dst)) => monkeys[dst as usize].borrow_mut().receive_item(item),
                    _ => break,
                }
            }
            if round % num_rounds_to_print == 0 {
                println!("monkey[{}].inspection = {}", i, monkey.inspection_count);
            }
        }
    }

    let mut inspections: Vec<i32> = Vec::new();
    for monkey in monkeys.iter() {
        inspections.push(monkey.borrow().inspection_count);
    }
    inspections.sort();
    inspections.reverse();

    println!("{}", inspections[0] * inspections[1]);
}

fn c2v(c: char) -> i32 {
    return (c as u8 - '0' as u8) as i32;
}

fn v2c(v: i32) -> char {
    return (v as u8 + '0' as u8) as char;
}

fn i2s(v: i32) -> String {
    if v == 0 {
        return String::from("0");
    }
    let mut result = String::from("");
    let mut curr = v;
    while curr > 0 {
        result.push(v2c(curr % 10));
        curr = curr / 10;
    }
    return result.chars().rev().collect::<String>();
}

fn mult_str(a: &str, b: &str) -> String {
    let bchars: Vec<char> = b.chars().rev().collect();
    let mut to_add: Vec<String> = Vec::new();
    for (ai, ac) in a.chars().rev().enumerate() {
        let mut carry: i32 = 0;
        let mut partials = String::from("");
        for _i in 0..ai {
            partials.push('0');
        }
        let av = c2v(ac);
        for bc in bchars.iter() {
            let bv = c2v(*bc);
            let tmp = (av * bv) + carry;
            partials.push(v2c(tmp % 10));
            carry = tmp / 10;
        }
        if carry > 0 {
            partials.push(v2c(carry));
        }
        to_add.push(partials.chars().rev().collect::<String>());
    }

    let mut result = String::from("0");
    for v in to_add {
        result = add_str(v.as_str(), result.as_str());
    }
    return result;
}

fn add_str(a: &str, b: &str) -> String {
    // ignore minus signs for now since that's a difference operation.
    let achars: Vec<char> = a.chars().collect();
    let bchars: Vec<char> = b.chars().collect();
    let mut ai: i32 = (achars.len() - 1) as i32;
    let mut bi: i32 = (bchars.len() - 1) as i32;
    let mut carry: i32 = 0;
    let mut result = String::from("");
    loop {
        if ai < 0 && bi < 0 {
            break;
        }

        let mut tmp = carry;
        if ai >= 0 {
            tmp += c2v(achars[ai as usize]);
        }
        if bi >= 0 {
            tmp += c2v(bchars[bi as usize]);
        }
        carry = tmp / 10;
        result.push(v2c(tmp % 10));
        ai -= 1;
        bi -= 1;
    }

    return result.chars().rev().collect::<String>();
}

fn sub_str(a: &str, b: &str) -> String {
    // ignore minus signs for now since that's a difference operation.
    let achars: Vec<char> = a.chars().collect();
    let bchars: Vec<char> = b.chars().collect();
    let mut ai: i32 = achars.len() as i32 - 1;
    let mut bi: i32 = bchars.len() as i32 - 1;
    let mut borrow: i32 = 0;
    let mut result = String::from("");
    loop {
        if ai < 0 && bi < 0 {
            break;
        }

        let mut tmp = 0;
        if ai >= 0 {
            tmp = c2v(achars[ai as usize]);
        }
        if bi >= 0 {
            tmp -= c2v(bchars[bi as usize]);
        }
        if borrow > 0 {
            tmp -= 1;
            borrow = 0;
        }
        if tmp < 0 {
            borrow = 1;
            tmp += 10;
        }
        result.push(v2c(tmp % 10));
        ai -= 1;
        bi -= 1;
    }

    return result
        .chars()
        .rev()
        .collect::<String>()
        .trim_start_matches('0')
        .to_owned();
}

fn divisible(a: &str, m: i32) -> bool {
    let mut res = 0;
    for c in a.chars() {
        res = ((res * 10) + (c as i32 - '0' as i32)) % m;
    }
    return res == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert!(add_str("5021", "13619") == "18640");
    }

    #[test]
    fn sub_works() {
        let tests = [
            ("23452", "2621", "20831"),
            ("1", "0", "1"),
            (
                "304690239803953095808098098",
                "12035123123125322132312312",
                "292655116680827773675785786",
            ),
        ];
        for (a, b, want) in tests {
            let got = sub_str(a, b);
            assert!(want == got, "{} * {} = {}, but got {}", a, b, want, got);
        }
    }

    #[test]
    fn mult_works() {
        let tests = [
            ("105", "6219", "652995"),
            (
                "1235421415454545454545454544",
                "1714546546546545454544548544544545",
                "2118187521397235888154583183918321221520083884298838480662480",
            ),
        ];
        for (a, b, want) in tests {
            let got = mult_str(a, b);
            assert_eq!(got, want, "{} * {} = {}, but got {}", a, b, want, got);
        }
    }

    #[test]
    fn mod_works() {
        let vec = [
            ("1023691090", 17, false),
            ("1023691091", 17, true),
            ("10", 5, true),
            ("690239803953095808098098", 3, false),
            ("690239803953095808098098", 2, true),
            ("690239803953095808098098", 5, false),
        ];
        for (num, m, want) in vec {
            assert!(
                divisible(num, m) == want,
                "{} % {} is divisible? want {}",
                num,
                m,
                want,
            );
        }
    }
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
    val: Option<i32>,
}

#[derive(Debug)]
pub struct ThrowCondition {
    divisor: i32,
    dst_true: i32,
    dst_false: i32,
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<String>,
    divisor: ThrowCondition,
    calc: Calc,
    inspection_count: i32,
}

impl Tok {
    pub fn from_str(s: &str) -> Self {
        match s.parse::<i32>() {
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

    pub fn get_value(&self, old: &str) -> String {
        if let Some(val) = self.val {
            return i2s(val);
        }

        self.var.expect("no val, must have var");
        return old.to_owned();
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

    pub fn calculate(&self, old: String) -> String {
        let tok1 = self.tok1.get_value(&old);
        let tok2 = self.tok2.get_value(&old);
        match self.op {
            '*' => return mult_str(tok1.as_str(), tok2.as_str()),
            '-' => return sub_str(tok1.as_str(), tok2.as_str()),
            '+' => return add_str(tok1.as_str(), tok2.as_str()),
            _ => (),
        }

        assert!(false);
        return String::from("");
    }
}

impl ThrowCondition {
    pub fn from_section(section: &Vec<String>) -> Self {
        return Self {
            divisor: section[3]
                .trim_start()
                .strip_prefix("Test: divisible by ")
                .expect("divisor mismatch")
                .parse::<i32>()
                .unwrap(),
            dst_true: section[4]
                .trim_start()
                .strip_prefix("If true: throw to monkey ")
                .expect("throw true mismatch")
                .parse::<i32>()
                .unwrap(),
            dst_false: section[5]
                .trim_start()
                .strip_prefix("If false: throw to monkey ")
                .expect("throw true mismatch")
                .parse::<i32>()
                .unwrap(),
        };
    }

    pub fn get_destination(&self, val: &str) -> i32 {
        if divisible(val, self.divisor) {
            return self.dst_true;
        }
        return self.dst_false;
    }
}

impl Monkey {
    pub fn from_section(section: &Vec<String>) -> RefCell<Self> {
        return RefCell::new(Self {
            items: Monkey::parse_starting_items(section[1].as_str()),
            divisor: ThrowCondition::from_section(section),
            calc: Monkey::parse_equation(section[2].as_str()),
            inspection_count: 0,
        });
    }

    fn parse_starting_items(line: &str) -> Vec<String> {
        let items_str = line
            .trim_start()
            .strip_prefix("Starting items: ")
            .expect("starting items mismatch");
        let mut res: Vec<String> = Vec::new();
        let parts: Vec<&str> = items_str.split(", ").collect();
        for part in parts {
            res.push(part.to_owned());
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

    fn perform_action(&mut self) -> Option<(String, i32)> {
        if self.items.len() == 0 {
            return None;
        }
        let item = self.items.pop().expect("just checked array length");
        let new_val = self.calc.calculate(item);
        self.inspection_count += 1;
        let dst = self.divisor.get_destination(&new_val);
        return Some((new_val, dst));
    }

    fn receive_item(&mut self, item: String) {
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
