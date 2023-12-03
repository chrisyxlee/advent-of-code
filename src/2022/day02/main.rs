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
    let lines = &read_lines(args.input);

    let mut score = 0;

    // File must exist in current path before this produces output
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        assert!(parts.len() == 2);
        let opponent = decode_play(parts[0]);
        let outcome = decode_outcome(parts[1]);

        let my_play = get_my_play(opponent, outcome);
        let choice_points = get_choice_points(my_play);

        score += outcome + choice_points;
    }
    println!("Total points: {}", score);
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
