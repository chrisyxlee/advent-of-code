use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
    let (grid, corner) = create_grid(&lines);

    println!("Part 1: {}", handle_pt1(&grid, corner));
    println!("Part 2: {}", handle_pt2(&grid, corner));
}

fn create_grid(lines: &Vec<String>) -> (HashMap<Point<i32>, i32>, Point<i32>) {
    let height = lines.len() as i32;
    let width = lines.iter().map(|line| line.len()).max().unwrap() as i32;
    let mut grid: HashMap<Point<i32>, i32> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, loss) in line.chars().enumerate() {
            let p = Point {
                x: col as i32,
                y: row as i32,
            };
            grid.insert(p, loss as i32 - '0' as i32);
        }
    }

    (
        grid,
        Point {
            x: width - 1,
            y: height - 1,
        },
    )
}

const WEST: char = 'W';
const EAST: char = 'E';
const NORTH: char = 'N';
const SOUTH: char = 'S';

fn go(p: Point<i32>, dir: char, corner: Point<i32>) -> Option<Point<i32>> {
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

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Visit {
    loc: Point<i32>,
    dir: char,
    streak: i32,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Heat<T> {
    visit: T,
    heat: i32,
}

impl<T: std::cmp::Eq> Ord for Heat<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl<T: std::cmp::Eq> PartialOrd for Heat<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Visit {
    pub fn new(loc: Point<i32>, dir: char, streak: i32) -> Self {
        Self { loc, dir, streak }
    }

    pub fn neighbors(&self, corner: Point<i32>) -> Vec<Visit> {
        let mut res = Vec::new();
        if self.streak < 3 {
            if let Some(forward_loc) = go(self.loc, self.dir, corner) {
                res.push(Visit {
                    loc: forward_loc,
                    dir: self.dir,
                    streak: self.streak + 1,
                });
            }
        }

        let right_dir = self.right();
        if let Some(right_loc) = go(self.loc, right_dir, corner) {
            res.append(&mut vec![Visit {
                loc: right_loc,
                dir: right_dir,
                streak: 1,
            }]);
        }

        let left_dir = self.left();
        if let Some(left_loc) = go(self.loc, left_dir, corner) {
            res.append(&mut vec![Visit {
                loc: left_loc,
                dir: left_dir,
                streak: 1,
            }]);
        }

        res
    }

    fn left(&self) -> char {
        match self.dir {
            WEST => SOUTH,
            SOUTH => EAST,
            EAST => NORTH,
            NORTH => WEST,
            _ => unreachable!(),
        }
    }

    fn right(&self) -> char {
        match self.dir {
            WEST => NORTH,
            NORTH => EAST,
            EAST => SOUTH,
            SOUTH => WEST,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Visit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@ {} -> {} x{}", self.loc, self.dir, self.streak)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct UltraVisit {
    loc: Point<i32>,
    dir: char,
    streak: i32,
}

impl UltraVisit {
    pub fn new(loc: Point<i32>, dir: char, streak: i32) -> Self {
        Self { loc, dir, streak }
    }

    pub fn neighbors(&self, corner: Point<i32>) -> Vec<UltraVisit> {
        let mut res = Vec::new();
        if self.streak < 10 {
            if let Some(forward_loc) = go(self.loc, self.dir, corner) {
                res.push(UltraVisit {
                    loc: forward_loc,
                    dir: self.dir,
                    streak: self.streak + 1,
                });
            }
        }

        if self.streak >= 4 {
            let right_dir = self.right();
            if let Some(right_loc) = go(self.loc, right_dir, corner) {
                res.append(&mut vec![UltraVisit {
                    loc: right_loc,
                    dir: right_dir,
                    streak: 1,
                }]);
            }

            let left_dir = self.left();
            if let Some(left_loc) = go(self.loc, left_dir, corner) {
                res.append(&mut vec![UltraVisit {
                    loc: left_loc,
                    dir: left_dir,
                    streak: 1,
                }]);
            }
        }

        res
    }

    fn left(&self) -> char {
        match self.dir {
            WEST => SOUTH,
            SOUTH => EAST,
            EAST => NORTH,
            NORTH => WEST,
            _ => unreachable!(),
        }
    }

    fn right(&self) -> char {
        match self.dir {
            WEST => NORTH,
            NORTH => EAST,
            EAST => SOUTH,
            SOUTH => WEST,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for UltraVisit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@ {} -> {} x{}", self.loc, self.dir, self.streak)
    }
}

pub struct State {
    heat_map: HashMap<Visit, i32>,
}

impl State {
    pub fn new(corner: Point<i32>) -> Self {
        let mut map = HashMap::new();
        for x in 0..=corner.x {
            for y in 0..=corner.y {
                for dir in vec![NORTH, EAST, WEST, SOUTH] {
                    for streak in 1..3 {
                        map.insert(Visit::new(Point { x: x, y: y }, dir, streak), i32::MAX);
                    }
                }
            }
        }

        Self { heat_map: map }
    }

    pub fn register(&mut self, visit: Visit, heat: i32) {
        self.heat_map
            .entry(visit)
            .and_modify(|e| {
                if *e > heat {
                    *e = heat;
                }
            })
            .or_insert(heat);
    }

    pub fn get_min_heat(&self, loc: Point<i32>) -> i32 {
        *self
            .heat_map
            .iter()
            .filter(|(k, _)| k.loc == loc)
            .map(|(_, v)| v)
            .min()
            .unwrap()
    }
}

fn handle_pt1(grid: &HashMap<Point<i32>, i32>, corner: Point<i32>) -> i32 {
    let mut state = State::new(corner);

    let start = Point { x: 0, y: 0 };

    // Use a sorted map (sorted on key, so key needs to be heat).
    let mut priority_queue = BinaryHeap::new();
    let right = Visit::new(go(start, EAST, corner).unwrap(), EAST, 1);
    let right_heat = *grid.get(&right.loc).unwrap();
    priority_queue.push(Heat {
        heat: right_heat,
        visit: right,
    });

    let down = Visit::new(go(start, SOUTH, corner).unwrap(), SOUTH, 1);
    let down_heat = *grid.get(&down.loc).unwrap();
    priority_queue.push(Heat {
        heat: down_heat,
        visit: down,
    });

    let mut visited: HashSet<Visit> = HashSet::new();

    while let Some(heat_state) = priority_queue.pop() {
        let heat = heat_state.heat;
        let visit = heat_state.visit;
        state.register(visit, heat);
        if !visited.insert(visit) {
            continue;
        }

        for neighbor in visit.neighbors(corner) {
            let neighbor_heat = heat + grid.get(&neighbor.loc).unwrap();
            priority_queue.push(Heat {
                heat: neighbor_heat,
                visit: neighbor,
            });
        }
    }

    state.get_min_heat(corner)
}
pub struct UltraState {
    heat_map: HashMap<UltraVisit, i32>,
}

impl UltraState {
    pub fn new(corner: Point<i32>) -> Self {
        let mut map = HashMap::new();
        for x in 0..=corner.x {
            for y in 0..=corner.y {
                for dir in vec![NORTH, EAST, WEST, SOUTH] {
                    for streak in 1..3 {
                        map.insert(UltraVisit::new(Point { x: x, y: y }, dir, streak), i32::MAX);
                    }
                }
            }
        }

        Self { heat_map: map }
    }

    pub fn register(&mut self, visit: UltraVisit, heat: i32) {
        self.heat_map
            .entry(visit)
            .and_modify(|e| {
                if *e > heat {
                    *e = heat;
                }
            })
            .or_insert(heat);
    }

    pub fn get_min_heat(&self, loc: Point<i32>) -> i32 {
        *self
            .heat_map
            .iter()
            .filter(|(k, _)| k.loc == loc && k.streak >= 4)
            .map(|(_, v)| v)
            .min()
            .unwrap()
    }
}

fn handle_pt2(grid: &HashMap<Point<i32>, i32>, corner: Point<i32>) -> i32 {
    let mut state = UltraState::new(corner);

    let start = Point { x: 0, y: 0 };

    // Use a sorted map (sorted on key, so key needs to be heat).
    let mut priority_queue = BinaryHeap::new();
    let right = UltraVisit::new(go(start, EAST, corner).unwrap(), EAST, 1);
    let right_heat = *grid.get(&right.loc).unwrap();
    priority_queue.push(Heat {
        heat: right_heat,
        visit: right,
    });

    let down = UltraVisit::new(go(start, SOUTH, corner).unwrap(), SOUTH, 1);
    let down_heat = *grid.get(&down.loc).unwrap();
    priority_queue.push(Heat {
        heat: down_heat,
        visit: down,
    });

    let mut visited: HashSet<UltraVisit> = HashSet::new();

    while let Some(heat_state) = priority_queue.pop() {
        let heat = heat_state.heat;
        let visit = heat_state.visit;
        state.register(visit, heat);
        if !visited.insert(visit) {
            continue;
        }

        for neighbor in visit.neighbors(corner) {
            let neighbor_heat = heat + grid.get(&neighbor.loc).unwrap();
            priority_queue.push(Heat {
                heat: neighbor_heat,
                visit: neighbor,
            });
        }
    }

    state.get_min_heat(corner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let tests = [
            (
                vec![
                    String::from("2413432311323"),
                    String::from("3215453535623"),
                    String::from("3255245654254"),
                    String::from("3446585845452"),
                    String::from("4546657867536"),
                    String::from("1438598798454"),
                    String::from("4457876987766"),
                    String::from("3637877979653"),
                    String::from("4654967986887"),
                    String::from("4564679986453"),
                    String::from("1224686865563"),
                    String::from("2546548887735"),
                    String::from("4322674655533"),
                ],
                (102, 94),
            ),
            (
                vec![
                    String::from("111111111111"),
                    String::from("999999999991"),
                    String::from("999999999991"),
                    String::from("999999999991"),
                    String::from("999999999991"),
                ],
                (59, 71),
            ),
        ];

        for (input, (want1, want2)) in tests {
            let (grid, corner) = create_grid(&input);
            assert_eq!(
                handle_pt1(&grid, corner),
                want1,
                "with input\n{}",
                input.join("\n")
            );
            assert_eq!(
                handle_pt2(&grid, corner),
                want2,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
}
