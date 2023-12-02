use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day18/input.txt";
    println!("In file {}", file_path);

    {
        let mut cubes = read_lines(file_path)
            .iter()
            .map(|s| Cube::parse(s))
            .collect::<Vec<Cube>>();
        let exposed = count_exposed(&mut cubes);
        println!("Part 1: {}", exposed);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
    exposed: [bool; 6],
}

const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;
const FRONT: usize = 4;
const BACK: usize = 5;

impl Cube {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            exposed: [true; 6],
        }
    }

    pub fn parse(s: &str) -> Self {
        let parts = s.split(",").collect::<Vec<&str>>();
        Self::new(s2i(parts[0]), s2i(parts[1]), s2i(parts[2]))
    }

    fn check(&mut self, other: &mut Cube) {
        let xd = (self.x - other.x).abs();
        let yd = (self.y - other.y).abs();
        let zd = (self.z - other.z).abs();
        if xd <= 1 && yd <= 1 && zd <= 1 && (xd == 0 || yd == 0 || zd == 0) {
            match (xd == 0, yd == 0, zd == 0) {
                (true, true, false) => {
                    if self.z > other.z {
                        self.exposed[BOTTOM] = false;
                        other.exposed[TOP] = false;
                    } else {
                        self.exposed[TOP] = false;
                        other.exposed[BOTTOM] = false;
                    }
                }
                (true, false, true) => {
                    if self.y > other.y {
                        self.exposed[BACK] = false;
                        other.exposed[FRONT] = false;
                    } else {
                        self.exposed[FRONT] = false;
                        other.exposed[BACK] = false;
                    }
                }
                (false, true, true) => {
                    if self.x > other.x {
                        self.exposed[LEFT] = false;
                        other.exposed[RIGHT] = false;
                    } else {
                        self.exposed[RIGHT] = false;
                        other.exposed[LEFT] = false;
                    }
                }
                _ => (),
            }
        }
    }

    fn exposed_faces(&self) -> usize {
        return self
            .exposed
            .iter()
            .fold(0, |acc, &x| if x { acc + 1 } else { acc });
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn count_exposed(cubes: &mut Vec<Cube>) -> usize {
    let mut xps: Vec<usize> = cubes
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let mut yps = xps.clone();
    let mut zps = xps.clone();

    xps.sort_by(|&a, &b| cubes[a].x.cmp(&cubes[b].x));
    yps.sort_by(|&a, &b| cubes[a].y.cmp(&cubes[b].y));
    zps.sort_by(|&a, &b| cubes[a].z.cmp(&cubes[b].z));

    for a in 0..xps.len() {
        for b in a + 1..xps.len() {
            if (cubes[xps[a]].x - cubes[xps[b]].x).abs() > 1 {
                break;
            }

            let mut a_cube = cubes[xps[a]];
            let mut b_cube = cubes[xps[b]];
            a_cube.check(&mut b_cube);
            cubes[xps[a]] = a_cube;
            cubes[xps[b]] = b_cube;
        }
    }

    for a in 0..yps.len() {
        for b in a + 1..yps.len() {
            if (cubes[yps[a]].y - cubes[yps[b]].y).abs() > 1 {
                break;
            }

            let mut a_cube = cubes[yps[a]];
            let mut b_cube = cubes[yps[b]];
            a_cube.check(&mut b_cube);
            cubes[yps[a]] = a_cube;
            cubes[yps[b]] = b_cube;
        }
    }

    for a in 0..zps.len() {
        for b in a + 1..zps.len() {
            if (cubes[zps[a]].z - cubes[zps[b]].z).abs() > 1 {
                break;
            }

            let mut a_cube = cubes[zps[a]];
            let mut b_cube = cubes[zps[b]];
            a_cube.check(&mut b_cube);
            cubes[zps[a]] = a_cube;
            cubes[zps[b]] = b_cube;
        }
    }

    return cubes.iter().map(|&c| c.exposed_faces()).sum();
}

fn s2i(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect::<Vec<String>>();
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
