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

    let pt1: usize = handle_pt1(&lines);
    // 7215 too low
    println!("Part 1: {}", pt1);
    //  let pt2: usize = handle_pt1(&lines);
    //  println!("Part 2: {}", pt2);
}

fn handle_pt1(lines: &Vec<String>) -> usize {
    lines.iter().map(|line| count_possibilities(line)).sum()
}
fn can_break(g: &char) -> bool {
    *g == '#' || *g == '?'
}
fn can_operate(g: &char) -> bool {
    *g == '.' || *g == '?'
}

fn try_possibility(gears: &Vec<char>, config: &Vec<usize>) -> usize {
    if gears.len() == 0 {
        let mut possibilities = 0;
        if config.len() == 0 {
            possibilities = 1;
        }
        println!(
            "({}, {}) --> = {} possibilites",
            gears.iter().collect::<String>(),
            config
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            possibilities
        );

        return possibilities;
    }

    if config.len() == 0 {
        let mut possibilities = 0;
        if gears.iter().all(|x| can_operate(x)) {
            possibilities = 1;
        }
        println!(
            "({}, {}) --> = {} possibilites",
            gears.iter().collect::<String>(),
            config
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            possibilities
        );

        return possibilities;
    }

    let c = config.first().unwrap();
    let g = gears.first().unwrap();

    match (*c, *g) {
        (1, '#') => {
            if gears.len() > 1 && can_operate(&gears[1]) {
                let possibilities = try_possibility(
                    &gears[2..gears.len()].to_vec(),
                    &config[1..config.len()].to_vec(),
                );
                println!(
                    "({}, {}) --> {} and {} = {} possibilites",
                    gears.iter().collect::<String>(),
                    config
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    c,
                    g,
                    possibilities
                );
                return possibilities;
            } else if gears.len() == 1 {
                println!(
                    "({}, {}) --> {} and {} = 1 possibilites",
                    gears.iter().collect::<String>(),
                    config
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    c,
                    g,
                );
                return 1;
            } else {
                println!(
                    "({}, {}) --> {} and {} = {} possibilites",
                    gears.iter().collect::<String>(),
                    config
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    c,
                    g,
                    0
                );
                return 0;
            }
        }
        (_, '#') => {
            let mut possibilities = 0;
            if *c <= gears.len()
                && gears[0..*c].iter().all(|x| can_break(x))
                && (*c == gears.len() || can_operate(&gears[*c]))
            {
                possibilities += try_possibility(
                    &gears[*vec![*c + 1, gears.len()].iter().min().unwrap()..gears.len()].to_vec(),
                    &config[1..config.len()].to_vec(),
                );
            }
            println!(
                "({}, {}) --> {} and {} = {} possibilites",
                gears.iter().collect::<String>(),
                config
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                c,
                g,
                possibilities
            );
            return possibilities;
        }
        (_, '.') => {
            let possibilities = try_possibility(&gears[1..gears.len()].to_vec(), config);
            println!(
                "({}, {}) --> {} and {} = {} possibilites",
                gears.iter().collect::<String>(),
                config
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                c,
                g,
                possibilities
            );
            return possibilities;
        }
        (1, '?') => {
            let operational = try_possibility(&gears[1..gears.len()].to_vec(), config);
            println!(
                "({}, {}) --> {} and {} (try operational) = {} possibilites",
                gears.iter().collect::<String>(),
                config
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                c,
                g,
                operational
            );
            let mut broken = 0;
            if gears.len() == 1 || (gears.len() > 1 && can_operate(&gears[1])) {
                broken = try_possibility(
                    &gears[*vec![2 as usize, gears.len()].iter().min().unwrap()..gears.len()]
                        .to_vec(),
                    &config[1..config.len()].to_vec(),
                );
                println!(
                    "({}, {}) --> {} and {} (try broken) = {} possibilites",
                    gears.iter().collect::<String>(),
                    config
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    c,
                    g,
                    broken,
                );
            }
            println!(
                "({}, {}) --> {} and {} = {} operational + {} broken = {} possibilites",
                gears.iter().collect::<String>(),
                config
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                c,
                g,
                operational,
                broken,
                operational + broken
            );
            return operational + broken;
        }
        (_, '?') => {
            // 4, ????.
            let operational = try_possibility(&gears[1..gears.len()].to_vec(), config);
            println!(
                "({}, {}) --> {} and {} (try operational) = {} possibilites",
                gears.iter().collect::<String>(),
                config
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                c,
                g,
                operational
            );
            let mut broken = 0;
            if *c < gears.len()
                && gears[0..*c].iter().all(|x| can_break(x))
                && (*c == gears.len() || can_operate(&gears[*c]))
            {
                broken = try_possibility(
                    &gears[*c + 1..gears.len()].to_vec(),
                    &config[1..config.len()].to_vec(),
                );
                println!(
                    "({}, {}) --> {} and {} (try broken) = {} possibilites",
                    gears.iter().collect::<String>(),
                    config
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    c,
                    g,
                    broken,
                );
            }
            println!(
                "({}, {}) --> {} and {} = {} operational + {} broken = {} possibilites",
                gears.iter().collect::<String>(),
                config
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                c,
                g,
                operational,
                broken,
                operational + broken
            );
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

    println!(
        "START gears = {}, config = {}",
        gears.iter().collect::<String>(),
        config
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    try_possibility(&gears, &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valid() {
        let tests = [
            // (String::from("???.### 1,1,3"), 1),
            // (String::from(".??..??...?##. 1,1,3"), 4),
            // (String::from("?#?#?#?#?#?#?#? 1,3,1,6"), 1),
            // (String::from("????.#...#... 4,1,1"), 1),
            // (String::from("????.######..#####. 1,6,5"), 4),
            // (String::from("?###???????? 3,2,1"), 10),
            // (String::from(".???..??##.. 2,4"), 2),
            // (String::from("??##.?#?.?#?# 4,3,3"), 1),
            // (String::from("?????.??##?????????. 2,6,2"), 48),
            (String::from("????????..?????#?#?? 3,5"), 18),
            // (String::from("?.???????###.????? 1,2,2,4,3"), 4),
            // (String::from("#?#???.??#?? 4,4"), 4),
            // (String::from(".????#?????.?????.? 8,3"), 4),
            // (String::from("??#??#?????.?????? 7,5"), 4),
            // (String::from("#??#??#???#..??? 1,5,2,1"), 4),
        ];

        for (input, want) in tests {
            assert_eq!(count_possibilities(&input), want, "with input\n{}", input);
        }
    }
}
