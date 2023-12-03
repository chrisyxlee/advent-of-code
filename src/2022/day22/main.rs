use advent_of_code::utils::input::read_lines;
use clap::Parser;

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

    {
        let exposed = part_1(&lines);
        // 57232 is too low.
        // 58236 is too low.
        // 64256
        println!("Part 1: {}", exposed);
    }
}

fn part_1(lines: &Vec<String>) -> usize {
    let map_end = lines.iter().position(|l| l.len() == 0).unwrap();
    let mut map: Vec<&str> = Vec::new();
    for i in 0..map_end {
        map.push(lines[i].as_str());
    }

    let mut rows: Vec<Slice> = Vec::new();
    let mut cols: Vec<Slice> = Vec::new();
    let num_columns = map.iter().map(|&l| l.len()).max().unwrap();

    for line in &map {
        rows.push(Slice::parse(line));
    }

    for i in 0..num_columns {
        let mut v: Vec<char> = Vec::new();
        for line in lines {
            let cs = line.chars().collect::<Vec<char>>();
            if cs.len() <= i {
                v.push(WRAP);
            } else {
                v.push(line.chars().collect::<Vec<char>>()[i]);
            }
        }
        cols.push(Slice::from(v));
    }

    assert!(rows.len() > 0);
    assert!(cols.len() > 0);

    let start = Point::new(
        rows[0].offset + rows[0].elems.iter().position(|&c| c == OK).unwrap(),
        0,
    );
    let mut state = State::new(start);
    let instructions = Instruction::parse(lines[lines.len() - 1].as_str());

    for &inst in &instructions {
        state.update(inst, &rows, &cols);
    }

    return (state.loc.y + 1) * 1000
        + (state.loc.x + 1) * 4
        + match state.dir.dir {
            RIGHT => 0,
            DOWN => 1,
            LEFT => 2,
            UP => 3,
            _ => {
                assert!(false);
                4
            }
        };
}

const OK: char = '.';
const WALL: char = '#';
const WRAP: char = ' ';

const LEFT: char = 'L';
const RIGHT: char = 'R';
const UP: char = 'U';
const DOWN: char = 'D';

#[derive(Debug, Clone)]
pub struct Slice {
    offset: usize,
    elems: Vec<char>,
}

impl Slice {
    pub fn parse(line: &str) -> Self {
        Self {
            offset: line.chars().position(|c| c != WRAP).unwrap(),
            elems: line.chars().filter(|&c| c != WRAP).collect::<Vec<char>>(),
        }
    }
    pub fn from(v: Vec<char>) -> Self {
        let mut elems: Vec<char> = Vec::new();
        let mut first_non_wrap = false;
        for &c in &v {
            if c != WRAP {
                elems.push(c);
                if !first_non_wrap {
                    first_non_wrap = true;
                }
            }
            if first_non_wrap && c == WRAP {
                break;
            }
        }
        Self {
            offset: (&v).iter().position(|&c| c != WRAP).unwrap(),
            elems: elems,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    walk: Option<usize>,
    turn: Option<char>,
}

impl Instruction {
    pub fn parse(line: &str) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut s = String::from("");
        for c in line.chars() {
            match c {
                LEFT | RIGHT => {
                    if s.len() > 0 {
                        instructions.push(Instruction {
                            walk: Some(s.parse::<usize>().unwrap()),
                            turn: None,
                        });
                        s = String::from("");
                    }
                    instructions.push(Instruction {
                        walk: None,
                        turn: Some(c),
                    });
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => s.push(c),
                _ => (),
            }
        }
        if s.len() > 0 {
            instructions.push(Instruction {
                walk: Some(s.parse::<usize>().unwrap()),
                turn: None,
            });
        }

        return instructions;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction {
    dir: char,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct State {
    loc: Point,
    dir: Direction,
}

impl State {
    pub fn new(p: Point) -> Self {
        return Self {
            loc: p,
            dir: Direction::new(),
        };
    }

    fn update(&mut self, instruction: Instruction, rows: &Vec<Slice>, cols: &Vec<Slice>) {
        match (instruction.turn, instruction.walk) {
            (Some(c), None) => self.dir.turn(c),
            (None, Some(w)) => {
                let slice: &Slice;
                let mut curr: usize;
                let positive = self.dir.dir == DOWN || self.dir.dir == RIGHT;
                match self.dir.dir {
                    UP | DOWN => {
                        slice = &cols[self.loc.x];
                        curr = self.loc.y - slice.offset;
                    }
                    LEFT | RIGHT => {
                        slice = &rows[self.loc.y];
                        curr = self.loc.x - slice.offset;
                    }
                    _ => {
                        assert!(false);
                        slice = &rows[self.loc.y];
                        curr = 0;
                    }
                }

                for _ in 0..w {
                    let mut next: usize = curr + 1;
                    if positive {
                        if curr == slice.elems.len() - 1 {
                            next = 0;
                        }
                    } else {
                        if curr == 0 {
                            next = slice.elems.len() - 1;
                        } else {
                            next = curr - 1;
                        }
                    }

                    if slice.elems[next] == WALL {
                        break;
                    } else if slice.elems[next] == OK {
                        curr = next;
                    }
                }

                match self.dir.dir {
                    UP | DOWN => self.loc = Point::new(self.loc.x, curr + slice.offset),
                    LEFT | RIGHT => self.loc = Point::new(curr + slice.offset, self.loc.y),
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }
}

impl Direction {
    pub fn new() -> Self {
        return Self { dir: RIGHT };
    }
    fn turn(&mut self, dir: char) {
        self.dir = match dir {
            RIGHT => match self.dir {
                LEFT => UP,
                UP => RIGHT,
                RIGHT => DOWN,
                DOWN => LEFT,
                _ => {
                    assert!(false);
                    LEFT
                }
            },
            LEFT => match self.dir {
                LEFT => DOWN,
                UP => LEFT,
                RIGHT => UP,
                DOWN => RIGHT,
                _ => {
                    assert!(false);
                    LEFT
                }
            },
            _ => {
                assert!(false);
                LEFT
            }
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let mut input = Vec::from([
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ])
        .iter()
        .map(|&x| Cube::parse(x))
        .collect::<Vec<Cube>>();

        {
            let exposed = count_exposed(&mut input);
            println!("{:?}", input);
            assert_eq!(64, exposed, "Count exposed");
        }
    }
}
