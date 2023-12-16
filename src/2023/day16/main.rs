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
    let (grid, (width, height)) = create_grid(&lines);

    println!("Part 1: {}", handle_pt1(&grid, width, height));
    println!("Part 2: {}", handle_pt2(&grid, width, height));
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

fn create_grid(lines: &Vec<String>) -> (HashMap<Point<i32>, char>, (i32, i32)) {
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

    (grid, (width, height))
}

fn shoot_beams(
    grid: &HashMap<Point<i32>, char>,
    width: i32,
    height: i32,
    start: Point<i32>,
    start_dir: char,
) -> i32 {
    let mut visited: HashSet<(Point<i32>, char)> = HashSet::new();

    let mut light: HashSet<Point<i32>> = HashSet::new();
    light.insert(start);

    let mut beams: Vec<(Point<i32>, char)> = Vec::new();
    beams.push((start, start_dir));

    while !beams.is_empty() {
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
        }

        beams = next_beams;
    }

    (0..width)
        .into_iter()
        .map(|w| {
            (0..height)
                .into_iter()
                .map(|h| light.contains(&Point { x: w, y: h }))
                .filter(|x| *x)
                .count()
        })
        .sum::<usize>()
        .try_into()
        .unwrap()
}

fn handle_pt1(grid: &HashMap<Point<i32>, char>, width: i32, height: i32) -> i32 {
    shoot_beams(
        grid,
        width,
        height,
        Point {
            x: 0,
            y: height - 1,
        },
        EAST,
    )
}

fn handle_pt2(grid: &HashMap<Point<i32>, char>, width: i32, height: i32) -> i32 {
    let mut max_lights = 0;

    let mut starts: Vec<(Point<i32>, char)> = Vec::new();
    for h in 0..height {
        match h {
            0 => starts.append(&mut vec![
                (Point { x: 0, y: h }, EAST),
                (Point { x: 0, y: h }, SOUTH),
                (Point { x: width - 1, y: h }, WEST),
                (Point { x: width - 1, y: h }, SOUTH),
            ]),
            height => starts.append(&mut vec![
                (Point { x: 0, y: h }, EAST),
                (Point { x: 0, y: h }, SOUTH),
                (Point { x: width - 1, y: h }, WEST),
                (Point { x: width - 1, y: h }, NORTH),
            ]),
            _ => starts.append(&mut vec![
                (Point { x: 0, y: h }, EAST),
                (Point { x: width - 1, y: h }, WEST),
            ]),
        }
    }
    for w in 1..width - 1 {
        starts.append(&mut vec![
            (Point { x: w, y: 0 }, NORTH),
            (
                Point {
                    x: w,
                    y: height - 1,
                },
                SOUTH,
            ),
        ]);
    }

    starts
        .iter()
        .map(|(start_loc, start_dir)| shoot_beams(grid, width, height, *start_loc, *start_dir))
        .max()
        .unwrap()
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
