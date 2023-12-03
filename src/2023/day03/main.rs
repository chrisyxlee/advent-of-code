use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
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
    let pt2: i32 = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
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

fn handle_pt2(lines: &Vec<String>) -> i32 {
    let mut maybe_gears: Vec<Point<i32>> = Vec::new();
    let mut parts: HashMap<Point<i32>, i32> = HashMap::new();

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
                for location in current_part.locations.borrow().iter() {
                    parts.insert(*location, current_part.val);
                }
                current_part = MaybePart::new();
            }

            if c == '*' {
                maybe_gears.push(location);
            }
        }

        if !current_part.empty() {
            for location in current_part.locations.borrow().iter() {
                parts.insert(*location, current_part.val);
            }
        }
    }

    //  maybe_parts.iter().map(|maybe_part| maybe_part.locations.iter().
    let mut total = 0;
    for gear in maybe_gears {
        let mut adjacent_parts: Vec<i32> = Vec::new();
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
            let location = Point::<i32> {
                x: gear.x + x,
                y: gear.y + y,
            };
            if let Some(val) = parts.get(&location) {
                adjacent_parts.push(*val);
            }
        }

        adjacent_parts.sort();
        adjacent_parts.dedup();
        if adjacent_parts.len() == 2 {
            total += adjacent_parts[0] * adjacent_parts[1];
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

    #[test]
    fn test_parsing_pt2() {
        let tests = [
            (
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
                467_835,
            ),
            (
                vec![
                    String::from("..589"),
                    String::from("..*.."),
                    String::from("699.."),
                ],
                411_711,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
