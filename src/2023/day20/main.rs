use core::fmt;
use std::collections::HashMap;

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
    let modules = parse_lines(&lines);
    println!("Part 1: {}", handle_pt1(&modules));
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
                ModuleType::Broadcaster => "broadcast",
                ModuleType::Conjunction => "&",
            },
        )
    }
}

pub struct Module {
    module_type: ModuleType,
    dsts: Vec<String>,
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

fn handle_pt1(modules: &HashMap<String, Module>) -> i64 {
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

    //  total
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let tests = [(
            vec![
                "broadcaster -> a, b, c",
                "%a -> b",
                "%b -> c",
                "%c -> inv",
                "&inv -> a",
            ],
            32000000,
        )];

        for (input, want) in tests {
            let lines = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            let modules = parse_lines(&lines);
            assert_eq!(
                handle_pt1(&modules),
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
