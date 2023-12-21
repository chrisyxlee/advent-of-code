use core::fmt;
use std::collections::{vec_deque, HashMap, VecDeque};

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
    let mut modules = parse_lines(&lines);
    println!("Part 1: {}", handle_pt1(&mut modules));
    println!("Elapsed: {:.2?}", start.elapsed());

    //  start = Instant::now();
    //  println!("Part 2: {}", handle_pt2(&checkers));
    //  println!("Elapsed: {:.2?}", start.elapsed());
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

impl Pulse {
    fn opposite(&self) -> Self {
        match self {
            Pulse::HI => Pulse::LO,
            Pulse::LO => Pulse::HI,
        }
    }
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
    module_type: ModuleType,
    dsts: Vec<String>,
    on: bool,
    // TODO FOR EACH -- this should be a HashMap that default to LO first
    memory: Option<Pulse>,
}

impl Module {
    fn process(&mut self, name: &str, pulse: Pulse) -> VecDeque<(Pulse, String)> {
        if name == "b" || name == "con"{
            println!("process {} {}{}", pulse, self.module_type, name);
        }
        match (self.module_type, pulse) {
            (ModuleType::Broadcaster, _) => {
                return self
                    .dsts
                    .iter()
                    .map(|d| (pulse, d.clone()))
                    .collect::<VecDeque<(Pulse, String)>>();
            }
            (ModuleType::FlipFlop, Pulse::LO) => {
                let prev = self.on;
                self.on = !self.on;
                println!("{} is now {}", name, self.on);

                let mut send = Pulse::LO;
                if self.on {
                    send = Pulse::HI;
                }
                //  println!("toggled: now {}, prev {}, so send {}", self.on, prev, send);
                return self
                    .dsts
                    .iter()
                    .map(|d| (send, d.clone()))
                    .collect::<VecDeque<(Pulse, String)>>();
            }
            (ModuleType::FlipFlop, Pulse::HI) => {}
            (ModuleType::Conjunction, _) => {
                // TODO THIS IS WRONG -- depends on the state of the upstream modules lol, not just the pulses
                if self.memory.is_some() {
                    println!("memory = {}, recv = {}", self.memory.unwrap(), pulse);
                } else {
                    println!("memory = none, recv = {}", pulse);
                }
                let mut send: Pulse;
                if let Some(mem) = self.memory {
                    (self.memory, send) = match (mem, pulse) {
                        (_, Pulse::LO) => (Some(Pulse::LO), Pulse::HI),
                        (Pulse::HI, Pulse::HI) => (Some(Pulse::HI), Pulse::LO),
                        (Pulse::LO, Pulse::HI) => (Some(Pulse::LO), Pulse::HI),
                    }
                } else {
                    self.memory = Some(pulse);
                    send = pulse.opposite();
                }

                return self
                    .dsts
                    .iter()
                    .map(|d| (send, d.clone()))
                    .collect::<VecDeque<(Pulse, String)>>();
            }
        }

        vec![].into()
    }

    fn destinations(&self, src: &String) -> VecDeque<(String, String)> {
        return self
            .dsts
            .iter()
            .map(|dst: &String| (src.clone(), dst.clone()))
            .collect::<VecDeque<(String, String)>>();
    }

    fn finalize(&mut self) {
        if self.module_type == ModuleType::Conjunction {
            self.memory = None;
        }
    }

    fn original(&self) -> bool {
        match self.module_type {
            ModuleType::Conjunction => self.memory.is_some() && self.memory.unwrap() == Pulse::LO,
            ModuleType::FlipFlop => !self.on,
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
        name,
        Module {
            module_type: module_type,
            dsts: chars
                .collect::<String>()
                .split(", ")
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            memory: None,
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

    println!("Parse Lines: elapsed: {:.2?}", start.elapsed());
    modules
}

fn handle_pt1(modules: &mut HashMap<String, Module>) -> i64 {
    println!("\n\n\n");
    let mut counts = HashMap::new();
    counts.insert(Pulse::LO, 0);
    counts.insert(Pulse::HI, 0);

    let mut button = 1;

    let total = 1000;
    while button <= total {
        let mut queue: VecDeque<(Pulse, String)> = VecDeque::new();
        queue.push_back((Pulse::LO, "broadcaster".to_string()));
        println!("\n|-> button {}: {} {}", button, Pulse::LO, "broadcaster");
        while let Some((pulse, curr)) = queue.pop_front() {
            counts.entry(pulse).and_modify(|e| *e += 1);
            modules.entry(curr.clone()).and_modify(|e| {
                for (send, dst) in e.process(&curr, pulse) {
                    println!("|-> {} {} {}", curr, send, &dst);
                    queue.push_back((send, dst));
                }
            });
        }

        modules.iter_mut().map(|(_, mut module)| module.finalize());

        if modules.iter().all(|(_, m)| m.original()) {
            println!("cycle detected on button {}", button);
            break;
        }
        if button > 4 {
            break;
        }

        button += 1;
    }

    //  let mut total = 0;
    //  for value in values {
    //      let mut workflow = "in".to_string();
    //      loop {
    //          if workflow == "A" || workflow == "R" {
    //              break;
    //          }

    //          workflow = modules.get(&workflow).unwrap().check(*value);
    //      }

    //      if workflow == "A" {
    //          total += value.accepted();
    //      }
    //  }
    let lo = *counts.get(&Pulse::LO).unwrap();
    let hi = *counts.get(&Pulse::HI).unwrap();
    let multiplier = total / button;
    let new_lo = multiplier * lo;
    let new_hi = multiplier * hi;
    println!(
        "{} low, {} high, repeated on {} button press, so multiply by {} to get {} low, {} high",
        lo, hi, button, multiplier, new_lo, new_hi
    );

    //  total
    new_hi * new_lo
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
            // assert_eq!(
            //     handle_pt2(&checker),
            //     want2,
            //     "with input\n{}",
            //     input.join("\n")
            // );
        }
    }
}
