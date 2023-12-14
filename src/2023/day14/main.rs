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
    //  let pt2: usize = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
}

fn handle_pt1(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let mut grid: HashMap<Point<usize>, char> = HashMap::new();
    let mut start: Point<usize> = Point { x: 0, y: 0 };

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
                    grid.entry(new_p).and_modify(|e| *e = 'O');
                    grid.entry(p).and_modify(|e| *e = '.');
                    total += max_location;
                    max_location -= 1;
                }
                Some('#') => max_location = h-1,
                _ => todo!(),
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
}
