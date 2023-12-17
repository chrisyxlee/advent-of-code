use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::collections::{HashMap, HashSet};

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

    // 2464 too high
    // 2454 too high
    // 2453 too high
    // 2421 too high
    println!("Part 1: {}", handle_pt1(&grid, corner));
    //  println!("Part 2: {}", handle_pt2(&grid, width, height));
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

fn backwards(dir: Option<char>, new_dir: char) -> bool {
    new_dir
        == match dir {
            Some(WEST) => EAST,
            Some(EAST) => WEST,
            Some(NORTH) => SOUTH,
            Some(SOUTH) => NORTH,
            None => '?',
            _ => unreachable!(),
        }
}

fn straight(dir: Option<char>, new_dir: char) -> bool {
    dir.is_some() && dir.unwrap() == new_dir
}

pub struct State {
    min_heat: Option<i32>,
}

impl State {
    pub fn new() -> Self {
        Self { min_heat: None }
    }

    pub fn register(&mut self, heat: i32) {
        if let Some(mh) = self.min_heat {
            if heat < mh {
                self.min_heat = Some(heat);
                println!("NEW MIN = {}", heat);
            }
        } else {
            self.min_heat = Some(heat);
            println!("NEW MIN = {}", heat);
        }
    }

    pub fn futile(&self, heat: i32) -> bool {
        self.min_heat.is_some() && heat >= self.min_heat.unwrap()
    }
}

fn lose_heat(
    grid: &HashMap<Point<i32>, i32>,
    corner: Point<i32>,
    p: Point<i32>,
    dir: Option<char>,
    streak: i32,
    heat: i32,
    state: &mut State,
    visited: &mut HashSet<Point<i32>>,
) {
    visited.insert(p);

    /*
    if let Some(d) = dir {
        println!(
            "p = {}, dir = {}, streak = {}, heat = {}, visited = {}, {}",
            p,
            d,
            streak,
            heat,
            visited.len(),
            visited
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    } else {
        println!(
            "start | p = {}, streak = {}, heat = {}, visited = {}, {}",
            p,
            streak,
            heat,
            visited.len(),
            visited
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    */

    if p == corner {
        //   println!("YAY {}", heat);
        state.register(heat);
        return;
    }

    if state.futile(heat) {
        //   println!(
        //       "futile {} (currently smallest = {})",
        //       heat,
        //       state.min_heat.unwrap()
        //   );
        return;
    }

    for new_dir in vec![NORTH, EAST, SOUTH, WEST] {
        if backwards(dir, new_dir) {
            continue;
        }

        if let Some(new_p) = go(p, new_dir, corner) {
            let mut new_streak = 1;
            if straight(dir, new_dir) {
                new_streak = streak + 1;
                if new_streak > 3 {
                    //   println!("exceeded 3 streak {}", p);
                    continue;
                }
            }

            if visited.contains(&new_p) {
                //  println!("visited {}", new_p);
                continue;
            }

            lose_heat(
                grid,
                corner,
                new_p,
                Some(new_dir),
                new_streak,
                heat + grid.get(&new_p).unwrap(),
                state,
                &mut visited.clone(),
            );
        }
    }
}

fn handle_pt1(grid: &HashMap<Point<i32>, i32>, corner: Point<i32>) -> i32 {
    let mut state = State::new();
    lose_heat(
        grid,
        corner,
        Point { x: 0, y: 0 },
        None,
        0,
        0,
        &mut state,
        &mut HashSet::new(),
    );

    state.min_heat.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let tests = [(
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
            102,
        )];

        for (input, want) in tests {
            let (grid, corner) = create_grid(&input);
            assert_eq!(
                handle_pt1(&grid, corner),
                want,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
}
