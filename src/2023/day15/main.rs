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
    let codes = lines.into_iter().nth(0).unwrap();

    let pt1: i32 = handle_pt1(&codes);
    println!("Part 1: {}", pt1);
    //  let pt2: i32 = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
}

/*
Determine the ASCII code for the current character of the string.
Increase the current value by the ASCII code you just determined.
Set the current value to itself multiplied by 17.
Set the current value to the remainder of dividing itself by 256.
*/
fn hash(code: &str) -> i32 {
    let mut total = 0;
    for c in code.chars() {
        total += c as i32;
        total *= 17;
        total %= 256;
    }

    total
}

fn handle_pt1(codes: &str) -> i32 {
    codes.split(",").into_iter().map(|c| hash(c)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let tests = [
            (String::from("HASH"), 52),
            (String::from("rn=1"), 30),
            (String::from("cm-"), 253),
            (String::from("qp=3"), 97),
            (String::from("cm=2"), 47),
            (String::from("qp-"), 14),
            (String::from("pc=4"), 180),
            (String::from("ot=9"), 9),
            (String::from("ab=5"), 197),
            (String::from("pc-"), 48),
            (String::from("pc=6"), 214),
            (String::from("ot=7"), 231),
        ];

        for (input, want) in tests {
            assert_eq!(hash(&input), want, "with input\n{}", input);
        }
    }

    #[test]
    fn test_handle_codes() {
        let tests = [
            (String::from("HASH"), 52),
            (
                String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
                1320,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "with input\n{}", input);
        }
    }
}
