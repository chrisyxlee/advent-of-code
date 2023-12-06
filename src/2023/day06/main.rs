use clap::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file.
    #[arg(short, long)]
    input: String,
}

fn main() {
    /*
    Time:        41     96     88     94
    Distance:   214   1789   1127   1055
    */
    let input = vec![(41, 214), (96, 1789), (88, 1127), (94, 1055)];

    let pt1: usize = handle_pt1(&input);
    println!("Part 1: {}", pt1);
    //  let pt2: i32 = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
}

fn count_hold_start(time: i32, distance: i32) -> usize {
    (0..=time)
        .into_iter()
        .filter(|start_speed| start_speed * (time - start_speed) > distance)
        .count()
}

fn handle_pt1(times: &Vec<(i32, i32)>) -> usize {
    let mut res = 1;
    for x in times
        .iter()
        .map(|(time, distance)| count_hold_start(*time, *distance))
    {
        res *= x;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(
            vec![
                String::from("467..114.."),
                String::from("...*......"),
                String::from("..35..633."),
                String::from("......#..."),
                String::from("617*......"),
                String::from(".....+.58."),
                String::from("..592....."),
                String::from("......755."),
                String::from("...$.*...."),
                String::from(".664.598.."),
            ],
            4361,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [
            (
                vec![
                    String::from("467..114.."),
                    String::from("...*......"),
                    String::from("..35..633."),
                    String::from("......#..."),
                    String::from("617*......"),
                    String::from(".....+.58."),
                    String::from("..592....."),
                    String::from("......755."),
                    String::from("...$.*...."),
                    String::from(".664.598.."),
                ],
                467_835,
            ),
            (
                vec![
                    String::from("..589"),
                    String::from("..*.."),
                    String::from("699.."),
                ],
                411_711,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
