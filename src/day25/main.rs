use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day25/input.txt";
    println!("In file {}", file_path);

    {
        println!(
            "Part 1: {}",
            to_snafu(
                read_lines(file_path)
                    .iter()
                    .map(|x| from_snafu(x))
                    .sum::<i64>()
            )
        );
    }
}

fn from_snafu(s: &str) -> i64 {
    let mut res: i64 = 0;
    let mut mult: i64 = 1;
    for c in s.chars().rev() {
        match c {
            '=' => res += -2 * mult,
            '-' => res += -mult,
            _ => res += ((c as u8 - '0' as u8) as i64) * mult,
        }
        mult *= 5;
    }
    return res;
}

fn to_snafu(v: i64) -> String {
    let mut base5: Vec<u8> = Vec::new();
    let mut v5 = v;
    while v5 > 0 {
        base5.push((v5 % 5) as u8);
        v5 = v5 / 5;
    }

    // Already in snafu format.
    if base5.iter().all(|&x| x <= 2) {
        return base5
            .iter()
            .map(|x| (x + '0' as u8) as char)
            .rev()
            .collect::<String>();
    }

    let mut conv: Vec<i8> = Vec::new();
    let mut carry: i8 = 0;
    for &x in &base5 {
        let med = (x as i8) + carry;
        if med > 2 {
            conv.push(med - 5);
            carry = 1;
        } else {
            conv.push(med);
            carry = 0;
        }
    }
    if carry > 0 {
        conv.push(carry);
    }

    return conv
        .iter()
        .map(|&x| match x {
            0 | 1 | 2 => ((x as u8) + ('0' as u8)) as char,
            -1 => '-',
            -2 => '=',
            _ => {
                assert!(false, "invalid snafu character: {}", x);
                '.'
            }
        })
        .rev()
        .collect::<String>();
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
    fn example_works() {
        let tests = [
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
            ("1", 1),
            ("2", 2),
            ("1=", 3),
            ("1-", 4),
            ("10", 5),
            ("11", 6),
            ("12", 7),
            ("2=", 8),
            ("2-", 9),
            ("20", 10),
            ("1=0", 15),
            ("1-0", 20),
            ("1=11-2", 2022),
            ("1-0---0", 12345),
            ("1121-1110-1=0", 314159265),
        ];

        for (sn, want) in tests {
            let got = from_snafu(sn);
            assert_eq!(got, want, "From {}, exected {}, got {}", sn, want, got);
        }

        for (want, vl) in tests {
            let got = to_snafu(vl);
            assert_eq!(got, want, "From {}, expected {}, got {}", vl, want, got);
        }
    }
}
