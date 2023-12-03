use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
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
    let lines = &read_lines(args.input);
    let pt1: usize = lines.iter().map(|x| handle_pt1(x)).sum();
    println!("Part 1: {}", pt1);
    let pt2: usize = lines.iter().map(|x| handle_pt2(x)).sum();
    println!("Part 2: {}", pt2);
}

fn handle_pt1(s: &str) -> usize {
    let mut set: HashSet<Point<i32>> = HashSet::new();

    let mut curr: Point<i32> = Point::<i32> { x: 0, y: 0 };
    set.insert(curr);
    for c in s.chars().into_iter() {
        curr = next(curr, c);
        set.insert(curr);
    }

    return set.len();
}

fn handle_pt2(s: &str) -> usize {
    let mut set: HashSet<Point<i32>> = HashSet::new();
    let mut santa: Point<i32> = Point::<i32> { x: 0, y: 0 };
    let mut robo: Point<i32> = Point::<i32> { x: 0, y: 0 };
    set.insert(santa);
    for (i, c) in s.chars().enumerate().into_iter() {
        if i % 2 == 0 {
            santa = next(santa, c);
            set.insert(santa);
        } else {
            robo = next(robo, c);
            set.insert(robo);
        }
    }

    return set.len();
}

fn next(p: Point<i32>, c: char) -> Point<i32> {
    return match c {
        '^' => Point::<i32> { x: p.x, y: p.y + 1 },
        '>' => Point::<i32> { x: p.x + 1, y: p.y },
        'v' => Point::<i32> { x: p.x, y: p.y - 1 },
        '<' => Point::<i32> { x: p.x - 1, y: p.y },
        _ => return p,
    };
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
