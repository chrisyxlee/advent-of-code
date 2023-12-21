use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
use clap::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::time::Instant;

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
    let (grid, dims, start) = create_grid(&lines);

    let mut trace = Instant::now();
    println!("Part 1: {}", handle_pt1(&grid, dims, start, 64));
    println!("Elapsed: {:.2?}", trace.elapsed());

    trace = Instant::now();
    println!("Part 2: {}", handle_pt2(&grid, dims, start, 26501365));
    println!("Elapsed: {:.2?}", trace.elapsed());
}

enum Plot {
    GARDEN = 1,
    ROCK = 2,
}

impl fmt::Display for Plot{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(
           f,
           "{}",
           match self {
               Plot::GARDEN=> ".",
               Plot::ROCK => "#",
           }
       )
   }
}
fn create_grid(lines: &Vec<String>) -> (HashMap<Point<i64>, Plot>, Point<i64>, Point<i64>) {
    let height = lines.len() as i64;
    let width = lines.iter().map(|line| line.len()).max().unwrap() as i64;
    let mut grid: HashMap<Point<i64>, Plot> = HashMap::new();
    let mut start = Point { x: 0, y: 0 };

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let p = Point {
                x: col as i64,
                y: row as i64,
            };
            grid.insert(
                p,
                match c {
                    '#' => Plot::ROCK,
                    _ => Plot::GARDEN,
                },
            );
            if c == 'S' {
                start = p;
            }
        }
    }

    (
        grid,
        Point {
            x: width,
            y: height,
        },
        start,
    )
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Direction {
    WEST = 1,
    EAST = 2,
    NORTH = 3,
    SOUTH = 4,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Dist {
    ODD = 1,
    EVEN = 2,
    BOTH = 3,
}

impl fmt::Display for Dist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dist::EVEN => "even",
                Dist::ODD => "odd",
                Dist::BOTH => "both",
            }
        )
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::SOUTH => "S",
                Direction::EAST => "E",
                Direction::NORTH => "N",
                Direction::WEST => "W",
            }
        )
    }
}

fn go(p: Point<i64>, dir: Direction, dims: Point<i64>) -> Option<Point<i64>> {
    let new_point = Point {
        x: p.x
            + match dir {
                Direction::EAST => 1,
                Direction::WEST => -1,
                _ => 0,
            },
        y: p.y
            + match dir {
                Direction::NORTH => -1,
                Direction::SOUTH => 1,
                _ => 0,
            },
    };

    if new_point.x < 0 || new_point.x >= dims.x || new_point.y < 0 || new_point.y >= dims.y {
        return None;
    }

    Some(new_point)
}

fn go_void(p: Point<i64>, dir: Direction) -> Point<i64> {
    Point {
        x: p.x
            + match dir {
                Direction::EAST => 1,
                Direction::WEST => -1,
                _ => 0,
            },
        y: p.y
            + match dir {
                Direction::NORTH => -1,
                Direction::SOUTH => 1,
                _ => 0,
            },
    }
}

fn real_coord(p: Point<i64>, dims: Point<i64>) -> Point<i64> {
    Point {
        x: p.x.rem_euclid(dims.x),
        y: p.y.rem_euclid(dims.y),
    }
}

fn handle_pt1(
    grid: &HashMap<Point<i64>, Plot>,
    dims: Point<i64>,
    start: Point<i64>,
    total_steps: i64,
) -> i64 {
    let mut steps = 0;

    let mut possible = HashSet::new();
    possible.insert(start);

    while steps < total_steps {
        steps += 1;

        let mut next_possible = HashSet::new();
        for p in possible.iter() {
            for dir in vec![
                Direction::NORTH,
                Direction::EAST,
                Direction::WEST,
                Direction::SOUTH,
            ] {
                if let Some(new_p) = go(*p, dir, dims) {
                    if let Some(plot) = grid.get(&new_p) {
                        match plot {
                            Plot::ROCK => continue,
                            Plot::GARDEN => {
                                next_possible.insert(new_p);
                            }
                        }
                    }
                }
            }
        }

        possible = next_possible;
    }

    possible.len() as i64
}

fn is_odd(x: i64) -> bool {
    return x.rem_euclid(2) == 1;
}

fn handle_pt2(
    grid: &HashMap<Point<i64>, Plot>,
    dims: Point<i64>,
    start: Point<i64>,
    total_steps: i64,
) -> i64 {
   //  println!("start = {}, dims = {}", start, dims);

    let mut visited: HashSet<(Point<i64>, Dist)> = HashSet::new();

    let mut next = VecDeque::new();
    next.push_back((start, 0));

    while let Some((p, steps)) = next.pop_front() {
        if visited.contains(&(p, Dist::BOTH)) {

            continue;
        } else if (!is_odd(steps) && visited.contains(&(p, Dist::EVEN)))
            || (is_odd(steps) && visited.contains(&(p, Dist::ODD)))
        {
            continue;
        } else if (is_odd(steps) && visited.contains(&(p, Dist::EVEN)))
            || (!is_odd(steps) && visited.contains(&(p, Dist::ODD)))
        {
            visited.insert((p, Dist::BOTH));
        } else {
            visited.insert((
                p,
                match is_odd(steps) {
                    true => Dist::ODD,
                    false => Dist::EVEN,
                },
            ));
        }

        //   println!("\n\nSTEP {}", steps);

        if steps == total_steps {
            // println!("p {} is at total steps {}", p, steps);
            continue;
        }
        for dir in vec![
            Direction::NORTH,
            Direction::EAST,
            Direction::WEST,
            Direction::SOUTH,
        ] {
            let new_p = go_void(p, dir);
            let real = real_coord(new_p, dims);
            let plot = grid.get(&real).unwrap();
            match plot {
                Plot::ROCK => continue,
                Plot::GARDEN => {
                    next.push_back((new_p, steps + 1));
                }
            }
            //  println!("{} -{}-> {} (really {})", p, dir, new_p, real);
        }
    }

    let mut possible = HashSet::new();
    for (p, v) in visited {
      //   println!("possible {} {}", p, v);
        if v == Dist::BOTH
            || v == match is_odd(total_steps) {
                true => Dist::ODD,
                false => Dist::EVEN,
            }
        {
            possible.insert(p);
            // println!("possible {}", p);
        }
    }
   //  println!("{}" , possible.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("\n"));

   //  println!("");
   //  for y in 0..dims.y {
   //    for x in 0..dims.x {
   //       let p = Point{x,y};
   //       print!("{}", match grid.get(&p) {
   //          Some(Plot::ROCK) => "#",
   //          Some(Plot::GARDEN) => match possible.contains(&p) {
   //             true => "O",
   //             false => ".",
   //          },
   //          _ => unreachable!(),
   //       })
   //    }
   //       // println!("");
   //  }

    possible.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_pt1() {
        let tests = [(
            (
                vec![
                    "...........",
                    ".....###.#.",
                    ".###.##..#.",
                    "..#.#...#..",
                    "....#.#....",
                    ".##..S####.",
                    ".##..#...#.",
                    ".......##..",
                    ".##.#.####.",
                    ".##..##.##.",
                    "...........",
                ],
                6,
            ),
            16,
        )];

        for ((input, steps), want) in tests {
            let (grid, dims, start) =
                create_grid(&input.iter().map(|x| x.to_string()).collect::<Vec<String>>());
            assert_eq!(
                handle_pt1(&grid, dims, start, steps),
                want,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
    #[test]
    fn test_handle_pt2() {
        let input = vec![
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
        let tests = [
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
            (5000, 16733044),
        ];

        for (steps, want) in tests {
            let (grid, dims, start) = create_grid(&input);
            assert_eq!(
                handle_pt2(&grid, dims, start, steps),
                want,
                "for {} steps for input\n{}",
                steps,
                input.join("\n")
            );
        }
    }
}
