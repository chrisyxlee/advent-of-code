use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

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
    let (grid, corner, start) = create_grid(&lines);

    let trace = Instant::now();
    println!("Part 1: {}", handle_pt1(&grid, corner, start, 64));
    println!("Elapsed: {:.2?}", trace.elapsed());

    // start = Instant::now();
    // println!("Part 2: {}", handle_pt2(&mut parse_lines(&lines)));
    // println!("Elapsed: {:.2?}", start.elapsed());
}

enum Plot {
    GARDEN = 1,
    ROCK = 2,
}

fn create_grid(lines: &Vec<String>) -> (HashMap<Point<i64>, Plot>, Point<i64>, Point<i64>) {
    let height = lines.len() as i64;
    let width = lines.iter().map(|line| line.len()).max().unwrap() as i64;
    let mut grid: HashMap<Point<i64>, Plot> = HashMap::new();
    let mut start = Point { x: 0, y: 0 };

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let p = Point {
                x: col as i64,
                y: row as i64,
            };
            grid.insert(
                p,
                match c {
                    '#' => Plot::ROCK,
                    _ => Plot::GARDEN,
                },
            );
            if c == 'S' {
                start = p;
            }
        }
    }

    (
        grid,
        Point {
            x: width - 1,
            y: height - 1,
        },
        start,
    )
}

const WEST: char = 'W';
const EAST: char = 'E';
const NORTH: char = 'N';
const SOUTH: char = 'S';

fn go(p: Point<i64>, dir: char, corner: Point<i64>) -> Option<Point<i64>> {
    let new_point = Point {
        x: p.x
            + match dir {
                EAST => 1,
                WEST => -1,
                _ => 0,
            },
        y: p.y
            + match dir {
                NORTH => -1,
                SOUTH => 1,
                _ => 0,
            },
    };

    if new_point.x < 0 || new_point.x > corner.x || new_point.y < 0 || new_point.y > corner.y {
        return None;
    }

    Some(new_point)
}

fn go_void(p: Point<i64>, dir: char) -> Point<i64> {
    Point {
        x: p.x
            + match dir {
                EAST => 1,
                WEST => -1,
                _ => 0,
            },
        y: p.y
            + match dir {
                NORTH => -1,
                SOUTH => 1,
                _ => 0,
            },
    }
}

fn real_coord(p: Point<i64>, corner: Point<i64>) -> Point<i64> {
    Point {
        x: p.x % corner.x,
        y: p.y % corner.y,
    }
}

fn handle_pt1(
    grid: &HashMap<Point<i64>, Plot>,
    corner: Point<i64>,
    start: Point<i64>,
    total_steps: i64,
) -> i64 {
    let mut steps = 0;

    let mut possible = HashSet::new();
    possible.insert(start);

    while steps < total_steps {
        steps += 1;

        let mut next_possible = HashSet::new();
        for p in possible.iter() {
            for dir in vec![NORTH, EAST, WEST, SOUTH] {
                let new_p = go_void(*p, dir);
                if p.x <0 || p.y < 0 {
                  println!("{} -> {}", p.x, p.y);
                }
                if let Some(plot) = grid.get(&real_coord(new_p, corner)) {
                    match plot {
                        Plot::ROCK => continue,
                        Plot::GARDEN => {
                            next_possible.insert(new_p);
                        }
                    }
                }
            }
        }

        possible = next_possible;
    }

    possible.len() as i64
}

fn handle_pt2(
    grid: &HashMap<Point<i64>, Plot>,
    corner: Point<i64>,
    start: Point<i64>,
    total_steps: i64,
) -> i64 {
    let mut steps = 0;

    let mut possible = HashSet::new();
    possible.insert(start);

    while steps < total_steps {
        steps += 1;

        let mut next_possible = HashSet::new();
        for p in possible.iter() {
            for dir in vec![NORTH, EAST, WEST, SOUTH] {
                if let Some(new_p) = go(*p, dir, corner) {
                    if let Some(plot) = grid.get(&new_p) {
                        match plot {
                            Plot::ROCK => continue,
                            Plot::GARDEN => {
                                next_possible.insert(new_p);
                            }
                        }
                    }
                }
            }
        }

        possible = next_possible;
    }

    possible.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_pt1() {
        let tests = [(
            (
                vec![
                    "...........",
                    ".....###.#.",
                    ".###.##..#.",
                    "..#.#...#..",
                    "....#.#....",
                    ".##..S####.",
                    ".##..#...#.",
                    ".......##..",
                    ".##.#.####.",
                    ".##..##.##.",
                    "...........",
                ],
                6,
            ),
            16,
        )];

        for ((input, steps), want) in tests {
            let (grid, corner, start) =
                create_grid(&input.iter().map(|x| x.to_string()).collect::<Vec<String>>());
            assert_eq!(
                handle_pt1(&grid, corner, start, steps),
                want,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
    #[test]
    fn test_handle_pt2() {
        let input = vec![
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let tests = [
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
            (5000, 16733044),
        ];

        for (steps, want) in tests {
            let (grid, corner, start) = create_grid(&input);
            assert_eq!(
                handle_pt2(&grid, corner, start, steps),
                want,
                "for {} steps for input\n{}",
                steps,
                input.join("\n")
            );
        }
    }
}
