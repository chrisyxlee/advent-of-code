use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::collections::HashMap;

const ROCK: char = '#';
const EMPTY: char = ' ';
const SAND: char = '.';

static START: Point<i32> = Point::<i32> { x: 500, y: 0 };

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

    {
        let mut input = parse_input(&lines);
        let pt1 = drop_sand_pt1(&mut input);
        println!("Part 1: {}", pt1);
    }
    {
        let mut input = parse_input(&lines);
        let pt2 = drop_sand_pt2(&mut input);
        println!("Part 2: {}", pt2);
    }
}

fn parse_point(s: &str) -> Point<i32> {
    let parts = s.split(",").collect::<Vec<&str>>();
    assert!(parts.len() == 2, "A point should only have x,y: {}", s);
    return Point {
        x: s2i(parts[0]),
        y: s2i(parts[1]),
    };
}

fn point_range_inclusive(from: Point<i32>, to: Point<i32>) -> Vec<Point<i32>> {
    assert!(
        from.x == to.x || from.y == to.y,
        "Points should differ only in one dimension. {} and {}",
        from,
        to,
    );

    let mut res: Vec<Point<i32>> = Vec::new();
    if from.x == to.x {
        let tmp = [from.y, to.y];
        for y in *tmp.iter().min().unwrap()..=*tmp.iter().max().unwrap() {
            res.push(Point::<i32> { x: from.x, y: y });
        }
    }
    if from.y == to.y {
        let tmp = [from.x, to.x];
        for x in *tmp.iter().min().unwrap()..=*tmp.iter().max().unwrap() {
            res.push(Point::<i32> { x: x, y: from.y });
        }
    }
    return res;
}

fn _print_map(m: &mut HashMap<i32, HashMap<i32, char>>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for x in m.keys() {
        left.push(x);
        right.push(x);
        left = Vec::from([*left.iter().min().unwrap()]);
        right = Vec::from([*right.iter().max().unwrap()]);
    }
    let leftest = *left[0];
    let rightest = *right[0];
    let lowest = get_lowest_point(m);
    println!(
        "LEFT: {}, RIGHT: {}, TOP: 0, BOTTOM: {}",
        leftest, rightest, lowest
    );

    for y in 0..lowest {
        for x in leftest..=rightest {
            if x == 500 && y == 0 {
                print!("S");
            } else {
                print!("{}", get(m, Point::<i32> { x: x, y: y }));
            }
        }
        println!("");
    }
}

fn drop_sand_pt1(m: &mut HashMap<i32, HashMap<i32, char>>) -> i32 {
    let mut curr: Point<i32> = START;
    let max_y = get_lowest_point(m);
    let mut inserted = 0;
    loop {
        let down = Point::<i32> {
            x: curr.x,
            y: curr.y + 1,
        };
        let left = Point::<i32> {
            x: curr.x - 1,
            y: curr.y + 1,
        };
        let right = Point::<i32> {
            x: curr.x + 1,
            y: curr.y + 1,
        };
        if curr.y > max_y {
            return inserted;
        }

        if can_insert(m, down) {
            curr = down;
            continue;
        }

        if can_insert(m, left) {
            curr = left;
            continue;
        }

        if can_insert(m, right) {
            curr = right;
            continue;
        }

        insert(m, curr, SAND);
        inserted += 1;
        curr = START;
    }
}

fn drop_sand_pt2(m: &mut HashMap<i32, HashMap<i32, char>>) -> i32 {
    let max_y = get_lowest_point(m) + 2;
    let mut inserted = 0;
    let mut curr: Point<i32> = START;
    loop {
        let next_y = curr.y + 1;
        let down = Point::<i32> {
            x: curr.x,
            y: next_y,
        };
        let left = Point::<i32> {
            x: curr.x - 1,
            y: next_y,
        };
        let right = Point::<i32> {
            x: curr.x + 1,
            y: next_y,
        };
        if next_y != max_y {
            if can_insert(m, down) {
                curr = down;
                continue;
            }

            if can_insert(m, left) {
                curr = left;
                continue;
            }

            if can_insert(m, right) {
                curr = right;
                continue;
            }
        }

        insert(m, curr, SAND);
        inserted += 1;
        if curr == START {
            return inserted;
        }

        curr = START;
    }
}

fn parse_input(lines: &Vec<String>) -> HashMap<i32, HashMap<i32, char>> {
    let mut input = HashMap::new();
    for line in lines {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        assert!(points.len() > 1, "Only one point?");
        let mut prev = parse_point(points[0]);
        for p in 1..points.len() {
            let curr = parse_point(points[p]);
            for point in point_range_inclusive(prev, curr) {
                insert(&mut input, point, ROCK);
            }
            prev = curr;
        }
    }

    return input;
}

fn can_insert(m: &mut HashMap<i32, HashMap<i32, char>>, p: Point<i32>) -> bool {
    return get(m, p) == EMPTY;
}

fn get_lowest_point(m: &mut HashMap<i32, HashMap<i32, char>>) -> i32 {
    let mut tmp = Vec::new();
    for ym in m.values() {
        for y in ym.keys() {
            tmp.push(y);
            tmp = Vec::from([*tmp.iter().max().unwrap()]);
        }
    }
    return *tmp[0];
}

fn get(m: &mut HashMap<i32, HashMap<i32, char>>, p: Point<i32>) -> char {
    let by_row = m.entry(p.x).or_insert(HashMap::new());
    return *by_row.entry(p.y).or_insert(EMPTY);
}

fn insert(m: &mut HashMap<i32, HashMap<i32, char>>, p: Point<i32>, c: char) {
    let by_row = m.entry(p.x).or_insert(HashMap::new());
    *by_row.entry(p.y).or_insert(c) = c;
}

fn s2i(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_dist_works() {
        let tests = [
            (
                Point::<i32> { x: 0, y: 5 },
                Point::<i32> { x: 0, y: 7 },
                Vec::from([
                    Point::<i32> { x: 0, y: 5 },
                    Point::<i32> { x: 0, y: 6 },
                    Point::<i32> { x: 0, y: 7 },
                ]),
            ),
            (
                Point::<i32> { x: 1, y: 5 },
                Point::<i32> { x: 2, y: 5 },
                Vec::from([Point::<i32> { x: 1, y: 5 }, Point::<i32> { x: 2, y: 5 }]),
            ),
        ];
        for (a, b, want) in tests {
            let gota = point_range_inclusive(a, b);
            let gotb = point_range_inclusive(b, a);
            assert!(gota == want, "{} to {} == {:?}\ngot {:?}", a, b, want, gota);
            assert!(gotb == want, "{} to {} == {:?}\ngot {:?}", b, a, want, gotb);
        }
    }
}
