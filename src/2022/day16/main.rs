use advent_of_code::utils::input::read_lines;
use clap::Parser;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
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
   Time:        41     96     88     94
Distance:   214   1789   1127   1055


    {
        // 1180 is too low.
        // 1250 is too low.
        // 1600 is too low.
        println!("Part 1: {}", solve_pt1(&lines));
    }
}
const MINUTES: usize = 30;
const START_VALVE: &str = "AA";

fn solve_pt1(lines: &Vec<String>) -> usize {
    let mut valves: HashMap<String, RefCell<Valve>> = HashMap::new();
    for line in lines {
        let (name, valve) = Valve::parse(&line);
        valves.entry(name).or_insert(valve);
    }

    let dists = create_dist_grid(&valves);

    for key in valves.keys() {
        println!("{}: {}", key, valves[key].borrow());
    }

    let mut closed: HashSet<String> = HashSet::new();
    for (name, valve) in &valves {
        let valve = valve.borrow();
        if valve.flow_rate > 0 {
            closed.insert(name.clone());
        }
    }

    return dfs(&mut valves, &mut closed, &dists);
}

#[derive(Debug, Clone)]
pub struct Valve {
    flow_rate: usize,
    leads_to: Vec<String>,
}

impl Valve {
    pub fn new(flow_rate: usize, leads_to: Vec<String>) -> Self {
        Self {
            flow_rate: flow_rate,
            leads_to: leads_to,
        }
    }

    pub fn parse(s: &str) -> (String, RefCell<Self>) {
        let valve_re = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)",
        )
        .unwrap();
        let mut name = String::from("");
        let mut leads_to: Vec<String> = Vec::new();
        let mut flow_rate = 0;
        assert!(valve_re.is_match(s), "Improper input? {}", s);
        for m in valve_re.captures_iter(s) {
            for (i, capt) in m.iter().enumerate() {
                if let Some(sub) = capt {
                    match i {
                        1 => name = String::from(sub.as_str()),
                        2 => flow_rate = sub.as_str().parse::<usize>().unwrap(),
                        3 => {
                            leads_to = sub
                                .as_str()
                                .split(", ")
                                .map(|v| String::from(v))
                                .collect::<Vec<String>>();
                        }
                        _ => (),
                    }
                }
            }
        }
        return (name, RefCell::new(Valve::new(flow_rate, leads_to)));
    }
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} flow_rate to {:?}", self.flow_rate, self.leads_to)
    }
}

#[derive(Debug, Clone)]
pub struct State {
    closed: HashSet<String>,
    opened: Vec<(String, usize)>,
    loc: String,
    flow: usize,
    minute: usize,
    max_minutes: usize,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "State @ {} (elapsed = {}m) (flow = {})\n  - Opened: {:?}\n  - Closed {:?}",
            self.loc, self.minute, self.flow, self.opened, self.closed,
        )
    }
}

fn dist(valves: &HashMap<String, RefCell<Valve>>, start: &str, end: &str) -> usize {
    if start == end {
        return 0;
    }
    let mut visited: HashSet<String> = HashSet::new();
    let mut to_visit: Vec<(String, usize)> = Vec::new();
    to_visit.push((String::from(start), 0));
    while to_visit.len() > 0 {
        let (curr, partial_distance) = to_visit.pop().unwrap();
        visited.insert(curr.clone());
        for neighbor in &valves[&curr].borrow().leads_to {
            if neighbor == end {
                return partial_distance + 1;
            }
            if !visited.contains(neighbor) {
                visited.insert(neighbor.clone());
                to_visit.push((String::from(neighbor), partial_distance + 1));
            }
        }
    }

    assert!(false);
    return 0;
}

fn get_dist(dists: &HashMap<String, usize>, start: &str, end: &str) -> usize {
    if start == end {
        return 0;
    }
    let key = match start.cmp(end) {
        Ordering::Less | Ordering::Equal => format!("{}:{}", start, end),
        Ordering::Greater => format!("{}:{}", end, start),
    };
    return dists[&key];
}

fn create_dist_grid(valves: &HashMap<String, RefCell<Valve>>) -> HashMap<String, usize> {
    let mut dist_grid: HashMap<String, usize> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    for k in valves.keys() {
        keys.push(k.clone());
    }
    keys.sort();

    for i in 0..keys.len() {
        for j in i..keys.len() {
            if (&keys[i] == START_VALVE || valves[&keys[i]].borrow().flow_rate > 0)
                && valves[&keys[j]].borrow().flow_rate > 0
            {
                let distance = dist(valves, &keys[i], &keys[j]);
                *dist_grid
                    .entry(format!("{}:{}", keys[i], keys[j]))
                    .or_insert(distance) = distance;
            }
        }
    }

    let mut keys: Vec<String> = Vec::new();
    for k in dist_grid.keys() {
        keys.push(k.clone());
    }
    keys.sort();
    println!("Distances:");
    for k in keys {
        let parts = k.split(":").collect::<Vec<&str>>();
        let flow_rate = valves[parts[1]].borrow().flow_rate;
        let mut potential = 0;
        if dist_grid[&k] <= MINUTES {
            potential = (MINUTES - dist_grid[&k]) * flow_rate;
        }
        println!(
            " - {} -> {} (flow={}): {} --> {}",
            parts[0], parts[1], flow_rate, dist_grid[&k], potential,
        );
    }
    return dist_grid;
}

impl State {
    fn next(
        &mut self,
        valves: &HashMap<String, RefCell<Valve>>,
        dists: &HashMap<String, usize>,
        max_flow: usize,
    ) -> Vec<State> {
        if self.minute >= self.max_minutes {
            return Vec::new();
        }

        if self.closed.len() == 0 {
            return Vec::new();
        }

        let mut potential_max_flow = self.flow;
        let mut mins_left = self.max_minutes - self.minute;
        for closed in &self.closed {
            let dist = get_dist(dists, self.loc.as_str(), closed.as_str());
            if dist < mins_left {
                potential_max_flow += valves[closed].borrow().flow_rate * (mins_left - dist);
            }
        }
        if potential_max_flow <= max_flow {
            println!(
                "Potential is {}, which is less than max {}",
                potential_max_flow, max_flow
            );
            return Vec::new();
        }

        let mut res: Vec<State> = Vec::new();
        // Then we move onto the next valve.
        for closed_valve in &self.closed {
            if closed_valve == &self.loc {
                continue;
            }

            let mut next = self.clone();
            next.loc = closed_valve.clone();
            let distance = get_dist(dists, &self.loc, closed_valve);
            if distance > mins_left {
                continue;
            }
            next.minute += distance;

            // Open this valve if we can.
            if next.closed.contains(&next.loc) && next.minute < next.max_minutes {
                next.minute += 1;
                mins_left = next.max_minutes - next.minute;
                next.flow += valves[&next.loc].borrow().flow_rate * mins_left;
                next.closed.remove(&next.loc);
                next.opened.push((
                    format!(
                        "{} ({})",
                        next.loc.clone(),
                        valves[&next.loc].borrow().flow_rate
                    ),
                    next.minute,
                ));
                println!("open {}", next.loc);
            }
            res.push(next);
        }

        res.sort_by(|a, b| {
            let by_flow = a.flow.cmp(&b.flow);
            if by_flow != Ordering::Equal {
                return by_flow;
            }
            let by_dist = b.minute.cmp(&a.minute);
            return by_dist;
        });
        println!("......res");
        for s in &res {
            println!("...{}", s);
        }

        return res;
    }
}

fn dfs(
    valves: &HashMap<String, RefCell<Valve>>,
    closed: &mut HashSet<String>,
    dists: &HashMap<String, usize>,
) -> usize {
    let mut potentials: Vec<State> = Vec::new();
    potentials.push(State {
        closed: closed.clone(),
        opened: Vec::new(),
        loc: String::from(START_VALVE),
        flow: 0,
        minute: 0,
        max_minutes: MINUTES,
    });

    let mut max_flow: usize = 0;
    let mut curr: State;
    while potentials.len() > 0 {
        curr = potentials.pop().unwrap();
        println!("MAX IS {}\n{}", max_flow, curr);
        let next = curr.next(valves, dists, max_flow);
        for n in &next {
            if n.flow > max_flow {
                max_flow = n.flow;
            }
            potentials.push(n.clone());
        }
    }

    return max_flow;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let mut input = Vec::from([
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ])
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();

        assert_eq!(1651, solve_pt1(&input));
    }

    #[test]
    fn dist_works() {
        let lines = Vec::from([
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ])
        .iter()
        .map(|&s| String::from(s))
        .collect::<Vec<String>>();
        let mut valves: HashMap<String, RefCell<Valve>> = HashMap::new();
        for line in &lines {
            let (name, valve) = Valve::parse(&line);
            valves.entry(name).or_insert(valve);
        }

        assert_eq!(7, dist(&valves, "JJ", "HH"));
        assert_eq!(1, dist(&valves, "AA", "DD"));
    }
}
