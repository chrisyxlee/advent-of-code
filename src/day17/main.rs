use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day17/input.txt";
    println!("In file {}", file_path);

    {
        let dirs = parse_directions(file_path);
        let pt1 = tetris_pt1(&dirs, 2022);
        // 3024 is too low.
        println!("Part 1: {}", pt1);
    }

    //{
    //    let dirs = parse_directions(file_path);
    //    let pt2 = tetris_pt1(&dirs, 1000000000000);
    //    println!("Parse 2: {}", pt2);
    //}
}

const STARTING_LEFT_PADDING: usize = 2;
// 1, 2, 3 are the padding. 0 and negatives are indexing into the actual grid.
const STARTING_BOTTOM_PADDING: i32 = 4;
const GRID_WIDTH: usize = 7;

const GO_LEFT: char = '<';
const GO_RIGHT: char = '>';

const HORIE: char = '_';
const PLUS: char = '+';
const ELL: char = 'L';
const VERTIE: char = '|';
const SQUARE: char = 'S';

const EMPTY: char = '.';
const FILLED: char = '#';
const FALLING: char = '@';

const SHAPES: [char; 5] = [HORIE, PLUS, ELL, VERTIE, SQUARE];

type Row = [char; GRID_WIDTH];
type Grid = Vec<Row>;

fn tetris_pt1(directions: &Vec<char>, num_rocks: usize) -> usize {
    let mut grid = create_grid();
    let mut shape_index: usize = 0;
    let mut dir_index: usize = 0;

    for _i in 0..num_rocks {
        let mut position = get_start(SHAPES[shape_index]);
        while push_and_drop(&mut grid, &mut position, directions[dir_index]) {
            dir_index = (dir_index + 1) % directions.len();
        }
        dir_index = (dir_index + 1) % directions.len();
        shape_index = (shape_index + 1) % SHAPES.len();
    }

    _print_grid(&grid);

    return grid.len();
}

// To make it easier not to mess up x and y.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Padding {
    left: usize,
    bottom: i32,
}

impl Padding {
    pub fn new(left: usize, bottom: i32) -> Self {
        Self {
            left: left,
            bottom: bottom,
        }
    }

    pub fn start() -> Self {
        Self {
            left: STARTING_LEFT_PADDING,
            bottom: STARTING_BOTTOM_PADDING,
        }
    }

    fn left(&self) -> Option<Padding> {
        if self.left > 0 {
            return Some(Padding::new(self.left - 1, self.bottom));
        }
        return None;
    }

    fn right(&self) -> Option<Padding> {
        if self.left < GRID_WIDTH - 1 {
            return Some(Padding::new(self.left + 1, self.bottom));
        }
        return None;
    }

    fn down(&self) -> Padding {
        return Padding::new(self.left, self.bottom - 1);
    }
}

impl fmt::Display for Padding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(l={},b={})", self.left, self.bottom)
    }
}

fn _print_grid_with_falling(grid: &Grid, falling: &Vec<Padding>) {
    let mut copy = grid.clone();
    for p in falling {
        set(&mut copy, *p, grid.len(), FALLING);
    }
    _print_grid(&copy);
}

fn _print_grid(grid: &Grid) {
    println!("| Grid  |");
    for i in (0..grid.len()).rev() {
        println!("|{}|", grid[i].iter().collect::<String>());
    }
}

fn get(grid: &Grid, padding: Padding) -> char {
    if padding.bottom > 0 {
        return EMPTY;
    }

    if (grid.len() as i32 + padding.bottom) == 0 {
        return FILLED;
    }

    return grid[(grid.len() as i32 - 1 + padding.bottom) as usize][padding.left];
}

fn set(grid: &mut Grid, padding: Padding, height: usize, c: char) {
    let y = (height as i32 + (padding.bottom - 1)) as usize;
    if padding.bottom > 0 {
        for _i in grid.len()..=(y) {
            grid.push([EMPTY; GRID_WIDTH]);
        }
    }

    grid[y][padding.left] = c;
}

fn get_start(shape: char) -> Vec<Padding> {
    let start = Padding::start();
    match shape {
        // ####
        HORIE => {
            return Vec::from([
                start,
                Padding::new(start.left + 1, start.bottom),
                Padding::new(start.left + 2, start.bottom),
                Padding::new(start.left + 3, start.bottom),
            ]);
        }
        //  #
        // ###
        // S#
        PLUS => {
            return Vec::from([
                Padding::new(start.left + 1, start.bottom),
                Padding::new(start.left, start.bottom + 1),
                Padding::new(start.left + 1, start.bottom + 1),
                Padding::new(start.left + 2, start.bottom + 1),
                Padding::new(start.left + 1, start.bottom + 2),
            ]);
        }
        //   #
        //   #
        // ###
        ELL => {
            return Vec::from([
                start,
                Padding::new(start.left + 1, start.bottom),
                Padding::new(start.left + 2, start.bottom),
                Padding::new(start.left + 2, start.bottom + 1),
                Padding::new(start.left + 2, start.bottom + 2),
            ])
        }
        // #
        // #
        // #
        // #
        VERTIE => {
            return Vec::from([
                start,
                Padding::new(start.left, start.bottom + 1),
                Padding::new(start.left, start.bottom + 2),
                Padding::new(start.left, start.bottom + 3),
            ]);
        }
        // ##
        // ##
        SQUARE => {
            return Vec::from([
                start,
                Padding::new(start.left, start.bottom + 1),
                Padding::new(start.left + 1, start.bottom),
                Padding::new(start.left + 1, start.bottom + 1),
            ]);
        }
        _ => assert!(false, "wrong shape: {}", shape),
    }

    return Vec::new();
}

fn push_and_drop(grid: &mut Grid, position: &mut Vec<Padding>, dir: char) -> bool {
    //	If any movement would cause any part of the rock to move into the walls,
    // floor, or a stopped rock, the movement instead does not occur.
    if position
        .iter()
        .map(|p| match dir {
            GO_LEFT => p.left(),
            GO_RIGHT => p.right(),
            _ => {
                assert!(false, "wrong direction? {}", dir);
                return None;
            }
        })
        .all(|p| match p {
            None => false,
            Some(ps) => get(grid, ps) == EMPTY,
        })
    {
        for i in 0..position.len() {
            match dir {
                GO_LEFT => position[i] = position[i].left().expect("just checked left"),
                GO_RIGHT => position[i] = position[i].right().expect("just checked right"),
                _ => assert!(false, "what are you doing"),
            }
        }
    }

    // If a downward movement would have caused a falling rock to move into the
    // floor or an already-fallen rock, the falling rock stops where it is
    // (having landed on something).
    let grid_height = grid.len();
    if position
        .iter()
        .map(|&p| p.down())
        .all(|p| get(grid, p) == EMPTY)
    {
        for i in 0..position.len() {
            position[i] = position[i].down();
        }
        return true;
    }

    for i in 0..position.len() {
        set(grid, position[i], grid_height, FILLED);
    }

    return false;
}

fn create_grid() -> Grid {
    return Vec::new();
}

fn parse_directions<P>(filename: P) -> Vec<char>
where
    P: AsRef<Path>,
{
    let lines: Vec<String> = io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect();
    assert!(
        lines.len() == 1,
        "there should only be 1 line: got {}",
        lines.len()
    );

    return lines[0].chars().collect::<Vec<char>>();
}
