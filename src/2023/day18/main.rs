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
    let lines = read_lines(args.input);
    let digs = parse_input(&lines);

    println!("Part 1: {}", handle_pt1(&digs));
    //  println!("Part 2: {}", handle_pt2(&grid, corner));
}

const UP: char = 'U';
const DOWN: char = 'D';
const LEFT: char = 'L';
const RIGHT: char = 'R';

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Dig {
    steps: i32,
    color: String,
    dir: char,
}

fn parse_line(s: &str) -> Dig {
    let mut dig = Dig {
        steps: 0,
        color: String::from(""),
        dir: '?',
    };
    for (i, part) in s.split(' ').enumerate() {
        match i {
            0 => dig.dir = *part.chars().collect::<Vec<char>>().first().unwrap(),
            1 => dig.steps = part.parse::<i32>().unwrap(),
            2 => {
                dig.color = part
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .to_string()
            }
            _ => {}
        }
    }
    dig
}

fn parse_input(lines: &Vec<String>) -> Vec<Dig> {
    lines.iter().map(|l| parse_line(&l)).collect::<Vec<Dig>>()
}

fn go(p: Point<i32>, dir: char) -> Point<i32> {
    Point {
        x: p.x
            + match dir {
                RIGHT => 1,
                LEFT => -1,
                _ => 0,
            },
        y: p.y
            + match dir {
                UP => 1,
                DOWN => -1,
                _ => 0,
            },
    }
}

fn new_top_left(original: Point<i32>, potential: Point<i32>) -> Point<i32> {
    Point {
        x: *vec![original.x, potential.x].iter().min().unwrap(),
        y: *vec![original.y, potential.y].iter().min().unwrap(),
    }
}

fn new_bottom_right(original: Point<i32>, potential: Point<i32>) -> Point<i32> {
    Point {
        x: *vec![original.x, potential.x].iter().max().unwrap(),
        y: *vec![original.y, potential.y].iter().max().unwrap(),
    }
}

fn create_grid(digs: &Vec<Dig>) -> (HashSet<Point<i32>>, (Point<i32>, Point<i32>)) {
    let mut current = Point { x: 0, y: 0 };
    let mut top_left: Point<i32> = current;
    let mut bottom_right: Point<i32> = current;

    let mut grid: HashSet<Point<i32>> = HashSet::new();
    grid.insert(current);

    for dig in digs {
        for _ in 0..dig.steps {
            current = go(current, dig.dir);
            grid.insert(current);

            top_left = new_top_left(top_left, current);
            bottom_right = new_bottom_right(bottom_right, current);
        }
    }

    println!(
        "width = {}, height = {}",
        bottom_right.x - top_left.x + 1,
        bottom_right.y - top_left.y + 1
    );

    (grid, (top_left, bottom_right))
}

fn new_point(x: i32, y: i32) -> Point<i32> {
    Point { x: x, y: y }
}

fn area(grid: &HashSet<Point<i32>>, (top_left, bottom_right): (Point<i32>, Point<i32>)) -> i32 {
    let mut inside = 0;
    for y in top_left.y..=bottom_right.y {
        let mut cross = 0;
        for x in top_left.x..=bottom_right.x {
            let p = Point { x, y };
            if grid.contains(&p) {
                inside += 1;

                // | L J
                if vec![new_point(x, y - 1), new_point(x, y + 1)]
                    .iter()
                    .all(|b| grid.contains(b))
                    || vec![new_point(x - 1, y), new_point(x, y + 1)]
                        .iter()
                        .all(|b| grid.contains(b))
                    || vec![new_point(x + 1, y), new_point(x, y + 1)]
                        .iter()
                        .all(|b| grid.contains(b))
                {
                    cross += 1;
                }
                continue;
            }

            if cross % 2 == 1 {
                inside += 1;
            }
        }
    }

    inside
}

fn handle_pt1(digs: &Vec<Dig>) -> i32 {
    let (grid, bounds) = create_grid(digs);
    area(&grid, bounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let tests = [(
            vec![
                String::from("R 6 (#70c710)"),
                String::from("D 5 (#0dc571)"),
                String::from("L 2 (#5713f0)"),
                String::from("D 2 (#d2c081)"),
                String::from("R 2 (#59c680)"),
                String::from("D 2 (#411b91)"),
                String::from("L 5 (#8ceee2)"),
                String::from("U 2 (#caa173)"),
                String::from("L 1 (#1b58a2)"),
                String::from("U 2 (#caa171)"),
                String::from("R 2 (#7807d2)"),
                String::from("U 3 (#a77fa3)"),
                String::from("L 2 (#015232)"),
                String::from("U 2 (#7a21e3)"),
            ],
            62,
        )];

        for (input, want) in tests {
            let digs = parse_input(&input);
            assert_eq!(handle_pt1(&digs), want, "with input\n{}", input.join("\n"));
            // assert_eq!(
            //     handle_pt2(&grid, corner),
            //     want2,
            //     "with input\n{}",
            //     input.join("\n")
            // );
        }
    }
}
