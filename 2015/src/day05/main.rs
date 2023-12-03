use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "tmp/day05/input.txt";
    let lines = &read_lines(file_path);
    let pt1: usize = lines
        .iter()
        .map(|x| match is_nice_pt1(x) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("Part 1: {}", pt1);
    let pt2: usize = lines
        .iter()
        .map(|x| match is_nice_pt2(x) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("Part 2: {}", pt2);
}

/*
A nice string is one with all of the following properties:

It contains at least three vowels (aeiou only), like .
It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
 */
fn is_nice_pt1(s: &str) -> bool {
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

/*
It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
*/
fn is_nice_pt2(s: &str) -> bool {
    let mut positions: HashMap<String, usize> = HashMap::new();

    let mut doubled = false;
    let mut sandwiched = false;

    for (i, c) in s.chars().enumerate() {
        if i > 0 {
            let p = s.chars().nth(i - 1).unwrap();
            let curr = format!("{}{}", p, c);

            if let Some(pos) = positions.get(&curr) {
                if i - 1 - *pos >= 2 {
                    doubled = true;
                }
            } else {
                positions.insert(curr, i - 1);
            }
        }

        if i > 1 && !sandwiched {
            let p = s.chars().nth(i - 2).unwrap();
            if p == c {
                sandwiched = true;
            }
        }

        if sandwiched && doubled {
            return true;
        }
    }

    return false;
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
            assert_eq!(is_nice_pt1(input), want, "for input {}", input);
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
            ("xyxy", true),
            ("aabcdefgaa", false),
            ("baaab", false),
            ("abcdefeghiab", true),
            ("xyaksllpqwoefexy", true),
            ("aaaa", true),
            ("aaabcb", false),
        ];

        for (input, want) in tests {
            assert_eq!(is_nice_pt2(input), want, "for input {}", input);
        }
    }
}
