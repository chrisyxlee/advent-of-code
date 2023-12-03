fn main() {
    let secret = "ckczppom";
    let pt1: usize = handle_pt1(secret);
    println!("Part 1: {}", pt1);
    let pt2: usize = handle_pt2(secret);
    println!("Part 2: {}", pt2);
}

fn handle_pt1(s: &str) -> usize {
    let mut i = 0;
    loop {
        if format!("{:x}", md5::compute(format!("{}{}", s, i))).starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

fn handle_pt2(s: &str) -> usize {
    let mut i = 0;
    loop {
        if format!("{:x}", md5::compute(format!("{}{}", s, i))).starts_with("000000") {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let tests = [("abcdef", 609043), ("pqrstuv", 1048970)];

        for (input, want) in tests {
            assert_eq!(handle_pt1(input), want, "for input {}", input);
        }
    }
}
