use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day06/input.txt";
    let lines = &read_lines(file_path);
    let instructions: Vec<(&str, Point, Point)> = lines.iter().map(|x| parse_line(x)).collect();
    let pt1 = handle_pt1(&instructions);
    println!("Part 1: {}", pt1);
    let pt2 = handle_pt2(&instructions);
    println!("Part 2: {}", pt2);
}

fn parse_line(s: &str) -> (&str, Point, Point) {
    let instr_re =
        Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut action: &str = "";
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    for m in instr_re.captures_iter(s) {
        for (i, capt) in m.iter().enumerate() {
            if let Some(sub) = capt {
                match i {
                    1 => {
                        action = sub.as_str();
                    }
                    2 => {
                        start.x = sub.as_str().parse::<usize>().unwrap();
                    }
                    3 => {
                        start.y = sub.as_str().parse::<usize>().unwrap();
                    }
                    4 => {
                        end.x = sub.as_str().parse::<usize>().unwrap();
                    }
                    5 => {
                        end.y = sub.as_str().parse::<usize>().unwrap();
                    }
                    _ => {}
                }
            }
        }
    }

    return (action, start, end);
}

fn handle_pt1(instructions: &Vec<(&str, Point, Point)>) -> i32 {
    let mut grid = [[false; 1000]; 1000];

    for (action, start, end) in instructions {
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                grid[x][y] = match *action {
                    "turn on" => true,
                    "toggle" => !grid[x][y],
                    "turn off" => false,
                    _ => false,
                };
            }
        }
    }

    grid.iter()
        .map(|rows| rows.iter().map(|x| *x as i32).sum::<i32>())
        .sum::<i32>()
}

fn handle_pt2(instructions: &Vec<(&str, Point, Point)>) -> i32 {
    let mut grid: HashMap<Point, i32> = HashMap::new();

    for (action, start, end) in instructions {
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let curr = Point::new(x, y);
                let mut brightness = 0;
                if let Some(b) = grid.get(&curr) {
                    brightness = *b;
                }

                brightness += match *action {
                    "turn on" => 1,
                    "toggle" => 2,
                    "turn off" => -1,
                    _ => 0,
                };
                if brightness < 0 {
                    brightness = 0;
                }
                *grid.entry(curr).or_insert(brightness) = brightness;
            }
        }
    }

    let mut total = 0;
    for x in 0..=1000 {
        for y in 0..=1000 {
            if let Some(brightness) = grid.get(&Point::new(x, y)) {
                total += brightness;
            }
        }
    }

    total
}

// To make it easier not to mess up x and y.
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
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
        let tests = [
            (["turn on 0,0 through 999,999"], 1_000_000),
            (["toggle 0,0 through 999,0"], 1_000),
            (["turn off 499,499 through 500,500"], 0),
        ];

        for (input, want) in tests {
            let instructions: Vec<(&str, Point, Point)> =
                input.iter().map(|x| parse_line(x)).collect();
            assert_eq!(
                handle_pt1(&instructions),
                want,
                "for input {}",
                input.join(" -> ")
            );
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [
            (["turn on 0,0 through 0,0"], 1),
            (["turn on 0,0 through 999,999"], 1_000_000),
            (["toggle 0,0 through 999,0"], 2_000),
            (["turn off 499,499 through 500,500"], 0),
        ];

        for (input, want) in tests {
            let instructions: Vec<(&str, Point, Point)> =
                input.iter().map(|x| parse_line(x)).collect();
            assert_eq!(
                handle_pt2(&instructions),
                want,
                "for input {}",
                input.join(" -> ")
            );
        }
    }
}
