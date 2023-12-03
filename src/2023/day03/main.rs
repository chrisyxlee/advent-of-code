use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::cell::RefCell;
use std::collections::HashSet;

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

    let pt1: i32 = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
}

fn handle_pt1(lines: &Vec<String>) -> i32 {
    let mut symbols: HashSet<Point<i32>> = HashSet::new();
    let mut maybe_parts: Vec<MaybePart> = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        let mut current_part = MaybePart::new();

        for (col, c) in line.chars().enumerate() {
            let location = Point::<i32> {
                x: row as i32,
                y: col as i32,
            };
            if c.is_numeric() {
                current_part.add(c, location);
                continue;
            }

            if !current_part.empty() {
                maybe_parts.push(current_part);
                current_part = MaybePart::new();
            }

            if c == '.' {
                continue;
            }

            symbols.insert(location);
        }

        if !current_part.empty() {
            maybe_parts.push(current_part);
        }
    }

    //  maybe_parts.iter().map(|maybe_part| maybe_part.locations.iter().
    let mut total = 0;
    for maybe_part in maybe_parts {
        for location in maybe_part.locations.borrow().iter() {
            let mut added = false;
            for (x, y) in [
                (0, 1),
                (1, 0),
                (-1, 0),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                if symbols.contains(&Point::<i32> {
                    x: location.x + x,
                    y: location.y + y,
                }) {
                    total += maybe_part.val;
                    added = true;
                    break;
                }
            }

            if added {
                break;
            }
        }
    }
    total
}

#[derive(Clone)]
pub struct MaybePart {
    pub locations: RefCell<Vec<Point<i32>>>,
    pub val: i32,
}

impl MaybePart {
    pub fn new() -> Self {
        MaybePart {
            locations: RefCell::new(Vec::new()),
            val: 0,
        }
    }
    pub fn add(&mut self, c: char, location: Point<i32>) {
        self.val = self.val * 10 + (c.to_digit(10).unwrap() as i32);
        self.locations.borrow_mut().push(location);
    }

    pub fn empty(&self) -> bool {
        self.locations.borrow().len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(
            vec![
                String::from("467..114.."),
                String::from("...*......"),
                String::from("..35..633."),
                String::from("......#..."),
                String::from("617*......"),
                String::from(".....+.58."),
                String::from("..592....."),
                String::from("......755."),
                String::from("...$.*...."),
                String::from(".664.598.."),
            ],
            4361,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
