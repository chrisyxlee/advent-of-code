use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day03/input.txt";
    let lines = &read_lines(file_path);
    let pt1: usize = lines.iter().map(|x| handle_pt1(x)).sum();
    println!("Part 1: {}", pt1);
    let pt2: usize = lines.iter().map(|x| handle_pt2(x)).sum();
    println!("Part 2: {}", pt2);
}

fn handle_pt1(s: &str) -> usize {
    let mut set: HashSet<Point> = HashSet::new();

    let mut curr = Point::new(0, 0);
    set.insert(curr);
    for c in s.chars().into_iter() {
        curr = curr.next(c);
        set.insert(curr);
    }

    return set.len();
}

fn handle_pt2(s: &str) -> usize {
    let mut set: HashSet<Point> = HashSet::new();
    let mut santa = Point::new(0, 0);
    let mut robo = Point::new(0, 0);
    set.insert(santa);
    for (i, c) in s.chars().enumerate().into_iter() {
        if i % 2 == 0 {
            santa = santa.next(c);
            set.insert(santa);
        } else {
            robo = robo.next(c);
            set.insert(robo);
        }
    }

    return set.len();
}

// To make it easier not to mess up x and y.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn next(&self, c: char) -> Point {
        return match c {
            '^' => Point::new(self.x, self.y + 1),
            '>' => Point::new(self.x + 1, self.y),
            'v' => Point::new(self.x, self.y - 1),
            '<' => Point::new(self.x - 1, self.y),
            _ => return *self,
        };
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (input, want) in tests {
            assert_eq!(handle_pt1(input), want, "for input {}", input);
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (input, want) in tests {
            assert_eq!(handle_pt2(input), want, "for input {}", input);
        }
    }
}
