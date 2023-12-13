use advent_of_code::utils::input::read_lines;
use clap::Parser;
use std::collections::HashMap;

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

    let pt1: usize = handle_pt1(&lines);
    // 7215 too low
    println!("Part 1: {}", pt1);
    let pt2: usize = handle_pt2(&lines);
    // 295687560777 too low
    println!("Part 2: {}", pt2);
}

fn handle_pt1(lines: &Vec<String>) -> usize {
    lines.iter().map(|line| count_possibilities(line)).sum()
}

fn handle_pt2(lines: &Vec<String>) -> usize {
    lines.iter().map(|line| count_5x_possibilities(line)).sum()
}

fn can_break(g: &char) -> bool {
    *g == '#' || *g == '?'
}

fn can_operate(g: &char) -> bool {
    *g == '.' || *g == '?'
}

fn memo_key(gears: &Vec<char>, config: &Vec<usize>) -> String {
    format!(
        "{} {}",
        gears.iter().collect::<String>(),
        config
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
    .to_string()
}

fn try_memo_possibility(
    gears: &Vec<char>,
    config: &Vec<usize>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let key = memo_key(gears, config);
    if let Some(m) = memo.get(&key) {
        return *m;
    }

    let m = try_possibility(gears, config, memo);
    memo.insert(key, m);

    m
}

fn try_possibility(
    gears: &Vec<char>,
    config: &Vec<usize>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if gears.len() == 0 {
        let mut possibilities = 0;
        if config.len() == 0 {
            possibilities = 1;
        }

        return possibilities;
    }

    if config.len() == 0 {
        let mut possibilities = 0;
        if gears.iter().all(|x| can_operate(x)) {
            possibilities = 1;
        }

        return possibilities;
    }

    let min_gears: usize = config.iter().sum::<usize>() + (config.len() - 1);
    if min_gears > gears.len() {
        return 0;
    }

    let c = config.first().unwrap();
    let g = gears.first().unwrap();

    match (*c, *g) {
        (1, '#') => {
            if gears.len() > 1 && can_operate(&gears[1]) {
                let possibilities = try_memo_possibility(
                    &gears[2..gears.len()].to_vec(),
                    &config[1..config.len()].to_vec(),
                    memo,
                );
                return possibilities;
            } else if gears.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        }
        (_, '#') => {
            let mut possibilities = 0;
            if *c <= gears.len()
                && gears[0..*c].iter().all(|x| can_break(x))
                && (*c == gears.len() || can_operate(&gears[*c]))
            {
                possibilities += try_memo_possibility(
                    &gears[*vec![*c + 1, gears.len()].iter().min().unwrap()..gears.len()].to_vec(),
                    &config[1..config.len()].to_vec(),
                    memo,
                );
            }
            return possibilities;
        }
        (_, '.') => return try_memo_possibility(&gears[1..gears.len()].to_vec(), config, memo),
        (1, '?') => {
            let mut broken = 0;
            if gears.len() == 1 || (gears.len() > 1 && can_operate(&gears[1])) {
                broken = try_memo_possibility(
                    &gears[*vec![2 as usize, gears.len()].iter().min().unwrap()..gears.len()]
                        .to_vec(),
                    &config[1..config.len()].to_vec(),
                    memo,
                );
            }
            return try_memo_possibility(&gears[1..gears.len()].to_vec(), config, memo) + broken;
        }
        (_, '?') => {
            let operational = try_memo_possibility(&gears[1..gears.len()].to_vec(), config, memo);
            let mut broken = 0;
            if *c <= gears.len()
                && gears[0..*c].iter().all(|x| can_break(x))
                && (*c == gears.len() || can_operate(&gears[*c]))
            {
                broken = try_memo_possibility(
                    &gears[*vec![*c + 1, gears.len()].iter().min().unwrap()..gears.len()].to_vec(),
                    &config[1..config.len()].to_vec(),
                    memo,
                );
            }
            return operational + broken;
        }
        _ => todo!(),
    }
}

fn count_possibilities(line: &str) -> usize {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let gears = parts[0].chars().collect::<Vec<char>>();
    let config = parts[1]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    try_memo_possibility(&gears, &config, &mut HashMap::new())
}

fn count_5x_possibilities(line: &str) -> usize {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let gears = vec![parts[0]; 5].join("?").chars().collect::<Vec<char>>();
    let config = vec![
        parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        5
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<usize>>();

    try_memo_possibility(&gears, &config, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid() {
        let tests = [
            (String::from("???.### 1,1,3"), 1),
            (String::from(".??..??...?##. 1,1,3"), 4),
            (String::from("?#?#?#?#?#?#?#? 1,3,1,6"), 1),
            (String::from("????.#...#... 4,1,1"), 1),
            (String::from("????.######..#####. 1,6,5"), 4),
            (String::from("?###???????? 3,2,1"), 10),
            (String::from(".???..??##.. 2,4"), 2),
            (String::from("??##.?#?.?#?# 4,3,3"), 1),
            (String::from("?????.??##?????????. 2,6,2"), 48),
            (String::from("????????..?????#?#?? 3,5"), 21),
            (String::from("???????? 3"), 6),
            (String::from("??? 3"), 1),
            (String::from("?????#?#?? 5"), 3),
            (String::from("?????#?#?? 3,5"), 3),
            (String::from("?.???????###.????? 1,2,2,4,3"), 3),
            (String::from("#?#???.??#?? 4,4"), 2),
            (String::from(".????#?????.?????.? 8,3"), 4),
            (String::from("??#??#?????.?????? 7,5"), 6),
            (String::from("#??#??#???#..??? 1,5,2,1"), 6),
        ];

        for (input, want) in tests {
            assert_eq!(count_possibilities(&input), want, "with input\n{}", input);
        }
    }

    #[test]
    fn test_get_valid_pt2() {
        let tests = [
            (String::from("???.### 1,1,3"), 1),
            (String::from(".??..??...?##. 1,1,3"), 16384),
            (String::from("?#?#?#?#?#?#?#? 1,3,1,6"), 1),
            (String::from("????.#...#... 4,1,1"), 16),
            (String::from("????.######..#####. 1,6,5"), 2500),
            (String::from("?###???????? 3,2,1"), 506250),
        ];

        for (input, want) in tests {
            assert_eq!(
                count_5x_possibilities(&input),
                want,
                "with input\n{}",
                input
            );
        }
    }
}
