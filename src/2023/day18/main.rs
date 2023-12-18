use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashSet};

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

    println!("Part 1: {}", handle_pt1(&lines));
    println!("Part 2: {}", handle_pt2(&lines));
}

const UP: char = 'U';
const DOWN: char = 'D';
const LEFT: char = 'L';
const RIGHT: char = 'R';

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Dig {
    steps: i64,
    dir: char,
}

fn parse_line_pt1(s: &str) -> Dig {
    let mut dig = Dig { steps: 0, dir: '?' };
    for (i, part) in s.split(' ').enumerate() {
        match i {
            0 => dig.dir = *part.chars().collect::<Vec<char>>().first().unwrap(),
            1 => dig.steps = part.parse::<i64>().unwrap(),
            _ => {}
        }
    }
    dig
}

fn parse_line_pt2(s: &str) -> Dig {
    let mut dig = Dig { steps: 0, dir: '?' };
    for (i, part) in s.split(' ').enumerate() {
        match i {
            2 => {
                let stripped = part.strip_prefix("(#").unwrap().strip_suffix(")").unwrap();
                let last_char = stripped.chars().last().unwrap();
                dig.dir = match last_char {
                    '0' => RIGHT,
                    '1' => DOWN,
                    '2' => LEFT,
                    '3' => UP,
                    _ => unreachable!(),
                };
                dig.steps =
                    i64::from_str_radix(stripped.strip_suffix(last_char).unwrap(), 16).unwrap();
            }
            _ => {}
        }
    }
    dig
}

fn go(p: Point<i64>, dir: char) -> Point<i64> {
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

fn jump(p: Point<i64>, dir: char, steps: i64) -> Point<i64> {
    Point {
        x: p.x
            + match dir {
                RIGHT => steps,
                LEFT => -steps,
                _ => 0,
            },
        y: p.y
            + match dir {
                UP => steps,
                DOWN => -steps,
                _ => 0,
            },
    }
}

fn new_top_left(original: Point<i64>, potential: Point<i64>) -> Point<i64> {
    Point {
        x: *vec![original.x, potential.x].iter().min().unwrap(),
        y: *vec![original.y, potential.y].iter().min().unwrap(),
    }
}

fn new_bottom_right(original: Point<i64>, potential: Point<i64>) -> Point<i64> {
    Point {
        x: *vec![original.x, potential.x].iter().max().unwrap(),
        y: *vec![original.y, potential.y].iter().max().unwrap(),
    }
}

fn create_grid_pt1(digs: &Vec<Dig>) -> (HashSet<Point<i64>>, (Point<i64>, Point<i64>)) {
    let mut current = Point { x: 0, y: 0 };
    let mut top_left: Point<i64> = current;
    let mut bottom_right: Point<i64> = current;

    let mut grid: HashSet<Point<i64>> = HashSet::new();
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

fn new_point(x: i64, y: i64) -> Point<i64> {
    Point { x: x, y: y }
}

fn area(grid: &HashSet<Point<i64>>, (top_left, bottom_right): (Point<i64>, Point<i64>)) -> i64 {
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

fn handle_pt1(lines: &Vec<String>) -> i64 {
    let digs = lines
        .iter()
        .map(|l| parse_line_pt1(&l))
        .collect::<Vec<Dig>>();
    let (grid, bounds) = create_grid_pt1(&digs);
    area(&grid, bounds)
}

fn handle_pt2(lines: &Vec<String>) -> i64 {
    let digs = lines
        .iter()
        .map(|l| parse_line_pt2(&l))
        .collect::<Vec<Dig>>();

    let mut curr: Point<i64> = Point { x: 0, y: 0 };
    let mut vertical_lines: BTreeMap<i64, BinaryHeap<(i64, i64)>> = BTreeMap::new();

    let mut space = 0;
    for dig in digs {
        space += dig.steps;
        let next = jump(curr, dig.dir, dig.steps);
        if !vec![UP, DOWN].contains(&dig.dir) {
            continue;
        }

        let ys = vec![curr.y, next.y];
        let line = (*ys.iter().min().unwrap(), *ys.iter().max().unwrap());
        vertical_lines
            .entry(curr.x)
            .and_modify(|e| e.push(line))
            .or_insert(BinaryHeap::from(vec![line]));

        curr = next;
    }
    // TODO
    // find overlap ranges -- split them up into pairs -- get the area between the pairs

    space
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
            (62, 952408144115),
        )];

        for (input, (want1, want2)) in tests {
            assert_eq!(
                handle_pt1(&input),
                want1,
                "with input\n{}",
                input.join("\n")
            );
            assert_eq!(
                handle_pt2(&input),
                want2,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
}
