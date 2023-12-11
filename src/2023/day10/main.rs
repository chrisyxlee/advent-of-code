use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::collections::HashMap;
use std::collections::HashSet;
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

    let pt1: i32 = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
    let pt2: i32 = handle_pt2(&lines);
    // 172 too low
    // 730 too high
    // 600 too high
    println!("Part 2: {}", pt2);
}

fn dir(p: Point<i32>, x: i32, y: i32) -> Point<i32> {
    Point {
        x: p.x + x,
        y: p.y + y,
    }
}

fn west(p: Point<i32>) -> Point<i32> {
    dir(p, -1, 0)
}
fn east(p: Point<i32>) -> Point<i32> {
    dir(p, 1, 0)
}
fn north(p: Point<i32>) -> Point<i32> {
    dir(p, 0, 1)
}
fn south(p: Point<i32>) -> Point<i32> {
    dir(p, 0, -1)
}

fn connections(p: Point<i32>, shape: char) -> Vec<Point<i32>> {
    match shape {
        '|' => vec![north(p), south(p)],
        '-' => vec![west(p), east(p)],
        'L' => vec![north(p), east(p)],
        'J' => vec![north(p), west(p)],
        '7' => vec![south(p), west(p)],
        'F' => vec![south(p), east(p)],
        'S' => vec![north(p), south(p), west(p), east(p)],
        _ => vec![],
    }
}

fn get_neighbors(grid: &HashMap<Point<i32>, char>, point: Point<i32>) -> Vec<Point<i32>> {
    connections(point, *grid.get(&point).unwrap())
        .into_iter()
        .filter(|neighbor| {
            if let Some(neighbor_shape) = grid.get(&neighbor) {
                if connections(*neighbor, *neighbor_shape).contains(&point) {
                    return true;
                }
            }
            return false;
        })
        .collect::<Vec<Point<i32>>>()
}

fn handle_pt1(lines: &Vec<String>) -> i32 {
    let mut grid: HashMap<Point<i32>, char> = HashMap::new();
    let mut start: Point<i32> = Point { x: 0, y: 0 };
    for (row, line) in lines.iter().enumerate() {
        for (col, shape) in line.chars().enumerate() {
            let p = Point {
                x: col as i32,
                y: lines.len() as i32 - row as i32 - 1,
            };
            match shape {
                '.' => continue,
                'S' => start = p,
                _ => {}
            }
            grid.insert(p, shape);
        }
    }

    let mut distances: HashMap<Point<i32>, i32> = HashMap::new();
    distances.insert(start, 0);
    let mut queue: Vec<Point<i32>> = vec![start];
    while !queue.is_empty() {
        let current = queue.remove(0);
        let neighbors = get_neighbors(&grid, current);
        queue.append(
            &mut neighbors
                .clone()
                .into_iter()
                .filter(|x| !distances.contains_key(x))
                .collect::<Vec<Point<i32>>>(),
        );

        if let Some(min_dist) = neighbors
            .iter()
            .map(|n| distances.get(n))
            .filter(|n| n.is_some())
            .map(|n| n.unwrap())
            .min()
        {
            let new_dist = *min_dist + 1;
            if let Some(dist) = distances.get(&current) {
                if *dist < new_dist {
                    continue;
                }
            }

            distances.insert(current, new_dist);
        }
    }

    let mut max_distance: i32 = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, _) in line.chars().enumerate() {
            let p = Point {
                x: col as i32,
                y: lines.len() as i32 - row as i32 - 1,
            };
            if let Some(dist) = distances.get(&p) {
                if max_distance < *dist {
                    max_distance = *dist;
                }
            }
        }
    }

    max_distance
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct TraversablePoint {
    pub a: Point<i32>,
    pub b: Option<Point<i32>>,
}

impl fmt::Display for TraversablePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.b.is_some() {
            return write!(f, "({}, {})", self.a, self.b.unwrap());
        } else {
            return write!(f, "{}", self.a);
        }
    }
}

fn in_loop(loop_bounds: &HashSet<Point<i32>>, height: i32, width: i32, point: Point<i32>) -> bool {
    println!("for point {}", point);
    let mut p = point;
    let mut cross = 0;
    let mut inside = true;
    while p.x < width {
        p = east(p);
        if loop_bounds.contains(&p) {
            cross += 1;
        }
    }
    println!("  east = {}", cross);
    inside = inside && cross % 2 == 1;

    p = point;
    cross = 0;
    while p.x >= 0 {
        p = west(p);
        if loop_bounds.contains(&p) {
            cross += 1;
        }
    }
    println!("  west = {}", cross);
    inside = inside && cross % 2 == 1;

    p = point;
    cross = 0;
    while p.y >= 0 {
        p = south(p);
        if loop_bounds.contains(&p) {
            cross += 1;
        }
    }
    println!("  south = {}", cross);
    inside = inside && cross % 2 == 1;

    p = point;
    cross = 0;
    while p.y < height {
        p = north(p);
        if loop_bounds.contains(&p) {
            cross += 1;
        }
    }
    println!("  north = {}", cross);
    inside = inside && cross % 2 == 1;

    inside
}

fn handle_pt2(lines: &Vec<String>) -> i32 {
    let height = lines.len() as i32;
    let width = lines.iter().map(|line| line.len()).max().unwrap() as i32;
    //  println!("new height is {}, new width is {}", height, width);

    let mut grid: HashMap<Point<i32>, char> = HashMap::new();
    let mut start: Point<i32> = Point { x: 0, y: 0 };
    for (row, line) in lines.iter().enumerate() {
        for (col, shape) in line.chars().enumerate() {
            let p = Point {
                x: col as i32,
                y: lines.len() as i32 - row as i32 - 1,
            };
            match shape {
                'S' => start = p,
                _ => {}
            }
            grid.insert(p, shape);
        }
    }

    let mut loop_bounds: HashSet<Point<i32>> = HashSet::new();
    loop_bounds.insert(start);
    let mut queue: Vec<Point<i32>> = vec![start];
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        queue.append(
            &mut get_neighbors(&grid, current)
                .into_iter()
                .filter(|x| !loop_bounds.contains(x))
                .collect::<Vec<Point<i32>>>(),
        );

        loop_bounds.insert(current);
    }

    grid.entry(start).and_modify(|e| {
        *e = match (
            loop_bounds.contains(&north(start)),
            loop_bounds.contains(&east(start)),
            loop_bounds.contains(&south(start)),
            loop_bounds.contains(&west(start)),
        ) {
            (true, true, false, false) => 'L',
            (true, false, true, false) => '|',
            (true, false, false, true) => 'J',
            (false, true, true, false) => 'F',
            (false, true, false, true) => '-',
            (false, false, true, true) => '7',
            _ => 'S',
        };
    });

    // Remove the non-loop points.
    for r in 0..height {
        for c in 0..width {
            let p = Point { x: c, y: r };
            if !loop_bounds.contains(&p) {
                if let Some(shape) = grid.get(&p) {
                    if *shape != '.' {
                        grid.entry(p).and_modify(|e| *e = '.');
                    }
                }
            }
        }
    }

    let mut inside_points: HashSet<Point<i32>> = HashSet::new();
    let mut inside_count = 0;
    for r in 0..height {
        let mut cross = 0;
        println!("reset");
        for c in 0..width {
            let p = Point { x: c, y: r };
            if loop_bounds.contains(&p) {
                // Only the pipes that have an edge going up.
                if vec!['|', 'J', 'L'].contains(grid.get(&p).unwrap()) {
                    println!("going right, crossed {} at {}", grid.get(&p).unwrap(), p);
                    cross += 1;
                }
                continue;
            }

            if cross % 2 == 1 {
                println!("inserting {}", p);
                inside_points.insert(p);
                inside_count += 1;
            }
        }
    }

    println!("CHECK ANSWER");
    for r in (0..height).rev() {
        for c in 0..width {
            let p = Point { x: c, y: r };
            if loop_bounds.contains(&p) {
                print!("{}", grid.get(&p).unwrap());
            } else if inside_points.contains(&p) {
                print!("I");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    inside_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [
            (
                vec![
                    String::from("....."),
                    String::from(".S-7."),
                    String::from(".|.|."),
                    String::from(".L-J."),
                    String::from("....."),
                ],
                4,
            ),
            (
                vec![
                    String::from("..F7."),
                    String::from(".FJ|."),
                    String::from("SJ.L7"),
                    String::from("|F--J"),
                    String::from("LJ..."),
                ],
                8,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [
            // (
            //     vec![
            //         String::from("S--7"),
            //         String::from("|..|"),
            //         String::from("L--J"),
            //     ],
            //     2,
            // ),
            (
                vec![
                    String::from("..........."),
                    String::from(".S-------7."),
                    String::from(".|F-----7|."),
                    String::from(".||.....||."),
                    String::from(".||.....||."),
                    String::from(".|L-7.F-J|."),
                    String::from(".|..|.|..|."),
                    String::from(".L--J.L--J."),
                    String::from("..........."),
                ],
                4,
            ),
            (
                vec![
                    String::from(".........."),
                    String::from(".S------7."),
                    String::from(".|F----7|."),
                    String::from(".||....||."),
                    String::from(".||....||."),
                    String::from(".|L-7F-J|."),
                    String::from(".|..||..|."),
                    String::from(".L--JL--J."),
                    String::from(".........."),
                ],
                4,
            ),
            (
                vec![
                    String::from(".F----7F7F7F7F-7...."),
                    String::from(".|F--7||||||||FJ...."),
                    String::from(".||.FJ||||||||L7...."),
                    String::from("FJL7L7LJLJ||LJ.L-7.."),
                    String::from("L--J.L7...LJS7F-7L7."),
                    String::from("....F-J..F7FJ|L7L7L7"),
                    String::from("....L7.F7||L7|.L7L7|"),
                    String::from(".....|FJLJ|FJ|F7|.LJ"),
                    String::from("....FJL-7.||.||||..."),
                    String::from("....L---J.LJ.LJLJ..."),
                ],
                8,
            ),
            (
                vec![
                    String::from("FF7FSF7F7F7F7F7F---7"),
                    String::from("L|LJ||||||||||||F--J"),
                    String::from("FL-7LJLJ||||||LJL-77"),
                    String::from("F--JF--7||LJLJ7F7FJ-"),
                    String::from("L---JF-JLJ.||-FJLJJ7"),
                    String::from("|F|F-JF---7F7-L7L|7|"),
                    String::from("|FFJF7L7F-JF7|JL---7"),
                    String::from("7-L-JL7||F7|L7F-7F7|"),
                    String::from("L.L7LFJ|||||FJL7||LJ"),
                    String::from("L7JLJL-JLJLJL--JLJ.L"),
                ],
                10,
            ),
            (
                vec![
                    String::from("F---7"),
                    String::from("|S--J"),
                    String::from("|L--7"),
                    String::from("L---J"),
                ],
                0,
            ),
            (
                vec![
                    String::from("F----7"),
                    String::from("|.S--J"),
                    String::from("|||---"),
                    String::from("|.L--7"),
                    String::from("L----J"),
                ],
                3,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
