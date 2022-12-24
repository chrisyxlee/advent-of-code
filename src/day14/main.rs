use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ROCK: char = '#';
const EMPTY: char = ' ';
const SAND: char = '.';

static START: Point = Point { x: 500, y: 0 };

fn main() {
    // --snip--
    let file_path = "tmp/day14/input.txt";
    println!("In file {}", file_path);

    {
        let mut input = parse_input(file_path);
        let pt1 = drop_sand_pt1(&mut input);
        println!("Part 1: {}", pt1);
    }
    {
        let mut input = parse_input(file_path);
        let pt2 = drop_sand_pt2(&mut input);
        println!("Part 2: {}", pt2);
    }
}

// To make it easier not to mess up x and y.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn parse(s: &str) -> Self {
        let parts = s.split(",").collect::<Vec<&str>>();
        assert!(parts.len() == 2, "A point should only have x,y: {}", s);
        return Point::new(s2i(parts[0]), s2i(parts[1]));
    }

    fn get_inclusive_between(&self, p: Point) -> Vec<Point> {
        assert!(
            self.x == p.x || self.y == p.y,
            "Points should differ only in one dimension. {} and {}",
            self,
            p,
        );

        let mut res: Vec<Point> = Vec::new();
        if self.x == p.x {
            let tmp = [self.y, p.y];
            for y in *tmp.iter().min().unwrap()..=*tmp.iter().max().unwrap() {
                res.push(Point::new(self.x, y));
            }
        }
        if self.y == p.y {
            let tmp = [self.x, p.x];
            for x in *tmp.iter().min().unwrap()..=*tmp.iter().max().unwrap() {
                res.push(Point::new(x, self.y));
            }
        }
        return res;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
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
                print!("{}", get(m, Point::new(x, y)));
            }
        }
        println!("");
    }
}

fn drop_sand_pt1(m: &mut HashMap<i32, HashMap<i32, char>>) -> i32 {
    let mut curr: Point = START;
    let max_y = get_lowest_point(m);
    let mut inserted = 0;
    loop {
        let down = Point::new(curr.x, curr.y + 1);
        let left = Point::new(curr.x - 1, curr.y + 1);
        let right = Point::new(curr.x + 1, curr.y + 1);
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
    let mut curr: Point = START;
    loop {
        let next_y = curr.y + 1;
        let down = Point::new(curr.x, next_y);
        let left = Point::new(curr.x - 1, next_y);
        let right = Point::new(curr.x + 1, next_y);
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

fn parse_input<P>(filename: P) -> HashMap<i32, HashMap<i32, char>>
where
    P: AsRef<Path>,
{
    let lines: Vec<String> = io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect();

    let mut input = HashMap::new();
    for line in lines {
        let points = line.split(" -> ").collect::<Vec<&str>>();
        assert!(points.len() > 1, "Only one point?");
        let mut prev = Point::parse(points[0]);
        for p in 1..points.len() {
            let curr = Point::parse(points[p]);
            for point in prev.get_inclusive_between(curr) {
                insert(&mut input, point, ROCK);
            }
            prev = curr;
        }
    }

    return input;
}

fn can_insert(m: &mut HashMap<i32, HashMap<i32, char>>, p: Point) -> bool {
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

fn get(m: &mut HashMap<i32, HashMap<i32, char>>, p: Point) -> char {
    let by_row = m.entry(p.x).or_insert(HashMap::new());
    return *by_row.entry(p.y).or_insert(EMPTY);
}

fn insert(m: &mut HashMap<i32, HashMap<i32, char>>, p: Point, c: char) {
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
                Point::new(0, 5),
                Point::new(0, 7),
                Vec::from([Point::new(0, 5), Point::new(0, 6), Point::new(0, 7)]),
            ),
            (
                Point::new(1, 5),
                Point::new(2, 5),
                Vec::from([Point::new(1, 5), Point::new(2, 5)]),
            ),
        ];
        for (a, b, want) in tests {
            let gota = a.get_inclusive_between(b);
            let gotb = b.get_inclusive_between(a);
            assert!(gota == want, "{} to {} == {:?}\ngot {:?}", a, b, want, gota);
            assert!(gotb == want, "{} to {} == {:?}\ngot {:?}", b, a, want, gotb);
        }
    }
}
