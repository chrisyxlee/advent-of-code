use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day16/input.txt";
    println!("In file {}", file_path);

    {
        println!("Part 1: {}", solve_pt1(read_lines(file_path)));
    }
}
const MINUTES: usize = 30;
const START_VALVE: &str = "AA";

fn solve_pt1(lines: Vec<String>) -> usize {
    let mut valves: HashMap<String, RefCell<Valve>> = HashMap::new();
    for line in lines {
        let (name, valve) = Valve::parse(&line);
        valves.entry(name).or_insert(valve);
    }

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

    return dfs(&mut valves, &mut closed, START_VALVE, MINUTES);
}

#[derive(Debug, Clone, Hash)]
pub struct NodeKey {
    name: String,
    minute: usize,
    opened: bool,
}

impl NodeKey {
    pub fn new(name: &str, minute: usize, opened: bool) -> Self {
        Self {
            name: String::from(name),
            minute: minute,
            opened: opened,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    added_flow: usize,
    leads_to: Vec<NodeKey>,
}

fn build_graph_rec(
    valves: &HashMap<String, RefCell<Valve>>,
    m: &mut HashMap<NodeKey, RefCell<Node>>,
    closed: &mut HashSet<String>,
    name: &str,
    minute: usize,
) -> Vec<NodeKey> {
    if minute == 0 {
        return Vec::new();
    }

    if closed.len() == 0 {
        return Vec::new();
    }

    let mut res: Vec<NodeKey> = Vec::new();
    let curr = valves[name].borrow();
    if minute > 0 && closed.contains(name) && curr.flow_rate > 0 {
        let key = NodeKey::new(name, minute, true);
        let curr_rate = curr.flow_rate * (minute - 1);
        let mut without_open = closed.clone();
        without_open.remove(name);
        if minute > 1 {
            for neighbor in &curr.leads_to {
                for edge in build_graph_rec(valves, m, &mut closed.clone(), neighbor, minute - 2) {}
            }
        } else {
            return res;
        }
    }

    for neighbor in &curr.leads_to {}

    return res;
}

fn build_graph(valves: &HashMap<String, RefCell<Valve>>) -> HashMap<NodeKey, RefCell<Node>> {
    let mut opened: HashSet<String> = HashSet::new();
    let mut m: HashMap<NodeKey, RefCell<Node>> = HashMap::new();
    let mut closed: HashSet<String> = HashSet::new();
    for (name, valve) in valves {
        let valve = valve.borrow();
        if valve.flow_rate > 0 {
            closed.insert(name.clone());
        }
    }

    build_graph_rec(valves, &mut m, &mut closed, START_VALVE, MINUTES);
    return m;
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

fn dfs(
    valves: &HashMap<String, RefCell<Valve>>,
    closed: &mut HashSet<String>,
    name: &str,
    minutes: usize,
) -> usize {
    if minutes == 0 {
        return 0;
    }

    if closed.len() == 0 {
        return 0;
    }

    let can_open = minutes > 0 && closed.contains(name) && valves[name].borrow().flow_rate > 0;
    let mut potentials: Vec<usize> = Vec::new();
    if can_open {
        let flow_rate = valves[name].borrow().flow_rate;
        let curr_rate = flow_rate * (minutes - 1);
        let mut without_open = closed.clone();
        without_open.remove(name);
        if minutes > 1 {
            for neighbor in &valves[name].borrow().leads_to {
                potentials.push(curr_rate + dfs(valves, &mut without_open, &neighbor, minutes - 2));
            }
        } else {
            return curr_rate;
        }
    }

    for neighbor in &valves[name].borrow().leads_to {
        potentials.push(dfs(valves, &mut closed.clone(), &neighbor, minutes - 1));
    }

    let max_flow = *potentials.iter().max().unwrap();
    println!(
        "Valve {} can maximally release {} with {} minutes left | {:?}",
        name, max_flow, minutes, closed
    );
    //for key in valves.keys() {
    //    println!("{}: {}", key, valves[key].borrow());
    //}

    return max_flow;
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .filter(|x| x.len() > 0)
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        /*
         */
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

        assert_eq!(1651, solve_pt1(input));
    }
}
