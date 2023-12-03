use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day05/input.txt";
    let lines = &read_lines(file_path);
    let pt1: usize = lines
        .iter()
        .map(|x| match is_nice(x) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("Part 1: {}", pt1);
}

/*
A nice string is one with all of the following properties:

It contains at least three vowels (aeiou only), like .
It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
 */
fn is_nice(s: &str) -> bool {
    let bad = ["ab", "cd", "pq", "xy"];
    for b in bad {
        if s.find(b).is_some() {
            return false;
        }
    }

    let mut vowel_count = 0;
    let mut doubled: bool = false;
    let mut prev = None;
    for c in s.chars() {
        vowel_count += match c {
            'a' => 1,
            'e' => 1,
            'i' => 1,
            'o' => 1,
            'u' => 1,
            _ => 0,
        };

        if let Some(p) = prev {
            if c == p {
                doubled = true;
            }
        }

        prev = Some(c);
    }

    return vowel_count >= 3 && doubled;
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
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for (input, want) in tests {
            assert_eq!(is_nice(input), want, "for input {}", input);
        }
    }
}
