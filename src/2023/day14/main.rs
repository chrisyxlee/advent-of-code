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

    let pt1: usize = handle_pt1(&lines);
    // 108518 not right
    println!("Part 1: {}", pt1);
    let pt2: usize = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
}

fn handle_pt1(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let mut grid: HashMap<Point<usize>, char> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, shape) in line.chars().enumerate() {
            let p = Point {
                x: col + 1 as usize,
                y: lines.len() as usize - row as usize,
            };
            grid.insert(p, shape);
        }
    }

    let mut total = 0;
    for w in 1..=width {
        let mut max_location = height;
        for h in (1..=height).rev() {
            let p = Point { x: w, y: h };
            match grid.get(&p) {
                Some('.') => {}
                Some('O') => {
                    let new_p = Point {
                        x: w,
                        y: max_location,
                    };
                    grid.entry(p).and_modify(|e| *e = '.');
                    grid.entry(new_p).and_modify(|e| *e = 'O');
                    total += max_location;
                    max_location -= 1;
                }
                Some('#') => max_location = h - 1,
                _ => todo!(),
            }
        }
    }

    total
}

fn grid_string(grid: &HashMap<Point<usize>, char>, width: usize, height: usize) -> String {
    let mut res: Vec<char> = Vec::new();
    for h in 1..=height {
        for w in 1..=width {
            let p = Point { x: w, y: h };
            res.push(*grid.get(&p).unwrap());
        }
    }

    res.iter().collect::<String>()
}

fn show_grid(grid: &HashMap<Point<usize>, char>, width: usize, height: usize) {
    let mut res: Vec<char> = Vec::new();
    for h in (1..=height).rev() {
        for w in 1..=width {
            let p = Point { x: w, y: h };
            res.push(*grid.get(&p).unwrap());
        }

        res.push('\n');
    }

    println!("{}", res.iter().collect::<String>());
}

fn parse_grid_string(s: &str, width: usize) -> HashMap<Point<usize>, char> {
    let mut grid: HashMap<Point<usize>, char> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        let w = i % width + 1;
        let h = i / width + 1;
        grid.insert(Point { x: w, y: h }, c);
    }

    grid
}

fn handle_pt2(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let mut grid: HashMap<Point<usize>, char> = HashMap::new();

    let mut memo: HashMap<String, i32> = HashMap::new();
    let mut rev_memo: HashMap<i32, String> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, shape) in line.chars().enumerate() {
            let p = Point {
                x: col + 1 as usize,
                y: lines.len() as usize - row as usize,
            };
            grid.insert(p, shape);
        }
    }

    println!("START");
    show_grid(&grid, width, height);
    let total_cycles = 1000000000;
    let mut cycle = 0;
    let mut key: String = String::from("");
    while cycle < total_cycles {
        // North
        for w in 1..=width {
            let mut next = height;
            for h in (1..=height).rev() {
                let p = Point { x: w, y: h };
                match grid.get(&p) {
                    Some('.') => {}
                    Some('O') => {
                        let new_p = Point { x: w, y: next };
                        grid.entry(p).and_modify(|e| *e = '.');
                        grid.entry(new_p).and_modify(|e| *e = 'O');
                        next -= 1;
                    }
                    Some('#') => next = h - 1,
                    _ => todo!(),
                }
            }
        }
        //   println!("NORTH");
        //   show_grid(&grid, width, height);

        // West
        for h in 1..=height {
            let mut next = 1;
            for w in 1..=width {
                let p = Point { x: w, y: h };
                match grid.get(&p) {
                    Some('.') => {}
                    Some('O') => {
                        let new_p = Point { x: next, y: h };
                        grid.entry(p).and_modify(|e| *e = '.');
                        grid.entry(new_p).and_modify(|e| *e = 'O');
                        next += 1;
                    }
                    Some('#') => next = w + 1,
                    _ => todo!(),
                }
            }
        }
        //   println!("WEST");
        //   show_grid(&grid, width, height);

        // South
        for w in 1..=width {
            let mut next = 1;
            for h in 1..=height {
                let p = Point { x: w, y: h };
                match grid.get(&p) {
                    Some('.') => {}
                    Some('O') => {
                        let new_p = Point { x: w, y: next };
                        grid.entry(p).and_modify(|e| *e = '.');
                        grid.entry(new_p).and_modify(|e| *e = 'O');
                        next += 1;
                    }
                    Some('#') => next = h + 1,
                    _ => todo!(),
                }
            }
        }
        //   println!("SOUTH");
        //   show_grid(&grid, width, height);

        // East
        for h in 1..=height {
            let mut next = width;
            for w in (1..=width).rev() {
                let p = Point { x: w, y: h };
                match grid.get(&p) {
                    Some('.') => {}
                    Some('O') => {
                        let new_p = Point { x: next, y: h };
                        grid.entry(p).and_modify(|e| *e = '.');
                        grid.entry(new_p).and_modify(|e| *e = 'O');
                        next -= 1;
                    }
                    Some('#') => next = w - 1,
                    _ => todo!(),
                }
            }
        }
        //   println!("CYCLE {}", cycle);
        //   show_grid(&grid, width, height);

        key = grid_string(&grid, width, height);
        if let Some(past_cycle) = memo.get(&key) {
            let period = cycle - past_cycle;
            let final_cycle = past_cycle + ((total_cycles - cycle) % period) - 1;
            // println!(
            //     "currently {}, past cycle = {}, with period = {}, ending up on = {}",
            //     cycle, past_cycle, period, final_cycle
            // );
            grid = parse_grid_string(rev_memo.get(&final_cycle).unwrap(), width);
            break;
        }

        //   println!("inserting {} for cycle {}", &key, cycle);
        memo.insert(key.clone(), cycle);
        rev_memo.insert(cycle, key.clone());
        cycle += 1;
    }

    //  println!("FINAL");
    //  show_grid(&grid, width, height);
    let mut total = 0;
    for h in 1..=height {
        for w in 1..=width {
            let p = Point { x: w, y: h };
            match grid.get(&p) {
                Some('O') => {
                    total += h;
                }
                _ => {}
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid() {
        let tests = [(
            vec![
                String::from("O....#...."),
                String::from("O.OO#....#"),
                String::from(".....##..."),
                String::from("OO.#O....O"),
                String::from(".O.....O#."),
                String::from("O.#..O.#.#"),
                String::from("..O..#O..O"),
                String::from(".......O.."),
                String::from("#....###.."),
                String::from("#OO..#...."),
            ],
            136,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "with input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_pt2() {
        let tests = [(
            vec![
                String::from("O....#...."),
                String::from("O.OO#....#"),
                String::from(".....##..."),
                String::from("OO.#O....O"),
                String::from(".O.....O#."),
                String::from("O.#..O.#.#"),
                String::from("..O..#O..O"),
                String::from(".......O.."),
                String::from("#....###.."),
                String::from("#OO..#...."),
            ],
            64,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "with input\n{}", input.join("\n"));
        }
    }
}
