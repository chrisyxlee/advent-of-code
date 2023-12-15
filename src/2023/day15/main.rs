use advent_of_code::utils::input::read_lines;
use clap::Parser;
use std::collections::HashMap;

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

    let pt1: usize = handle_pt1(&codes);
    println!("Part 1: {}", pt1);
    let pt2: usize = handle_pt2(&codes);
    println!("Part 2: {}", pt2);
}

/*
Determine the ASCII code for the current character of the string.
Increase the current value by the ASCII code you just determined.
Set the current value to itself multiplied by 17.
Set the current value to the remainder of dividing itself by 256.
*/
fn hash(code: &str) -> usize {
    let mut total = 0;
    for c in code.chars() {
        total += c as usize;
        total *= 17;
        total %= 256;
    }

    total
}

fn handle_pt1(codes: &str) -> usize {
    codes.split(",").into_iter().map(|c| hash(c)).sum()
}

fn parse_code(code: &str) -> (String, char, Option<usize>) {
    if let Some(idx) = code.find('-') {
        return (code[0..idx].to_string(), '-', None);
    } else if let Some(idx) = code.find('=') {
        return (
            code[0..idx].to_string(),
            '=',
            Some(code[idx + 1..code.len()].parse::<usize>().unwrap()),
        );
    }

    assert!(false);
    return (String::from(""), '?', None);
}

pub struct LensBox {
    indices: HashMap<String, usize>,
    labels: HashMap<usize, String>,
    focals: HashMap<String, usize>,
    next_index: usize,
}

impl LensBox {
    pub fn new() -> Self {
        Self {
            indices: HashMap::new(),
            labels: HashMap::new(),
            focals: HashMap::new(),
            next_index: 0,
        }
    }

    // If the operation character is an equals sign (=), it will be followed by
    // a number indicating the focal length of the lens that needs to go into the
    // relevant box; be sure to use the label maker to mark the lens with the label
    // given in the beginning of the step so you can find it later.
    //
    // There are two possible situations:
    // - If there is already a lens in the box with the same label, replace the old lens
    //   with the new lens: remove the old lens and put the new lens in its place, not
    //   moving any other lenses in the box.
    // - If there is not already a lens in the box with the same label, add the lens to
    //   the box immediately behind any lenses already in the box. Don't move any of the
    //   other lenses when you do this. If there aren't any lenses in the box, the new
    //   lens goes all the way to the front of the box.
    pub fn add(&mut self, label: &str, focal_length: usize) {
        if !self.indices.contains_key(label) {
            self.indices.insert(label.to_string(), self.next_index);
            self.labels.insert(self.next_index, label.to_string());
            self.next_index += 1;
        }
        self.focals
            .entry(label.to_string())
            .and_modify(|e| *e = focal_length)
            .or_insert(focal_length);
    }

    // If the operation character is a dash (-), go to the relevant box and
    // remove the lens with the given label if it is present in the box. Then,
    // move any remaining lenses as far forward in the box as they can go without
    // changing their order, filling any space made by removing the indicated
    // lens. (If no lens in that box has the given label, nothing happens.)
    pub fn remove(&mut self, label: &str) {
        if self.indices.get(label).is_none() {
            return;
        }

        let idx = *self.indices.get(label).unwrap();
        self.indices.remove(label);
        self.labels.remove(&idx);
        self.focals.remove(label);
    }
}

fn handle_pt2(codes: &str) -> usize {
    let mut lens_boxes: Vec<LensBox> = Vec::new();
    for _ in 0..256 {
        lens_boxes.push(LensBox::new());
    }

    for code in codes.split(",") {
        let (label, op, focal_length) = parse_code(code);
        let box_number = hash(&label);
        match op {
            '-' => lens_boxes[box_number].remove(&label),
            '=' => lens_boxes[box_number].add(&label, focal_length.unwrap()),
            _ => todo!(),
        }
    }

    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    lens_boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let box_number = i + 1;
            let mut lens_total = 0;
            let mut real_index = 0;
            for fake_index in 0..b.next_index {
                if let Some(label) = b.labels.get(&fake_index) {
                    let focal_length = b.focals.get(label).unwrap();
                    let lens_value = box_number * (real_index + 1) * focal_length;
                    lens_total += lens_value;
                    real_index += 1;
                }
            }

            lens_total
        })
        .sum()
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

    #[test]
    fn test_handle_pt2() {
        let tests = [(
            String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "with input\n{}", input);
        }
    }
}
