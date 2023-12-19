use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};
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

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct VerticalLine {
    x: i64,
    y_start: i64,
    y_end: i64,
}

impl VerticalLine {
    fn contains_y(&self, y: i64) -> bool {
        self.y_start <= y && y <= self.y_end
    }

    fn equal_y(&self, other: &Self) -> bool {
        self.y_start == other.y_start && self.y_end == other.y_end
    }

    fn overlaps_y(&self, other: &Self) -> bool {
        self.contains_y(other.y_start)
            || self.contains_y(other.y_end)
            || other.contains_y(self.y_start)
            || other.contains_y(self.y_end)
    }
}

impl fmt::Display for VerticalLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <-> {} @ {}", self.y_start, self.y_end, self.x)
    }
}

impl Ord for VerticalLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y_start
            .cmp(&other.y_start)
            .then(self.y_end.cmp(&other.y_end))
            .then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for VerticalLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn split_vertical_ranges(lines: &Vec<VerticalLine>) -> Vec<VerticalLine> {
    let mut curr_lines = lines.clone();
    let mut count = 0;
    loop {
        println!("curr lines has length {}", curr_lines.len());
        let mut split_lines = Vec::new();
        let ys = curr_lines
            .iter()
            .map(|l| vec![l.y_start, l.y_end])
            .flatten()
            .collect::<Vec<i64>>();
        while i < curr_lines.len() {
            if group.is_empty() {
                group.push(curr_lines[i]);
            } else if group.last().unwrap().overlaps_y(&curr_lines[i]) {
                group.push(curr_lines[i]);
            }
            if self.y_end < other.y_start {
                return vec![*self, *other];
            }

            let mut ys = vec![self.y_start, self.y_end, other.y_start, other.y_end];

            ys.sort();
            ys.dedup();

            for i in 0..ys.len() - 1 {
                let mut y_start = ys[i];
                let mut y_end = ys[i + 1];

                if i > 0 && self.contains_y(y_start) && other.contains_y(y_start) {
                    y_start += 1;
                }

                if i + 1 < ys.len() - 1 && !(self.contains_y(y_end) && other.contains_y(y_end)) {
                    y_end -= 1;
                }

                if self.contains_y(y_start) && self.contains_y(y_end) {
                    res.push(VerticalLine {
                        x: self.x,
                        y_start: y_start,
                        y_end: y_end,
                    });
                }

                if other.contains_y(y_start) && other.contains_y(y_end) {
                    res.push(VerticalLine {
                        x: other.x,
                        y_start: y_start,
                        y_end: y_end,
                    });
                }
            }
        }
        split_lines.sort();
        split_lines.dedup();

        if split_lines.len() == curr_lines.len() {
            println!("got it");
            break;
        }

        curr_lines = split_lines;
        println!(
            "now is
{}",
            curr_lines
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );

        count += 1;
        if count == 2 {
            return Vec::new();
        }
    }

    curr_lines
}

fn handle_pt2(lines: &Vec<String>) -> i64 {
    let digs = lines
        .iter()
        .map(|l| parse_line_pt2(&l))
        .collect::<Vec<Dig>>();

    println!("part 2: got digs");
    println!(
        "{}",
        digs.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );

    //  let mut vertical_lines: BTreeMap<i64, BinaryHeap<(i64, i64)>> = BTreeMap::new();
    let mut space = 0;
    let mut vertical_lines: Vec<VerticalLine> = Vec::new();
    {
        let mut raw_lines: Vec<VerticalLine> = Vec::new();
        let mut curr: Point<i64> = Point { x: 0, y: 0 };
        for dig in digs {
            space += dig.steps;
            let next = jump(curr, dig.dir, dig.steps);
            println!("next = {} after {} ", next, dig);
            if vec![UP, DOWN].contains(&dig.dir) {
                let ys = vec![curr.y, next.y];
                raw_lines.push(VerticalLine {
                    x: curr.x,
                    y_start: *ys.iter().min().unwrap(),
                    y_end: *ys.iter().max().unwrap(),
                });
                // vertical_lines
                //     .entry(min_y)
                //     .and_modify(|e| e.push((curr.x, max_y)))
                //     .or_insert(BinaryHeap::from(vec![line]));
            }

            curr = next;
        }
        raw_lines.sort();

        println!("part 2: got vertical lines, spaces = {}", space);
        println!(
            "{}",
            raw_lines
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );

        // Split
        vertical_lines = split_vertical_ranges(&raw_lines);

        println!("part 2: split lines");
    }

    let mut i = 0;
    let mut xs: Vec<VerticalLine> = Vec::new();
    let mut y_now: VerticalLine = VerticalLine {
        y_start: 0,
        y_end: 0,
        x: 0,
    };
    while i < vertical_lines.len() {
        let curr = vertical_lines[i];

        if xs.is_empty() {
            xs.push(curr);
            y_now = curr;
            continue;
        }

        if y_now.equal_y(&curr) {
            xs.push(curr);
            continue;
        } else {
            let mut l = 0;
            while l < xs.len() {
                if l + 1 >= xs.len() {
                    break;
                }

                let height = xs[l].y_end - xs[l].y_start;
                let width = xs[l + 1].x - xs[l].x;

                space += height * width;

                l += 2;
            }

            xs.clear();
        }

        i += 1;
    }

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

    #[test]
    fn test_split_vertical_ranges() {
        let tests = [(
            vec![
                VerticalLine {
                    y_start: -919647,
                    y_end: -56407,
                    x: 818608,
                },
                VerticalLine {
                    y_start: -500254,
                    y_end: 0,
                    x: 0,
                },
                VerticalLine {
                    y_start: -56407,
                    y_end: 0,
                    x: 461937,
                },
            ],
            vec![
                VerticalLine {
                    x: 0,
                    y_start: 0,
                    y_end: 5,
                },
                VerticalLine {
                    x: 5,
                    y_start: 10,
                    y_end: 15,
                },
            ],
        )];

        for (that, want) in tests {
            assert_eq!(
                split_vertical_ranges(&that),
                want,
                "{}",
                that.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }
    }

    #[test]
    fn test_split() {
        let tests = [
            (
                "different x, same y",
                (
                    VerticalLine {
                        x: 0,
                        y_start: 10,
                        y_end: 15,
                    },
                    VerticalLine {
                        x: 5,
                        y_start: 10,
                        y_end: 15,
                    },
                ),
                vec![
                    VerticalLine {
                        x: 0,
                        y_start: 10,
                        y_end: 15,
                    },
                    VerticalLine {
                        x: 5,
                        y_start: 10,
                        y_end: 15,
                    },
                ],
            ),
            (
                "same y start, one is longer",
                (
                    VerticalLine {
                        x: 0,
                        y_start: 0,
                        y_end: 5,
                    },
                    VerticalLine {
                        x: 12,
                        y_start: 0,
                        y_end: 15,
                    },
                ),
                vec![
                    VerticalLine {
                        x: 0,
                        y_start: 0,
                        y_end: 5,
                    },
                    VerticalLine {
                        x: 12,
                        y_start: 0,
                        y_end: 5,
                    },
                    VerticalLine {
                        x: 12,
                        y_start: 6,
                        y_end: 15,
                    },
                ],
            ),
            (
                "not overlapping",
                (
                    VerticalLine {
                        x: 0,
                        y_start: 0,
                        y_end: 5,
                    },
                    VerticalLine {
                        x: 5,
                        y_start: 10,
                        y_end: 15,
                    },
                ),
                vec![
                    VerticalLine {
                        x: 0,
                        y_start: 0,
                        y_end: 5,
                    },
                    VerticalLine {
                        x: 5,
                        y_start: 10,
                        y_end: 15,
                    },
                ],
            ),
        ];

        for (desc, (this, that), want) in tests {
            assert_eq!(this.split(&that), want, "{}", desc);
        }
    }
}
