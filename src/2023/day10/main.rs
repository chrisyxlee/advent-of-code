use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::collections::HashMap;

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
    //  let pt2: i32 = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
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
}
