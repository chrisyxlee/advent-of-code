use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::fmt;

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

impl fmt::Display for Dig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.steps, self.dir)
    }
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

fn handle_pt1(lines: &Vec<String>) -> i64 {
    let digs = lines
        .iter()
        .map(|l| parse_line_pt1(&l))
        .collect::<Vec<Dig>>();

    shoelace(&digs)
}

fn determinant(a: Point<i64>, b: Point<i64>) -> i64 {
    a.x * b.y - b.x * a.y
}

fn is_counter_clockwise(digs: &Vec<Dig>) -> bool {
    digs.iter().nth(1).unwrap().dir
        == match digs.first().unwrap().dir {
            RIGHT => UP,
            UP => LEFT,
            LEFT => DOWN,
            DOWN => RIGHT,
            _ => unreachable!(),
        }
}

fn shoelace(digs: &Vec<Dig>) -> i64 {
    let mut points = Vec::new();
    let mut curr: Point<i64> = Point { x: 0, y: 0 };
    let mut surface_area = 0;
    points.push(curr);
    for dig in digs {
        surface_area += dig.steps;
        curr = jump(curr, dig.dir, dig.steps);
        points.push(curr);
    }

    if !is_counter_clockwise(digs) {
        points.reverse();
    }

    let mut prev = *points.last().unwrap();
    for i in 0..points.len() {
        let curr = points[i];
        surface_area += determinant(prev, curr);
        prev = curr;
    }

    (surface_area.abs() / 2) + 1
}

fn handle_pt2(lines: &Vec<String>) -> i64 {
    let digs = lines
        .iter()
        .map(|l| parse_line_pt2(&l))
        .collect::<Vec<Dig>>();

    shoelace(&digs)
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
