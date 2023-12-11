use advent_of_code::utils::input::read_lines;
use advent_of_code::utils::point::Point;
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

    let pt1: usize = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
    //  let pt2: i32 = handle_pt2(&lines);
    //  println!("Part 2: {}", pt2);
}

fn handle_pt1(lines: &Vec<String>) -> usize {
    let mut galaxies: Vec<Point<usize>> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, shape) in line.chars().enumerate() {
            match shape {
                '.' => continue,
                '#' => galaxies.push(Point { x: col, y: row }),
                _ => {}
            }
        }
    }

    let height = lines.len();
    let empty_rows = (0..height)
        .into_iter()
        .filter(|row| galaxies.iter().all(|galaxy| galaxy.y != *row))
        .collect::<Vec<usize>>();

    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let empty_cols = (0..width)
        .into_iter()
        .filter(|col| galaxies.iter().all(|galaxy| galaxy.x != *col))
        .collect::<Vec<usize>>();

    //     println!(
    //         "RAW
    // {}",
    //         galaxies
    //             .iter()
    //             .map(|x| x.to_string())
    //             .collect::<Vec<String>>()
    //             .join("\n")
    //     );
    //  println!(
    //      "expand rows {} and cols {}",
    //      empty_rows
    //          .iter()
    //          .map(|x| x.to_string())
    //          .collect::<Vec<String>>()
    //          .join(","),
    //      empty_cols
    //          .iter()
    //          .map(|x| x.to_string())
    //          .collect::<Vec<String>>()
    //          .join(",")
    //  );

    for (r, row) in empty_rows.iter().enumerate() {
        for i in 0..galaxies.len() {
            let mut g = galaxies[i];
            if g.y > row + r {
                g.y += 1;
            }
            galaxies[i] = g;
        }
    }
    //     println!(
    //         "AFTER ROW EXPAND
    // {}",
    //         galaxies
    //             .iter()
    //             .map(|x| x.to_string())
    //             .collect::<Vec<String>>()
    //             .join("\n")
    //     );
    for (c, col) in empty_cols.iter().enumerate() {
        for i in 0..galaxies.len() {
            let mut g = galaxies[i];
            if g.x > col + c {
                g.x += 1;
            }
            galaxies[i] = g;
        }
    }

    //     println!(
    //         "AFTER COL EXPAND
    // {}",
    //         galaxies
    //             .iter()
    //             .map(|x| x.to_string())
    //             .collect::<Vec<String>>()
    //             .join("\n")
    //     );

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total += ((galaxies[i].x as i32 - galaxies[j].x as i32).abs() as usize)
                + ((galaxies[i].y as i32 - galaxies[j].y as i32).abs() as usize);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(
            vec![
                String::from("...#......"),
                String::from(".......#.."),
                String::from("#........."),
                String::from(".........."),
                String::from("......#..."),
                String::from(".#........"),
                String::from(".........#"),
                String::from(".........."),
                String::from(".......#.."),
                String::from("#...#....."),
            ],
            374,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
