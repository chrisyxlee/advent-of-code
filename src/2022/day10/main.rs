use advent_of_code::utils::input::read_lines;
use clap::Parser;

// Disclaimer: there's a bug here somewhere, but it got close enough that I submitted the right answer.

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

    {
        let mut cycle = 0;
        let mut x = 1;
        let mut total = 0;

        // Part 1
        for line in lines.iter() {
            if let Some(num_add) = parse_line(line.as_str()) {
                total += maybe_grab_value(cycle + 1, x);
                total += maybe_grab_value(cycle + 2, x);
                cycle += 2;
                x += num_add;
            } else {
                cycle += 1;
                total += maybe_grab_value(cycle, x);
            }
        }
        println!("total is {}", total);
    }

    // Part 2
    {
        let mut cycle = 0;
        let mut x = 1;
        for line in lines.iter() {
            if let Some(num_add) = parse_line(line.as_str()) {
                cycle = get_pixel(cycle + 1, x);
                cycle = get_pixel(cycle + 1, x);
                x += num_add;
                if x < 0 {
                    x = 0;
                }
                if x > 39 {
                    x = 40;
                }
            } else {
                cycle = get_pixel(cycle + 1, x);
            }
        }
    }
}

fn get_pixel(cycle: i32, x: i32) -> i32 {
    let position = cycle % 40;
    if (position - x).abs() <= 1 {
        print!("██");
    } else {
        print!("░░");
    }
    if position == 0 {
        println!("");
    }
    return cycle;
}

fn maybe_grab_value(cycle: i32, x: i32) -> i32 {
    if cycle % 40 == 20 {
        println!("cycle {} has value {} = {}", cycle, x, cycle * x);
        return cycle * x;
    }
    return 0;
}

fn parse_line(line: &str) -> Option<i32> {
    if line == "noop" {
        return None;
    }

    let parts: Vec<&str> = line.split(" ").collect();
    assert!(parts[0] == "addx");
    return Some(parts[1].parse::<i32>().unwrap());
}
