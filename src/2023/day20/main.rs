use core::fmt;
use std::{
    borrow::Borrow,
    collections::{HashMap, VecDeque},
};

use advent_of_code::utils::input::read_lines;
use clap::Parser;
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

    let mut start = Instant::now();
    println!("Part 1: {}", handle_pt1(&mut parse_lines(&lines)));
    println!("Elapsed: {:.2?}", start.elapsed());

    start = Instant::now();
    println!("Part 2: {}", handle_pt2(&mut parse_lines(&lines)));
    println!("Elapsed: {:.2?}", start.elapsed());
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum ModuleType {
    FlipFlop = 1,
    Conjunction = 2,
    Broadcaster = 3,
}

impl fmt::Display for ModuleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ModuleType::FlipFlop => "%",
                ModuleType::Broadcaster => "",
                ModuleType::Conjunction => "&",
            },
        )
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Pulse {
    LO = 0,
    HI = 1,
}

impl fmt::Display for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "-{}->",
            match self {
                Pulse::HI => "high",
                Pulse::LO => "low",
            },
        )
    }
}

pub struct Module {
    name: String,
    module_type: ModuleType,
    dsts: Vec<String>,
    on: bool,
    // TODO FOR EACH -- this should be a HashMap that default to LO first
    memory: HashMap<String, Pulse>,
}

impl Module {
    fn process(&mut self, from: &str, recv: Pulse) -> VecDeque<(Pulse, String)> {
        match (self.module_type, recv) {
            (ModuleType::Broadcaster, _) => {
                return self
                    .dsts
                    .iter()
                    .map(|d| (recv, d.clone()))
                    .collect::<VecDeque<(Pulse, String)>>();
            }
            (ModuleType::FlipFlop, Pulse::LO) => {
                self.on = !self.on;

                let mut send = Pulse::LO;
                if self.on {
                    send = Pulse::HI;
                }
                return self
                    .dsts
                    .iter()
                    .map(|d| (send, d.clone()))
                    .collect::<VecDeque<(Pulse, String)>>();
            }
            (ModuleType::FlipFlop, Pulse::HI) => {}
            (ModuleType::Conjunction, _) => {
                self.memory
                    .entry(from.to_string())
                    .and_modify(|e| *e = recv);

                let send: Pulse = match self.memory.iter().all(|(_, p)| *p == Pulse::HI) {
                    true => Pulse::LO,
                    false => Pulse::HI,
                };

                return self
                    .dsts
                    .iter()
                    .map(|d| (send, d.clone()))
                    .collect::<VecDeque<(Pulse, String)>>();
            }
        }

        vec![].into()
    }

    fn original(&self) -> bool {
        match self.module_type {
            // ModuleType::Conjunction => self.memory.iter().all(|(_, p)| *p == Pulse::LO),
            ModuleType::FlipFlop => {
                if self.on {
                    return false;
                } else {
                    return true;
                }
            }
            _ => true,
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.module_type, self.dsts.join(", "))
    }
}

fn parse_line(line: &str) -> (String, Module) {
    let mut module_type = ModuleType::Broadcaster;
    let mut find_name = true;
    let mut chars = line.chars();

    match chars.nth(0).unwrap() {
        '%' => module_type = ModuleType::FlipFlop,
        '&' => module_type = ModuleType::Conjunction,
        _ => {
            find_name = false;
            chars.nth("roadcaster -> ".len() - 1);
        }
    };

    let mut name = "broadcaster".to_string();
    let mut name_builder: Vec<char> = Vec::new();
    if find_name {
        while let Some(c) = chars.nth(0) {
            if c.is_whitespace() {
                break;
            }

            name_builder.push(c);
        }
        name = name_builder.iter().collect::<String>();
        chars.nth("-> ".len() - 1);
    }

    (
        name.clone(),
        Module {
            name: name.clone(),
            module_type: module_type,
            dsts: chars
                .collect::<String>()
                .split(", ")
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            memory: HashMap::new(),
            on: false,
        },
    )
}

fn parse_lines(lines: &Vec<String>) -> HashMap<String, Module> {
    let start = Instant::now();
    let mut modules = HashMap::new();
    for i in 0..lines.len() {
        let (name, module) = parse_line(&lines[i]);
        modules.insert(name, module);
    }

    let mut keys = Vec::new();
    for k in modules.borrow().keys() {
        keys.push(k.clone());
    }

    for name in keys {
        for dst in modules.get(&name).unwrap().dsts.clone() {
            modules.entry(dst.clone()).and_modify(|e| {
                e.memory.insert(name.to_string(), Pulse::LO);
            });
        }
    }

    println!("Parse Lines: elapsed: {:.2?}", start.elapsed());
    modules
}

fn handle_pt1(modules: &mut HashMap<String, Module>) -> i64 {
    let mut counts = HashMap::new();
    counts.insert(Pulse::LO, 0);
    counts.insert(Pulse::HI, 0);

    let mut button = 1;

    let total = 1000;
    while button <= total {
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
        queue.push_back((
            format!("button {}", button).to_string(),
            Pulse::LO,
            "broadcaster".to_string(),
        ));
        while let Some((from, pulse, curr)) = queue.pop_front() {
            counts.entry(pulse).and_modify(|e| *e += 1);
            modules.entry(curr.clone()).and_modify(|e| {
                for (send, dst) in e.process(&from, pulse) {
                    queue.push_back((curr.clone(), send, dst.clone()));
                }
            });
        }

        if modules.iter().all(|(_, m)| m.original()) {
            break;
        }

        button += 1;
    }

    let mut lo = *counts.get(&Pulse::LO).unwrap();
    let mut hi = *counts.get(&Pulse::HI).unwrap();
    let multiplier = total / button;
    if button <= total {
        lo *= multiplier;
        hi *= multiplier;
    }

    hi * lo
}

fn handle_pt2(modules: &mut HashMap<String, Module>) -> i64 {
    let mut counts = HashMap::new();
    counts.insert(Pulse::LO, 0);
    counts.insert(Pulse::HI, 0);

    let mut button = 1;
    loop {
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
        queue.push_back((
            format!("button {}", button).to_string(),
            Pulse::LO,
            "broadcaster".to_string(),
        ));
        while let Some((from, pulse, curr)) = queue.pop_front() {
            if pulse == Pulse::LO && curr == "rx" {
                return button;
            }

            counts.entry(pulse).and_modify(|e| *e += 1);
            modules.entry(curr.clone()).and_modify(|e| {
                for (send, dst) in e.process(&from, pulse) {
                    queue.push_back((curr.clone(), send, dst.clone()));
                }
            });
        }

        button += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let tests = [
            (
                vec![
                    "broadcaster -> a, b, c",
                    "%a -> b",
                    "%b -> c",
                    "%c -> inv",
                    "&inv -> a",
                ],
                32000000,
            ),
            (
                vec![
                    "broadcaster -> a",
                    "%a -> inv, con",
                    "&inv -> b",
                    "%b -> con",
                    "&con -> output",
                ],
                11687500,
            ),
        ];

        for (input, want) in tests {
            let lines = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            let mut modules = parse_lines(&lines);
            assert_eq!(
                handle_pt1(&mut modules),
                want,
                "with input\n{}",
                input.join("\n")
            );
        }
    }
}
