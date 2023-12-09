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

    let pt1: i64 = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
    //  let pt2: i64 = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
}

fn guess_value(values: &Vec<i64>) -> i64 {
    let mut differences: Vec<i64> = Vec::new();

    for i in 1..values.len() {
        differences.push(values[i] - values[i - 1]);
    }

    if differences.iter().all(|x| *x == 0) {
        return *values.last().unwrap();
    }

    return guess_value(&differences) + *values.last().unwrap();
}

fn parse_line_pt1(line: &String) -> i64 {
    let starting = line
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    return guess_value(&starting);
}

fn handle_pt1(lines: &Vec<String>) -> i64 {
    lines.iter().map(|x| parse_line_pt1(x)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(
            vec![
                String::from("0 3 6 9 12 15"),
                String::from("1 3 6 10 15 21"),
                String::from("10 13 16 21 30 45"),
            ],
            114,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
