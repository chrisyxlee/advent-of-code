use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day02/input.txt";
    let lines = &read_lines(file_path);
    let pt1: i64 = lines.iter().map(|line| parse_line_pt1(line)).sum();
    println!("Part 1: {}", pt1);
}

fn parse_line_pt1(s: &str) -> i64 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let game_id_re = Regex::new(r"Game (\d+)").unwrap();
    assert!(game_id_re.is_match(s));

    let mut game_id = 0;
    for m in game_id_re.captures_iter(s) {
        for (i, capt) in m.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if let Some(sub) = capt {
                game_id = str2i64(sub.as_str());
            }
        }
    }

    let possible = color_is_possible(s, "green", max_green)
        && color_is_possible(s, "blue", max_blue)
        && color_is_possible(s, "red", max_red);

    if possible {
        return game_id;
    }

    return 0;
}

fn color_is_possible(s: &str, color: &str, max_count: i64) -> bool {
    let color_re = Regex::new(&format!(r"(\d+) {}", color)).unwrap();
    for m in color_re.captures_iter(s) {
        for (i, capt) in m.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if let Some(sub) = capt {
                let count = str2i64(sub.as_str());
                if count > max_count {
                    return false;
                }
            }
        }
    }
    return true;
}

fn str2i64(s: &str) -> i64 {
    return s.parse::<i64>().unwrap();
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect::<Vec<String>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                2,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                0,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                0,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5),
        ];

        for (input, want) in tests {
            assert_eq!(parse_line_pt1(input), want, "for input {}", input);
        }
    }
}
