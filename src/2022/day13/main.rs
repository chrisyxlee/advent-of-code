use advent_of_code::utils::input::read_lines;
use clap::Parser;
use std::cmp::Ordering;
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

    let input = parse_input(lines);
    let mapped: Vec<usize> = input
        .iter()
        .enumerate()
        .filter(|(_i, (left, right))| {
            Signal::parse(&left).compare(&Signal::parse(&right)) == Ordering::Less
        })
        .map(|(i, _)| i + 1)
        .collect();

    let pt1 = mapped.iter().sum::<usize>();
    println!("Part 1: {}", pt1);

    let mut p2_input: Vec<Signal> = Vec::new();
    for (left, right) in &input {
        p2_input.push(Signal::parse(left));
        p2_input.push(Signal::parse(right));
    }
    p2_input.push(Signal::signals(Vec::from([Signal::val(2)])));
    p2_input.push(Signal::signals(Vec::from([Signal::val(6)])));
    p2_input.sort_by(|x, y| x.compare(&y));

    let pt2 = (1 + p2_input
        .iter()
        .position(|s| s.compare(&Signal::signals(Vec::from([Signal::val(2)]))) == Ordering::Equal)
        .unwrap())
        * (1 + p2_input
            .iter()
            .position(|s| {
                s.compare(&Signal::signals(Vec::from([Signal::val(6)]))) == Ordering::Equal
            })
            .unwrap());
    println!("Part 2: {}", pt2);
}

#[derive(PartialEq, Debug)]
pub struct Signal {
    signals: Option<Vec<Signal>>,
    val: Option<i32>,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.signals, self.val) {
            (Some(sigs), None) => write!(
                f,
                "[{}]",
                sigs.iter()
                    .map(|s| format!("{}", s))
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            (None, Some(v)) => write!(f, "{}", v),
            _ => write!(f, "help"),
        }
    }
}

impl Clone for Signal {
    fn clone(&self) -> Signal {
        match (&self.signals, self.val) {
            (Some(sigs), None) => Signal {
                signals: Some(sigs.clone()),
                val: None,
            },
            (None, Some(v)) => Signal {
                signals: None,
                val: Some(v),
            },
            _ => Signal {
                signals: None,
                val: None,
            },
        }
    }
}

impl Signal {
    pub fn parse(v: &Vec<char>) -> Self {
        // Values can have more than 1 digit...
        if v.iter().position(|&c| c == '[') == None {
            return Signal::val(v.iter().collect::<String>().parse::<i32>().unwrap());
        }
        let mut num_brackets = 0;
        let mut bracket_start: usize = 0;
        let mut bracket_end: usize;
        let mut signals: Vec<Signal> = Vec::new();
        let mut last_non_digit: usize = 0;
        for i in 0..v.len() {
            match v[i] {
                '[' => {
                    num_brackets += 1;
                    match num_brackets {
                        1 => last_non_digit = i,
                        2 => bracket_start = i,
                        _ => (),
                    }
                }
                ']' => {
                    num_brackets -= 1;
                    if num_brackets == 0 && i > 0 && v[i - 1] != ']' && i - last_non_digit - 1 >= 1
                    {
                        signals.push(Signal::parse(&Vec::from(&v[last_non_digit + 1..i])));
                    }
                    if num_brackets == 1 {
                        bracket_end = i;
                        signals.push(Signal::parse(&Vec::from(&v[bracket_start..=bracket_end])));
                    }
                }
                ',' => {
                    if num_brackets == 1 && i > 0 && v[i - 1] != ']' && i - last_non_digit - 1 >= 1
                    {
                        signals.push(Signal::parse(&Vec::from(&v[last_non_digit + 1..i])));
                    }
                    last_non_digit = i;
                }
                _ => (),
            }
        }

        return Self {
            signals: Some(signals),
            val: None,
        };
    }

    pub fn val(v: i32) -> Self {
        Self {
            val: Some(v),
            signals: None,
        }
    }

    pub fn signals(v: Vec<Signal>) -> Self {
        Self {
            val: None,
            signals: Some(v),
        }
    }

    fn compare(&self, b: &Signal) -> Ordering {
        let a = self;
        match (&a.signals, &b.signals, a.val, b.val) {
            // If both values are integers, the lower integer should come first. If
            // the left integer is lower than the right integer, the inputs  are in
            // the right order. If the left integer is higher than the right integer,
            // the inputs are not in the right order. Otherwise, the inputs are the
            // same integer; continue checking the next part of the input.
            (None, None, Some(av), Some(bv)) => return av.cmp(&bv),

            // If exactly one value is an integer, convert the integer to a list
            // which contains that integer as its only value, then retry the
            // comparison. For example, if comparing [0,0,0] and 2, convert the
            // right value to [2] (a list containing 2); the result is then found by
            // instead comparing [0,0,0] and [2].
            (None, Some(bsig), Some(av), None) => {
                return Signal::signals(Vec::from([Signal::val(av)]))
                    .compare(&Signal::signals(bsig.clone()));
            }
            (Some(asig), None, None, Some(bv)) => {
                return Signal::signals(asig.clone())
                    .compare(&Signal::signals(Vec::from([Signal::val(bv)])));
            }

            // If both values are lists, compare the first value of each list,
            // then the second value, and so on. If the left list runs out of items
            // first, the inputs are in the right order. If the right list runs out
            // of items first, the inputs are not in the right order. If the lists
            // are the same length and no comparison makes a decision about the
            // order, continue checking the next part of the input.
            (Some(asig), Some(bsig), None, None) => {
                let mut i: usize = 0;
                while i < asig.len() && i < bsig.len() {
                    let cmp = asig[i].clone().compare(&bsig[i]);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                    i = i + 1;
                }
                return asig.len().cmp(&bsig.len());
            }
            _ => assert!(false, "How did we get here?"),
        }

        return Ordering::Greater;
    }
}

fn parse_input(lines: Vec<String>) -> Vec<(Vec<char>, Vec<char>)> {
    let mut input: Vec<(Vec<char>, Vec<char>)> = Vec::new();
    let mut i = 0;
    while i < lines.len() - 1 {
        if lines[i].len() == 0 {
            i += 1;
            continue;
        }

        if i < lines.len() - 1 && lines[i + 1].len() > 0 {
            let l: Vec<char> = lines[i].chars().collect();
            let r: Vec<char> = lines[i + 1].chars().collect();
            input.push((l, r));
            i += 2;
        }
    }

    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        let tests = [
            (
                "[1,1,3]",
                Signal::signals(Vec::from([Signal::val(1), Signal::val(1), Signal::val(3)])),
            ),
            (
                "[[1],4]",
                Signal::signals(Vec::from([
                    Signal::signals(Vec::from([Signal::val(1)])),
                    Signal::val(4),
                ])),
            ),
            (
                "[[4,4],4,4]",
                Signal::signals(Vec::from([
                    Signal::signals(Vec::from([Signal::val(4), Signal::val(4)])),
                    Signal::val(4),
                    Signal::val(4),
                ])),
            ),
            (
                "[[[]]]",
                Signal::signals(Vec::from([Signal::signals(Vec::from([Signal::signals(
                    Vec::from([]),
                )]))])),
            ),
            ("10", Signal::val(10)),
            (
                "[10,5]",
                Signal::signals(Vec::from([Signal::val(10), Signal::val(5)])),
            ),
        ];
        for (input, want) in tests {
            let input_chars: Vec<char> = input.chars().collect();
            let got = Signal::parse(&input_chars);
            assert!(
                want == got,
                "\ninput '{}'\nwant {:?}\ngot  {:?}\n",
                input,
                want,
                got
            );
        }
    }

    #[test]
    fn ordering_works() {
        let tests = [
            (("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less),
            (("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less),
            (("[9]", "[[8,7,6]]"), Ordering::Greater),
            (("[[4,4],4,4]", "[[4,4],4,4,4]"), Ordering::Less),
            (("[7,7,7,7]", "[7,7,7]"), Ordering::Greater),
            (
                ("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
                Ordering::Greater,
            ),
            (("[]", "[3]"), Ordering::Less),
            (("[[[]]]", "[[]]"), Ordering::Greater),
            (
                (
                    "[[],[],[6,9,7],[1,[[8],[],5,[3,1]],[[2,5],10,[],3]]]",
                    "[[[5,[7,0,7,10,5],5,0],1,6],[0,8],[1,0,[[3],[9],[1,3,4,5,3],10],[],5]]",
                ),
                Ordering::Less,
            ),
            (("[[1,2],4]", "[[1],5,5]"), Ordering::Greater),
        ];

        for ((a, b), want) in tests {
            let ac: Vec<char> = a.chars().collect();
            let bc: Vec<char> = b.chars().collect();
            let left = Signal::parse(&ac);
            let right = Signal::parse(&bc);
            {
                let got = left.compare(&right);
                assert!(
                    want == got,
                    "\nCompare\n- {}\n- {}\nwant {:?}, got  {:?}\n",
                    a,
                    b,
                    want,
                    got
                );
            }
            {
                let got = right.compare(&left);
                let want = match want {
                    Ordering::Less => Ordering::Greater,
                    Ordering::Greater => Ordering::Less,
                    _ => want,
                };
                assert!(
                    want == got,
                    "\nCompare\n- {}\n- {}\nwant {:?}, got  {:?}\n",
                    b,
                    a,
                    want,
                    got
                );
            }
        }
    }
}
