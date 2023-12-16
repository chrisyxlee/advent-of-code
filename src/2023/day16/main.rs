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

    // 8244 too low
    println!("Part 1: {}", handle_pt1(&lines));
    //  println!("Part 2: {}", handle_pt2(&lines));
}

const WEST: char = 'W';
const EAST: char = 'E';
const NORTH: char = 'N';
const SOUTH: char = 'S';

fn go(p: Point<i32>, dir: char, width: i32, height: i32) -> Option<(Point<i32>, char)> {
    let new_point = Point {
        x: p.x
            + match dir {
                EAST => 1,
                WEST => -1,
                _ => 0,
            },
        y: p.y
            + match dir {
                NORTH => 1,
                SOUTH => -1,
                _ => 0,
            },
    };

    if new_point.x < 0 || new_point.x >= width || new_point.y < 0 || new_point.y >= height {
        return None;
    }

    Some((new_point, dir))
}

fn handle_pt1(lines: &Vec<String>) -> i32 {
    let height = lines.len() as i32;
    let width = lines.iter().map(|line| line.len()).max().unwrap() as i32;
    let mut grid: HashMap<Point<i32>, char> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, shape) in line.chars().enumerate() {
            let p = Point {
                x: col as i32,
                y: lines.len() as i32 - row as i32 - 1,
            };
            grid.insert(p, shape);
        }
    }

    let start = Point {
        x: 0,
        y: height - 1,
    };

    let mut visited: HashSet<(Point<i32>, char)> = HashSet::new();

    let mut light: HashSet<Point<i32>> = HashSet::new();
    light.insert(start);

    let mut beams: Vec<(Point<i32>, char)> = Vec::new();
    beams.push((start, 'E'));

    while !beams.is_empty() {
        //   println!("");
        //   println!("ROUND");
        let mut next_beams: Vec<(Point<i32>, char)> = Vec::new();
        for (loc, dir) in beams {
            if visited.contains(&(loc, dir)) {
                continue;
            }

            next_beams.append(
                &mut match (grid.get(&loc).unwrap(), dir) {
                    ('-', NORTH) => {
                        vec![go(loc, WEST, width, height), go(loc, EAST, width, height)]
                    }
                    ('-', SOUTH) => {
                        vec![go(loc, WEST, width, height), go(loc, EAST, width, height)]
                    }
                    ('|', EAST) => {
                        vec![go(loc, NORTH, width, height), go(loc, SOUTH, width, height)]
                    }
                    ('|', WEST) => {
                        vec![go(loc, NORTH, width, height), go(loc, SOUTH, width, height)]
                    }
                    ('/', NORTH) => vec![go(loc, EAST, width, height)],
                    ('/', EAST) => vec![go(loc, NORTH, width, height)],
                    ('/', WEST) => vec![go(loc, SOUTH, width, height)],
                    ('/', SOUTH) => vec![go(loc, WEST, width, height)],
                    ('\\', NORTH) => vec![go(loc, WEST, width, height)],
                    ('\\', EAST) => vec![go(loc, SOUTH, width, height)],
                    ('\\', WEST) => vec![go(loc, NORTH, width, height)],
                    ('\\', SOUTH) => vec![go(loc, EAST, width, height)],
                    _ => vec![go(loc, dir, width, height)],
                }
                .iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<(Point<i32>, char)>>(),
            );
            light.insert(loc);
            visited.insert((loc, dir));

            // println!(
            //     "{}",
            //     next_beams
            //         .iter()
            //         .map(|(loc, dir)| format!("{} {}", loc, dir).to_string())
            //         .collect::<Vec<String>>()
            //         .join("\n")
            // );
        }

        //   show_grid(&grid, width, height);
        //   show_lights(&light, width, height);

        beams = next_beams;
    }

    let mut total = 0;
    for w in 0..width {
        for h in 0..height {
            if light.contains(&Point { x: w, y: h }) {
                total += 1;
            }
        }
    }

    total
}

fn show_grid(grid: &HashMap<Point<i32>, char>, width: i32, height: i32) {
    let mut res: Vec<char> = Vec::new();
    for h in (0..height).rev() {
        for w in 0..width {
            let p = Point { x: w, y: h };
            res.push(*grid.get(&p).unwrap());
        }

        res.push('\n');
    }

    println!("{}", res.iter().collect::<String>());
}

fn show_lights(grid: &HashSet<Point<i32>>, width: i32, height: i32) {
    let mut res: Vec<char> = Vec::new();
    for h in (0..height).rev() {
        for w in 0..width {
            let p = Point { x: w, y: h };
            if grid.contains(&p) {
                res.push('#');
            } else {
                res.push('.');
            }
        }

        res.push('\n');
    }

    println!("{}", res.iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid() {
        let tests = [(
            vec![
                String::from(r".|...\...."),
                String::from(r"|.-.\....."),
                String::from(r".....|-..."),
                String::from(r"........|."),
                String::from(r".........."),
                String::from(r".........\"),
                String::from(r"..../.\\.."),
                String::from(r".-.-/..|.."),
                String::from(r".|....-|.\"),
                String::from(r"..//.|...."),
            ],
            46,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "with input\n{}", input.join("\n"));
        }
    }
}
