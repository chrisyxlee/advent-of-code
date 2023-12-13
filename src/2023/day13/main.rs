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

    let (pt1, pt2): (usize, usize) = handle(&lines);
    // 34893 is low
    // 34993
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}

fn handle(lines: &Vec<String>) -> (usize, usize) {
    let (part1, part2): (Vec<usize>, Vec<usize>) =
        split_input(lines).iter().map(|l| reflection(l)).unzip();
    (part1.iter().sum(), part2.iter().sum())
}

fn diff(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());

    let mut diffs = 0;
    for i in 0..a.len() {
        if a.chars().nth(i).unwrap() != b.chars().nth(i).unwrap() {
            diffs += 1;
        }
    }

    diffs
}

fn find_smudged_reflection(lines: &Vec<String>) -> usize {
    for i in 1..lines.len() {
        let mut matches = true;
        let mut diffs = 0;
        for r in 0..*(vec![i, lines.len() - i].iter().min().unwrap()) {
            diffs += diff(&lines[i + r], &lines[i - r - 1]);
            if diffs > 1 {
                matches = false;
                break;
            }
        }
        if diffs == 1 && matches {
            return i;
        }
    }

    0
}

fn find_reflection(lines: &Vec<String>) -> usize {
    for i in 1..lines.len() {
        let mut matches = true;
        for r in 0..*(vec![i, lines.len() - i].iter().min().unwrap()) {
            if lines[i + r] != lines[i - r - 1] {
                matches = false;
                break;
            }
        }
        if matches {
            return i;
        }
    }

    return 0;
}

fn reflection(lines: &Vec<String>) -> (usize, usize) {
    let transposed = transpose(lines);

    let horizontal_pt1 = find_reflection(lines) * 100;
    let vertical_pt1 = find_reflection(&transposed);
    assert!(horizontal_pt1 > 0 || vertical_pt1 > 0);
    assert!(horizontal_pt1 == 0 || vertical_pt1 == 0);

    let horizontal_pt2 = find_smudged_reflection(lines) * 100;
    let vertical_pt2 = find_smudged_reflection(&transposed);
    assert!(
        horizontal_pt2 > 0 || vertical_pt2 > 0,
        "{}",
        lines.join("\n")
    );
    assert!(
        horizontal_pt2 == 0 || vertical_pt2 == 0,
        "h={}, v={}",
        horizontal_pt2,
        vertical_pt2
    );

    let pt1 = horizontal_pt1 + vertical_pt1;
    let mut pt2 = vertical_pt2;
    if horizontal_pt2 > 0 && horizontal_pt2 != pt1 {
        pt2 = horizontal_pt2;
    }
    assert_ne!(pt1, pt2);

    (pt1, pt2)
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
    let height = lines.len();
    let width = lines.iter().map(|line| line.len()).max().unwrap();

    let mut transpose: Vec<String> = Vec::new();
    let mut curr: Vec<char> = Vec::new();
    for w in 0..width {
        for h in 0..height {
            curr.push(lines[h].chars().nth(w).unwrap());
        }
        transpose.push(curr.iter().collect::<String>());
        curr = Vec::new();
    }

    transpose
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
                (5, 300),
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
                (400, 100),
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
                (3, 7),
            ),
        ];

        for (input, want) in tests {
            assert_eq!(reflection(&input), want, "with input\n{}", input.join("\n"));
        }
    }
}
