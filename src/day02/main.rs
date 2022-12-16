use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("In file {}", file_path);

    let mut score = 0;

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let parts = ip.split(" ").collect::<Vec<&str>>();
                assert!(parts.len() == 2);
                let opponent = decode_play(parts[0]);
                let outcome = decode_outcome(parts[1]);

                let my_play = get_my_play(opponent, outcome);
                let choice_points = get_choice_points(my_play);

                score += outcome + choice_points;
            }
        }
        println!("Total points: {}", score);
    }
}

fn decode_play(code: &str) -> &str {
    match code {
        "A" => "Rock",
        "B" => "Paper",
        "C" => "Scissors",
        _ => "",
    }
}

fn decode_outcome(code: &str) -> i32 {
    match code {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => -1,
    }
}

fn get_my_play(opponent: &str, outcome: i32) -> &str {
    if outcome == 3 {
        return opponent; // Draw
    }
    match opponent {
        "Rock" => match outcome {
            6 => return "Paper",
            0 => return "Scissors",
            _ => return "",
        },
        "Paper" => match outcome {
            6 => return "Scissors",
            0 => return "Rock",
            _ => return "",
        },
        "Scissors" => match outcome {
            6 => return "Rock",
            0 => return "Paper",
            _ => return "",
        },
        _ => return "",
    }
}

fn get_choice_points(code: &str) -> i32 {
    match code {
        "Rock" => return 1,
        "Paper" => return 2,
        "Scissors" => return 3,
        _ => return -1,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
