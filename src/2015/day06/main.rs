use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;

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
    let instructions: Vec<(&str, Point<usize>, Point<usize>)> =
        lines.iter().map(|x| parse_line(x)).collect();
    let pt1 = handle_pt1(&instructions);
    println!("Part 1: {}", pt1);
    let pt2 = handle_pt2(&instructions);
    println!("Part 2: {}", pt2);
}

fn parse_line(s: &str) -> (&str, Point<usize>, Point<usize>) {
    let instr_re =
        Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut action: &str = "";
    let mut start: Point<usize> = Point::<usize> { x: 0, y: 0 };
    let mut end: Point<usize> = Point::<usize> { x: 0, y: 0 };
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

fn handle_pt1(instructions: &Vec<(&str, Point<usize>, Point<usize>)>) -> i32 {
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

fn handle_pt2(instructions: &Vec<(&str, Point<usize>, Point<usize>)>) -> i32 {
    let mut grid: HashMap<Point<usize>, i32> = HashMap::new();

    for (action, start, end) in instructions {
        for x in start.x..=end.x {
            for y in start.y..=end.y {
                let curr = Point::<usize> { x: x, y: y };
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
            if let Some(brightness) = grid.get(&Point::<usize> { x: x, y: y }) {
                total += brightness;
            }
        }
    }

    total
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
            let instructions: Vec<(&str, Point<usize>, Point<usize>)> =
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
            let instructions: Vec<(&str, Point<usize>, Point<usize>)> =
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
