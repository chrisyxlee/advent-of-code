use clap::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file.
    #[arg(short, long)]
    input: String,
}

fn main() {
    /*
    Time:        41     96     88     94
    Distance:   214   1789   1127   1055
    */
    let pt1: usize = handle_pt1(vec![(41, 214), (96, 1789), (88, 1127), (94, 1055)]);
    println!("Part 1: {}", pt1);
    let pt2: usize = count_hold_start(41968894, 214178911271055);
    //  let pt2: i32 = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
}

fn count_hold_start(time: i64, distance: i64) -> usize {
    (0..=time)
        .into_iter()
        .filter(|start_speed| start_speed * (time - start_speed) > distance)
        .count()
}

fn handle_pt1(times: Vec<(i64, i64)>) -> usize {
    let mut res = 1;
    for x in times
        .iter()
        .map(|(time, distance)| count_hold_start(*time, *distance))
    {
        res *= x;
    }
    res
}
