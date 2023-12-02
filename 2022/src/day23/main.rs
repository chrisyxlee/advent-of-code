use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day23/input.txt";
    println!("In file {}", file_path);

    {
        println!("Part 1: {}", part_1(&read_lines(file_path)));
    }
    {
        // 1020 is too low
        println!("Part 2: {}", part_2(&read_lines(file_path)));
    }
}

fn part_1(lines: &Vec<String>) -> i64 {
    let mut grid = parse_grid(lines);
    for i in 0..10 {
        perform_round(&mut grid, i);
    }
    return count_empty(&grid);
}

fn part_2(lines: &Vec<String>) -> usize {
    let mut round: usize = 0;
    let mut grid = parse_grid(lines);
    while perform_round(&mut grid, round) {
        round += 1;
    }
    return round + 1;
}

fn locations_to_check(p: Point, d: usize) -> [Point; 3] {
    match d {
        N => [p.to(NW), p.to(N), p.to(NE)],
        W => [p.to(SW), p.to(W), p.to(NW)],
        S => [p.to(SW), p.to(S), p.to(SE)],
        E => [p.to(SE), p.to(E), p.to(NE)],
        _ => {
            assert!(false, "invalid direction: {}", d);
            [p, p, p]
        }
    }
}

fn perform_round(grid: &mut Grid, round: usize) -> bool {
    let mut dst2src: HashMap<Point, Point> = HashMap::new();
    let mut conflicts: HashSet<Point> = HashSet::new();

    for &p in grid.iter() {
        if [N, S, W, E, NW, SW, NE, SE]
            .iter()
            .all(|&l| !grid.contains(&p.to(l)))
        {
            continue;
        }
        for d in 0..DIRECTIONS.len() {
            let dir = DIRECTIONS[(round + d) % DIRECTIONS.len()];
            if locations_to_check(p, dir).iter().all(|l| !grid.contains(l)) {
                let dst = p.to(dir);
                if dst2src.contains_key(&dst) {
                    conflicts.insert(dst);
                } else {
                    *dst2src.entry(dst).or_insert(p) = p;
                }
                break;
            }
        }
    }

    for c in conflicts.iter() {
        dst2src.remove(c);
    }

    let num_moves = dst2src.len();
    for (dst, src) in dst2src {
        grid.remove(&src);
        grid.insert(dst);
    }
    return num_moves > 0;
}

fn count_empty(grid: &Grid) -> i64 {
    let south = get_most(&grid, S);
    let north = get_most(&grid, N);
    let west = get_most(&grid, W);
    let east = get_most(&grid, E);
    let mut empty = 0;
    for y in (south..=north).rev() {
        for x in west..=east {
            if !grid.contains(&Point::new(x, y)) {
                empty += 1;
            }
        }
    }
    return empty;
}

const W: usize = 0;
const E: usize = 1;
const N: usize = 2;
const S: usize = 3;
const NW: usize = 4;
const NE: usize = 5;
const SW: usize = 6;
const SE: usize = 7;

const DIRECTIONS: [usize; 4] = [N, S, W, E];

const ELF: char = '#';
const _EMPTY: char = '.';

type Grid = HashSet<Point>;

// To make it easier not to mess up x and y.
#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Point {
    x: i64, // column
    y: i64, // row
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x: x, y: y }
    }

    fn to(&self, d: usize) -> Point {
        match d {
            N => Point::new(self.x, self.y + 1),
            W => Point::new(self.x - 1, self.y),
            E => Point::new(self.x + 1, self.y),
            S => Point::new(self.x, self.y - 1),
            NW => Point::new(self.x - 1, self.y + 1),
            NE => Point::new(self.x + 1, self.y + 1),
            SW => Point::new(self.x - 1, self.y - 1),
            SE => Point::new(self.x + 1, self.y - 1),
            _ => {
                assert!(false, "invalid direction: {}", d);
                Point::new(0, 0)
            }
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn _print_grid(grid: &Grid) {
    let south = get_most(grid, S);
    let north = get_most(grid, N);
    let west = get_most(grid, W);
    let east = get_most(grid, E);
    println!("Grid: x[{},{}] y[{},{}]", west, east, south, north);
    for y in (south..=north).rev() {
        for x in west..=east {
            if grid.contains(&Point::new(x, y)) {
                print!("{}", ELF);
            } else {
                print!("{}", _EMPTY);
            }
        }
        println!("");
    }
}

fn get_most(grid: &Grid, dir: usize) -> i64 {
    match dir {
        N => return grid.iter().map(|&p| p.y).max().unwrap(),
        S => return grid.iter().map(|&p| p.y).min().unwrap(),
        W => return grid.iter().map(|&p| p.x).min().unwrap(),
        E => return grid.iter().map(|&p| p.x).max().unwrap(),
        _ => assert!(false, "invalid direction: {}", dir),
    }
    return 0;
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect::<Vec<String>>();
}

fn parse_grid(lines: &Vec<String>) -> Grid {
    let mut grid: Grid = Grid::new();
    let mut y: i64 = 0;
    for line in lines {
        for (x, c) in line.chars().enumerate() {
            match c {
                ELF => _ = grid.insert(Point::new(x as i64, y)),
                _ => (),
            }
        }

        y -= 1;
    }
    return grid;
}

fn str_vec(v: &Vec<&str>) -> Vec<String> {
    v.iter().map(|&x| String::from(x)).collect::<Vec<String>>()
}

fn _eq_grid(a: &Grid, b: &Grid) {
    let a_south = get_most(a, S);
    let a_north = get_most(a, N);
    let a_west = get_most(a, W);
    let a_east = get_most(a, E);
    let b_south = get_most(b, S);
    let b_north = get_most(b, N);
    let b_west = get_most(b, W);
    let b_east = get_most(b, E);
    assert_eq!(a_north - a_south, b_north - b_south);
    assert_eq!(a_east - a_west, b_east - b_west);
    for y in 0..=(a_north - a_south) {
        for x in 0..=(a_east - a_west) {
            let ac = a.contains(&Point::new(a_west + x, a_south + y));
            let bc = b.contains(&Point::new(b_west + x, b_south + y));
            assert!(
                (ac && bc) || (!ac && !bc),
                "Grid A = {}, Grid B = {}, (x={},y={})",
                ac,
                bc,
                x,
                y
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = str_vec(&Vec::from([
            "..............", //
            "..............", //
            ".......#......", //
            ".....###.#....", //
            "...#...#.#....", //
            "....#...##....", //
            "...#.###......", //
            "...##.#.##....", //
            "....#..#......", //
            "..............", //
            "..............", //
            "..............", //
        ]));
        let mut grid = parse_grid(&input);

        perform_round(&mut grid, 0);
        _eq_grid(
            &parse_grid(&str_vec(&Vec::from([
                "..............",
                ".......#......",
                ".....#...#....",
                "...#..#.#.....",
                ".......#..#...",
                "....#.#.##....",
                "..#..#.#......",
                "..#.#.#.##....",
                "..............",
                "....#..#......",
                "..............",
                "..............",
            ]))),
            &grid,
        );

        perform_round(&mut grid, 1);
        _eq_grid(
            &parse_grid(&str_vec(&Vec::from([
                "..............",
                ".......#......",
                "....#.....#...",
                "...#..#.#.....",
                ".......#...#..",
                "...#..#.#.....",
                ".#...#.#.#....",
                "..............",
                "..#.#.#.##....",
                "....#..#......",
                "..............",
                "..............",
            ]))),
            &grid,
        );

        perform_round(&mut grid, 2);
        assert_eq!(
            count_empty(&parse_grid(&str_vec(&Vec::from([
                "..............",
                ".......#......",
                ".....#....#...",
                "..#..#...#....",
                ".......#...#..",
                "...#..#.#.....",
                ".#..#.....#...",
                ".......##.....",
                "..##.#....#...",
                "...#..........",
                ".......#......",
                "..............",
            ])))),
            count_empty(&grid)
        );

        perform_round(&mut grid, 3);
        assert_eq!(
            count_empty(&parse_grid(&str_vec(&Vec::from([
                "..............",
                ".......#......",
                "......#....#..",
                "..#...##......",
                "...#.....#.#..",
                ".........#....",
                ".#...###..#...",
                "..#......#....",
                "....##....#...",
                "....#.........",
                ".......#......",
                "..............",
            ])))),
            count_empty(&grid)
        );

        perform_round(&mut grid, 4);
        assert_eq!(
            count_empty(&parse_grid(&str_vec(&Vec::from([
                ".......#......",
                "..............",
                "..#..#.....#..",
                ".........#....",
                "......##...#..",
                ".#.#.####.....",
                "...........#..",
                "....##..#.....",
                "..#...........",
                "..........#...",
                "....#..#......",
                "..............",
            ])))),
            count_empty(&grid)
        );

        /*
         == End of Round 5 ==
        After a few more rounds...

         == End of Round 10 ==
         .......#......
         ...........#..
         ..#.#..#......
         ......#.......
         ...#.....#..#.
         .#......##....
         .....##.......
         ..#........#..
         ....#.#..#....
         ..............
         ....#..#..#...
         ..............
         */

        //            assert_eq!(110, part_1(&input));
    }

    #[test]
    fn small_example_works() {
        let input = str_vec(&Vec::from([
            ".....", //
            "..##.", //
            "..#..", //
            ".....", //
            "..##.", //
            ".....", //
        ]));
        let mut grid = parse_grid(&input);
        assert_eq!(3, count_empty(&grid));

        perform_round(&mut grid, 0);
        _eq_grid(
            &parse_grid(&str_vec(&Vec::from([
                "##", //
                "..", //
                "#.", //
                ".#", //
                "#.", //
            ]))),
            &grid,
        );
        assert_eq!(5, count_empty(&grid));

        perform_round(&mut grid, 1);
        _eq_grid(
            &parse_grid(&str_vec(&Vec::from([
                ".##.", //
                "#...", //
                "...#", //
                "....", //
                ".#..", //
            ]))),
            &grid,
        );
        assert_eq!(15, count_empty(&grid));

        perform_round(&mut grid, 2);
        _eq_grid(
            &parse_grid(&str_vec(&Vec::from([
                "..#..", //
                "....#", //
                "#....", //
                "....#", //
                ".....", //
                "..#..", //
            ]))),
            &grid,
        );
        assert_eq!(25, count_empty(&grid));
    }
}
