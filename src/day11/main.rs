use std::cell::RefCell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

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

    let num_rounds_to_print = 1;

    for round in 0..10000 {
        if round % num_rounds_to_print == 0 {
            println!("Round {}:", round);
        }
        for (_i, monkey) in monkeys.iter().enumerate() {
            let mut monkey = monkey.borrow_mut();
            loop {
                match monkey.perform_action() {
                    Some((item, dst)) => monkeys[dst as usize].borrow_mut().receive_item(item),
                    _ => break,
                }
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

fn i2s(v: i32) -> Vec<char> {
    if v == 0 {
        return Vec::new();
    }
    let mut result = String::from("");
    let mut curr = v;
    while curr > 0 {
        result.push(v2c(curr % 10));
        curr = curr / 10;
    }
    return result.chars().collect();
}

fn mult_str(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let now = Instant::now();
    let mut result: Vec<char> = Vec::new();
    for ai in 0..a.len() {
        let mut carry: i32 = 0;
        let mut partials: Vec<char> = Vec::new();
        for _i in 0..ai {
            partials.push('0');
        }
        let av = c2v(a[ai]);
        for bi in 0..b.len() {
            let bv = c2v(b[bi]);
            let tmp = (av * bv) + carry;
            partials.push(v2c(tmp % 10));
            carry = tmp / 10;
        }
        if carry > 0 {
            partials.push(v2c(carry));
        }
        result = add_str(result, partials);
    }

    let elapsed = now.elapsed();
    println!("MULT Elapsed: {:.2?}", elapsed);
    return result;
}

fn add_str(left: Vec<char>, right: Vec<char>) -> Vec<char> {
    let mut a: Vec<char>;
    let b: Vec<char>;
    if left.len() > right.len() {
        a = left;
        b = right;
    } else {
        a = right;
        b = left;
    }
    if a.len() == 0 {
        return b;
    }
    if b.len() == 0 {
        return a;
    }
    // ignore minus signs for now since that's a difference operation.
    let mut ai: i32 = 0;
    let mut bi: i32 = 0;
    let mut carry: i32 = 0;
    while ai < a.len() as i32 && bi < b.len() as i32 {
        let mut tmp = carry;
        tmp += c2v(a[ai as usize]);
        if bi < b.len() as i32 {
            tmp += c2v(b[bi as usize]);
        }
        carry = tmp / 10;
        a[ai as usize] = v2c(tmp % 10);
        ai += 1;
        bi += 1;
    }
    while carry > 0 {
        if ai < a.len() as i32 {
            let tmp = carry + c2v(a[ai as usize]);
            carry = tmp / 10;
            a[ai as usize] = v2c(tmp % 10);
            ai += 1;
        } else {
            a.push(v2c(carry));
            carry = 0;
        }
    }

    return a;
}

fn vc2s(v: &Vec<char>) -> String {
    if v.len() == 0 {
        return String::from("0");
    }
    return v.clone().iter().rev().collect::<String>();
}

fn s2vc(s: &str) -> Vec<char> {
    return s.chars().rev().collect();
}

fn sub_str(mut a: Vec<char>, b: &Vec<char>) -> Vec<char> {
    if b.len() == 0 {
        return a;
    }
    // ignore minus signs for now since that's a difference operation.
    let mut ai: i32 = 0;
    let mut bi: i32 = 0;
    let mut borrow: i32 = 0;
    while ai < a.len() as i32 || bi < b.len() as i32 {
        let mut tmp = 0;
        if ai < a.len() as i32 {
            tmp = c2v(a[ai as usize]);
        }
        if bi < b.len() as i32 {
            tmp -= c2v(b[bi as usize]);
        }
        if borrow > 0 {
            tmp -= 1;
            borrow = 0;
        }
        if tmp < 0 {
            borrow = 1;
            tmp += 10;
        }
        if ai < a.len() as i32 {
            a[ai as usize] = v2c(tmp % 10);
        }
        ai += 1;
        bi += 1;
    }

    if let Some(last) = a.iter().rposition(|&c| c != '0') {
        return a[0..last + 1].to_vec();
    }

    return a;
}

fn divisible(a: &Vec<char>, m: i32) -> bool {
    let mut res = 0;
    for ai in (0..a.len()).rev() {
        res = ((res * 10) + (a[ai] as i32 - '0' as i32)) % m;
    }
    return res == 0;
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
    items: Vec<Vec<char>>,
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

    pub fn get_value(&self, old: Vec<char>, is_first: bool) -> (Vec<char>, Vec<char>) {
        if let Some(val) = self.val {
            return (i2s(val), old);
        }

        self.var.expect("no val, must have var");
        if is_first {
            return (old.clone(), old);
        } else {
            return (old, Vec::new());
        }
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

    pub fn calculate(&self, old: Vec<char>) -> Vec<char> {
        let (tok1, old) = self.tok1.get_value(old, true);
        let (tok2, _) = self.tok2.get_value(old, false);
        match self.op {
            '*' => return mult_str(&tok1, &tok2),
            '-' => return sub_str(tok1, &tok2),
            '+' => return add_str(tok1, tok2),
            _ => (),
        }

        assert!(false);
        return Vec::new();
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

    pub fn get_destination(&self, val: &Vec<char>) -> i32 {
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

    fn parse_starting_items(line: &str) -> Vec<Vec<char>> {
        let items_str = line
            .trim_start()
            .strip_prefix("Starting items: ")
            .expect("starting items mismatch");
        let mut res: Vec<Vec<char>> = Vec::new();
        let parts: Vec<&str> = items_str.split(", ").collect();
        for part in parts {
            let partial: Vec<char> = part.chars().rev().collect();
            res.push(partial);
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

    fn perform_action(&mut self) -> Option<(Vec<char>, i32)> {
        if self.items.len() == 0 {
            return None;
        }
        let item = self.items.pop().expect("just checked array length");
        let new_val = self.calc.calculate(item);
        self.inspection_count += 1;
        let dst = self.divisor.get_destination(&new_val);
        return Some((new_val, dst));
    }

    fn receive_item(&mut self, item: Vec<char>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let tests = [("5021", "13619", "18640"), ("9999999", "1", "10000000")];

        for (a, b, want) in tests {
            let avc = s2vc(a);
            let bvc = s2vc(b);
            let got = vc2s(&add_str(avc, bvc));
            assert!(want == got, "{} + {} = {}, but got {}", a, b, want, got);
        }
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
            ("100000", "1", "99999"),
        ];
        for (a, b, want) in tests {
            let avc = s2vc(a);
            let bvc = s2vc(b);
            let got = vc2s(&sub_str(avc, &bvc));
            assert!(want == got, "{} - {} = {}, but got {}", a, b, want, got);
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
            let avc = s2vc(&a);
            let bvc = s2vc(&b);
            let got = vc2s(&mult_str(&avc, &bvc));
            assert_eq!(got, want, "{} * {} = {}, but got {}", a, b, want, got);
        }
    }

    #[test]
    fn mod_works() {
        let vec = [
            ("1023691090", 17, false),
            ("1023691091", 17, true),
            ("153", 17, true),
            ("153", 9, true),
            ("153", 5, false),
            ("10", 5, true),
            ("690239803953095808098098", 3, false),
            ("690239803953095808098098", 2, true),
            ("690239803953095808098098", 5, false),
        ];
        for (num, m, want) in vec {
            let numvc = s2vc(&num);
            assert!(
                divisible(&numvc, m) == want,
                "{} % {} is divisible? want {}",
                num,
                m,
                want,
            );
        }
    }
}
