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
    // 34893 is low
    // 34993
    println!("Part 1: {}", pt1);
    //  let pt2: usize = handle_pt1(&lines);
    //  println!("Part 2: {}", pt2);
}

fn handle_pt1(lines: &Vec<String>) -> usize {
    println!(
        "{}",
        split_input(lines)
            .iter()
            .map(|l| reflection(l))
            .enumerate()
            .map(|(i, x)| format!("{} = {}", i, x).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
    split_input(lines).iter().map(|l| reflection(l)).sum()
}

fn find_reflection(lines: &Vec<String>) -> usize {
    for i in 1..lines.len() {
        let mut matches = true;
        for r in 0..*(vec![i, lines.len() - i].iter().min().unwrap()) {
            if lines[i + r] != lines[i - r - 1] {
                matches = false;
            }
        }
        if matches {
            return i;
        }
    }

    return 0;
}

fn reflection(lines: &Vec<String>) -> usize {
    let horizontal = find_horizontal_reflection(lines);
    let vertical = find_vertical_reflection(lines);
    assert!(horizontal > 0 || vertical > 0);
    assert!(horizontal == 0 || vertical == 0);
    let mut vertical_indicator: String = String::from("");
    if vertical > 0 {
        vertical_indicator = format!("{}><", vec![" "; vertical - 1].join("")).to_string();
    }
    //     println!(
    //         "GRID
    //  {}
    // {}
    //  {}
    // horizontal = {}
    // vertical = {}
    // ",
    //         vertical_indicator,
    //         lines
    //             .iter()
    //             .enumerate()
    //             .map(|(i, l)| {
    //                 if horizontal > 0 {
    //                     if i == horizontal / 100 {
    //                         return format!("^{}^", l).to_string();
    //                     } else if i == horizontal / 100 - 1 {
    //                         return format!("v{}v", l).to_string();
    //                     }
    //                 }
    //                 return format!(" {}", l).to_string();
    //             })
    //             .collect::<Vec<String>>()
    //             .join("\n"),
    //         vertical_indicator,
    //         horizontal,
    //         vertical
    //     );
    return horizontal + vertical;
}

fn find_horizontal_reflection(lines: &Vec<String>) -> usize {
    find_reflection(lines) * 100
}

fn find_vertical_reflection(lines: &Vec<String>) -> usize {
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    //  println!("width = {}, height = {}", width, height);

    let mut transpose: Vec<String> = Vec::new();
    let mut curr: Vec<char> = Vec::new();
    for w in 0..width {
        for h in 0..height {
            curr.push(lines[h].chars().nth(w).unwrap());
        }
        transpose.push(curr.iter().collect::<String>());
        curr = Vec::new();
    }

    find_reflection(&transpose)
}

fn split_input(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = Vec::new();
    let mut curr: Vec<String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            res.push(curr);
            curr = Vec::new();
            continue;
        }

        curr.push(line.clone());
    }

    if !curr.is_empty() {
        res.push(curr);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection() {
        let tests = [
            (
                vec![
                    String::from("#.##..##."),
                    String::from("..#.##.#."),
                    String::from("##......#"),
                    String::from("##......#"),
                    String::from("..#.##.#."),
                    String::from("..##..##."),
                    String::from("#.#.##.#."),
                ],
                5,
            ),
            (
                vec![
                    String::from("#...##..#"),
                    String::from("#....#..#"),
                    String::from("..##..###"),
                    String::from("#####.##."),
                    String::from("#####.##."),
                    String::from("..##..###"),
                    String::from("#....#..#"),
                ],
                400,
            ),
            (
                vec![
                    String::from("##..##..##."),
                    String::from("######..###"),
                    String::from(".####.##.##"),
                    String::from("..........#"),
                    String::from(".####.##.##"),
                    String::from(".####....##"),
                    String::from("..##..##..#"),
                ],
                3,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(reflection(&input), want, "with input\n{}", input.join("\n"));
        }
    }
}
