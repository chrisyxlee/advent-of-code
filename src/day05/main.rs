use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Ship {
    stacks: Vec<Stack<char>>,
}

impl Ship {
    pub fn with_buckets(total: i32) -> Self {
        let mut vec = Vec::with_capacity(total as usize);
        for _i in 0..total {
            vec.push(Stack::with_capacity(50));
        }
        Self { stacks: vec }
    }

    pub fn move_container(&mut self, num_times: i32, src: i32, dst: i32) {
        let mut to_push: Vec<char> = Vec::with_capacity(num_times as usize);
        for _i in 0..num_times {
            if let Some(item) = self.stacks[src as usize].pop() {
                to_push.push(item);
            }
        }
        to_push.reverse();
        for c in to_push {
            self.stacks[dst as usize].push(c);
        }
    }

    pub fn push_container(&mut self, dst: i32, v: char) {
        self.stacks[dst as usize].push(v);
    }

    pub fn finalize_initial_state(&mut self) {
        for i in 0..self.stacks.len() {
            self.stacks[i].reverse();
        }
    }

    pub fn peek_top(self) -> String {
        let mut top: String = "".to_owned();
        for stk in self.stacks {
            let item = stk.peek();
            if let Some(c) = item {
                top.push(*c);
            }
        }
        return top.to_string();
    }

    pub fn print(&mut self) {
        for i in 0..self.stacks.len() {
            println!("{}: {:?}", i, self.stacks[i]);
        }
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(maxsize: usize) -> Self {
        Self {
            maxsize,
            items: Vec::with_capacity(maxsize),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() == self.maxsize {
            return false;
        }
        self.items.push(item);
        return true;
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn reverse(&mut self) {
        self.items.reverse()
    }
}

fn main() {
    // --snip--
    let file_path = "tmp/day05/input.txt";
    println!("In file {}", file_path);

    let mut past_header = false;
    let container_re = Regex::new(r"(\[\w\]\s*)+").unwrap();
    let mut ship = Ship::with_buckets(9);

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if !past_header && container_re.is_match(ip.as_str()) {
                    for (i, c) in ip.chars().enumerate() {
                        match c {
                            '[' | ']' | ' ' => (),
                            _ => {
                                let idx = (i - 1) / 4;
                                ship.push_container(idx as i32, c);
                            }
                        }
                    }
                } else if !past_header {
                    past_header = true;
                    ship.finalize_initial_state();
                } else {
                    let (num_to_move, src, dst) = parse_move(ip.as_str());
                    ship.move_container(num_to_move, src - 1, dst - 1);
                }
            }
        }
    }
    println!("Total overlapping: {}", ship.peek_top());
}

fn parse_move(line: &str) -> (i32, i32, i32) {
    let mut num_to_move = 0;
    let mut src = 0;
    let mut dst = 0;
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)$").unwrap();
    for m in re.captures_iter(line) {
        for (i, capt) in m.iter().enumerate() {
            if capt.is_some() {
                if let Some(thing) = capt {
                    match i {
                        1 => num_to_move = thing.as_str().parse::<i32>().unwrap(),
                        2 => src = thing.as_str().parse::<i32>().unwrap(),
                        3 => dst = thing.as_str().parse::<i32>().unwrap(),
                        _ => (),
                    }
                }
            }
        }
    }
    return (num_to_move, src, dst);
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
