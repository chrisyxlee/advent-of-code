use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day19/input.txt";
    println!("In file {}", file_path);

    {
        let lines = read_lines(file_path);
        part_1(&lines);
        part_2(&lines);
    }
}

fn part_1(lines: &Vec<String>) {
    let qualities = lines
        .iter()
        .map(|line| Blueprint::parse(line))
        .map(|bp| {
            let max_geodes = bp.max_geodes(24);
            println!("{:?} has max {} = {}", bp, max_geodes, bp.id * max_geodes);
            bp.id * max_geodes
        })
        .sum::<usize>();

    // 1936 is too low.
    // 2068 is too low.
    // 2160
    println!("Part 1: {:?}", qualities);
}

fn part_2(lines: &Vec<String>) {
    let qualities = lines[0..3]
        .iter()
        .map(|line| Blueprint::parse(line))
        .map(|bp| {
            let max_geodes = bp.max_geodes(32);
            println!("{:?} has max {} = {}", bp, max_geodes, bp.id * max_geodes);
            max_geodes
        })
        .reduce(|acc, x| acc * x)
        .unwrap();

    // 80040 is too high
    // 13340
    println!("Part 2: {:?}", qualities)
}

type Rock = usize;
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const ROCKS: [usize; 4] = [ORE, CLAY, OBSIDIAN, GEODE];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct State {
    rocks: [usize; 4],
    robots: [usize; 4],
    minute: usize,
    max_minutes: usize,
}

impl State {
    pub fn new(bp: &Blueprint, max_minutes: usize) -> Self {
        let lowest = bp.lowest_ore_cost();
        Self {
            rocks: [lowest, 0, 0, 0],
            robots: [1, 0, 0, 0],
            minute: lowest,
            max_minutes: max_minutes,
        }
    }

    fn update(&mut self) -> bool {
        if self.minute == self.max_minutes {
            return false;
        }

        for (r, num_robots) in self.robots.iter().enumerate() {
            self.rocks[r] += num_robots;
        }
        self.minute += 1;
        return true;
    }

    fn prev(&self) -> Option<State> {
        let mut state = self.clone();
        if ROCKS
            .iter()
            .map(|&r| state.rocks[r] >= state.robots[r])
            .all(|b| b)
        {
            state.rocks[ORE] -= state.robots[ORE];
            state.rocks[CLAY] -= state.robots[CLAY];
            state.rocks[OBSIDIAN] -= state.robots[OBSIDIAN];
            state.rocks[GEODE] -= state.robots[GEODE];
            return Some(state);
        }
        return None;
    }

    fn can_buy(&self, r: Rock, bp: &Blueprint) -> bool {
        match r {
            ORE => self.rocks[ORE] >= bp.ore_cost.ores,
            CLAY => self.rocks[ORE] >= bp.clay_cost.ores,
            OBSIDIAN => {
                self.rocks[ORE] >= bp.obsidian_cost.ores
                    && self.rocks[CLAY] >= bp.obsidian_cost.clays
            }
            GEODE => {
                self.rocks[ORE] >= bp.geode_cost.ores
                    && self.rocks[OBSIDIAN] >= bp.geode_cost.obsidians
            }
            _ => {
                assert!(false, "Invalid rock: {}", r);
                false
            }
        }
    }

    fn should_buy(&self, r: Rock, bp: &Blueprint) -> bool {
        if let Some(prev) = self.prev() {
            if prev.can_buy(r, bp) {
                return false;
            }
        }

        return self.can_buy(r, bp) && (r == GEODE || self.robots[r] < bp.highest_cost(r));
    }

    fn buy(&self, r: Rock, bp: &Blueprint) -> Option<State> {
        let mut state = self.clone();
        if !state.update() {
            return None;
        }
        match r {
            ORE => state.rocks[ORE] -= bp.ore_cost.ores,
            CLAY => state.rocks[ORE] -= bp.clay_cost.ores,
            OBSIDIAN => {
                state.rocks[ORE] -= bp.obsidian_cost.ores;
                state.rocks[CLAY] -= bp.obsidian_cost.clays
            }
            GEODE => {
                state.rocks[ORE] -= bp.geode_cost.ores;
                state.rocks[OBSIDIAN] -= bp.geode_cost.obsidians
            }
            _ => assert!(false, "Invalid rock: {}", r),
        }
        state.robots[r] += 1;
        return Some(state);
    }

    fn theoretical_geode_count(&self) -> usize {
        let min_left = self.max_minutes - self.minute;
        let steady = self.rocks[GEODE] + (self.robots[GEODE] * min_left);
        if min_left == 0 {
            return steady;
        }

        return steady + ((min_left * (min_left - 1)) / 2);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Blueprint {
    id: usize,
    ore_cost: OreCost,
    clay_cost: ClayCost,
    obsidian_cost: ObsidianCost,
    geode_cost: GeodeCost,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ObsidianCost {
    ores: usize,
    clays: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GeodeCost {
    ores: usize,
    obsidians: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OreCost {
    ores: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClayCost {
    ores: usize,
}

impl Blueprint {
    pub fn parse(s: &str) -> Self {
        let blueprint_re= Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let mut id: usize = 0;
        let mut ore_cost = 0;
        let mut clay_cost = 0;
        let mut obsidian_cost_ore = 0;
        let mut obsidian_cost_clay = 0;
        let mut geode_cost_ore = 0;
        let mut geode_cost_obsidian = 0;

        assert!(blueprint_re.is_match(s));
        for m in blueprint_re.captures_iter(s) {
            for (i, capt) in m.iter().enumerate() {
                if let Some(sub) = capt {
                    if i == 0 {
                        continue;
                    }

                    let v = str2usize(sub.as_str());
                    match i {
                        1 => id = v,
                        2 => ore_cost = v,
                        3 => clay_cost = v,
                        4 => obsidian_cost_ore = v,
                        5 => obsidian_cost_clay = v,
                        6 => geode_cost_ore = v,
                        7 => geode_cost_obsidian = v,
                        _ => (),
                    }
                }
            }
        }
        return Self {
            id: id,
            ore_cost: OreCost { ores: ore_cost },
            clay_cost: ClayCost { ores: clay_cost },
            obsidian_cost: ObsidianCost {
                ores: obsidian_cost_ore,
                clays: obsidian_cost_clay,
            },
            geode_cost: GeodeCost {
                ores: geode_cost_ore,
                obsidians: geode_cost_obsidian,
            },
        };
    }

    fn lowest_ore_cost(&self) -> usize {
        *[self.ore_cost.ores, self.clay_cost.ores]
            .iter()
            .min()
            .unwrap()
    }

    fn highest_cost(&self, r: Rock) -> usize {
        match r {
            ORE => *[
                self.ore_cost.ores,
                self.clay_cost.ores,
                self.obsidian_cost.ores,
                self.geode_cost.ores,
            ]
            .iter()
            .max()
            .unwrap(),
            CLAY => self.obsidian_cost.clays,
            OBSIDIAN => self.geode_cost.obsidians,
            GEODE => 0,
            _ => {
                assert!(false, "Invalid rock: {}", r);
                0
            }
        }
    }

    fn max_geodes(&self, max_minutes: usize) -> usize {
        let mut geode_count = 0;
        let mut states: Vec<State> = Vec::from([State::new(self, max_minutes)]);
        while states.len() > 0 {
            let mut curr = states.pop().unwrap();
            // println!("{:?} | max = {}", curr, geode_count);
            if curr.theoretical_geode_count() <= geode_count {
                continue;
            }

            let should_buy = ROCKS
                .iter()
                .map(|&r| curr.should_buy(r, self))
                .collect::<Vec<bool>>();
            let bought = should_buy
                .iter()
                .enumerate()
                .filter(|&(_, should_buy)| *should_buy)
                .map(|(r, _)| curr.buy(r, self))
                .filter(|&s| s.is_some())
                .map(|s| s.unwrap())
                .rev()
                .collect::<Vec<State>>();
            states.extend(bought.iter());

            if curr.rocks[GEODE] > geode_count {
                geode_count = curr.rocks[GEODE];
            }

            // We should buy an ORE robot as soon as we can when we have no other robots..
            if curr.rocks[ORE] > self.highest_cost(ORE)
                && (curr.robots[CLAY] == 0 && curr.robots[OBSIDIAN] == 0 && curr.robots[GEODE] == 0)
            {
                continue;
            }

            if !curr.update() {
                continue;
            }

            //	If we can buy each type of robot, then we should buy at least one.
            if should_buy.iter().all(|&x| x) {
                continue;
            }

            // Push the state where we don't buy anything.
            states.push(curr);
        }
        return geode_count;
    }
}

fn str2usize(s: &str) -> usize {
    return s.parse::<usize>().unwrap();
}

//impl fmt::Display for Blueprint {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "({},{},{})", self.x, self.y, self.z)
//    }
//}

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
        let tests = Vec::from([
("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.", 9),
("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.", 12),
//("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 7 obsidian.", 8),
//("Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 18 obsidian.", 0),
//("Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 12 obsidian.", 2),
]);

        for (input, want) in tests {
            let bp = Blueprint::parse(input);
            let got = bp.max_geodes();
            println!("BP {:?} = {}", bp, got);
            assert_eq!(want, got, "\n{:?}\nWant {}, but got {}", bp, want, got);
        }
    }
}
