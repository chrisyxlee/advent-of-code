use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day08/input.txt";
    println!("In file {}", file_path);

    let lines: Vec<String> = read_lines(file_path)
        .expect("where is the file")
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect();

    // Initialize the matrix.
    let num_rows = lines.len();
    let mut num_columns = 0;
    for line in lines.iter() {
        assert!(line.len() > 0);
        num_columns = line.len();
    }

    println!("height is {}, width is {}", num_rows, num_columns);
    let mut matrix: Vec<Vec<i8>> = Vec::with_capacity(num_rows);
    let mut visibles: Vec<Vec<i8>> = Vec::with_capacity(num_rows);
    for r in 0..num_rows {
        matrix.push(Vec::with_capacity(num_columns));
        visibles.push(Vec::with_capacity(num_columns));
        for _c in 0..num_columns {
            matrix[r].push(0);
            visibles[r].push(0);
        }
    }

    // Populate the matrix.
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            matrix[r][c] = ch as i8 - '0' as i8;
        }
    }

    // Find all visible:

    // From LEFT
    for r in 0..num_rows {
        let mut tallest = -1;
        for c in 0..num_columns {
            if matrix[r][c] > tallest {
                println!(
                    "Row ({}x{}): {} is taller than {}",
                    r, c, matrix[r][c], tallest
                );
                visibles[r][c] = 1;
                tallest = matrix[r][c];
            }
        }
    }
    // From RIGHT
    for r in 0..num_rows {
        let mut tallest = -1;
        for c in (0..num_columns).rev() {
            if matrix[r][c] > tallest {
                println!(
                    "Row ({}x{}): {} is taller than {}",
                    r, c, matrix[r][c], tallest
                );
                visibles[r][c] = 1;
                tallest = matrix[r][c];
            }
        }
    }
    // From TOP
    for c in 0..num_columns {
        let mut tallest = -1;
        for r in 0..num_rows {
            if matrix[r][c] > tallest {
                println!(
                    "Row ({}x{}): {} is taller than {}",
                    r, c, matrix[r][c], tallest
                );
                visibles[r][c] = 1;
                tallest = matrix[r][c];
            }
        }
    }
    // From BOTTOM
    for c in 0..num_columns {
        let mut tallest = -1;
        for r in (0..num_rows).rev() {
            if matrix[r][c] > tallest {
                println!(
                    "Row ({}x{}): {} is taller than {}",
                    r, c, matrix[r][c], tallest
                );
                visibles[r][c] = 1;
                tallest = matrix[r][c];
            }
        }
    }

    // Count.
    let mut visible = 0;
    for r in 0..num_rows {
        for c in 0..num_columns {
            if visibles[r][c] > 0 {
                visible += 1;
            }
        }
    }

    println!("Matrix");
    for r in 0..num_rows {
        println!("{:?}", matrix[r])
    }

    println!("\n\nVisibility");
    for r in 0..num_rows {
        println!("{:?}", visibles[r])
    }

    println!("Visible tree: {}", visible);

    let mut max_score = 0;
    for r in 0..num_rows {
        for c in 0..num_columns {
            let current = matrix[r][c];

            // From TOP
            let mut top_score = 0;
            for rr in (0..r).rev() {
                top_score += 1;
                if matrix[rr][c] >= current {
                    break;
                }
            }
            // From BOTTOM
            let mut bottom_score = 0;
            for rr in r + 1..num_rows {
                bottom_score += 1;
                if matrix[rr][c] >= current {
                    break;
                }
            }
            // From LEFT
            let mut left_score = 0;
            for cc in (0..c).rev() {
                left_score += 1;
                if matrix[r][cc] >= current {
                    break;
                }
            }
            // From RIGHT
            let mut right_score: i32 = 0;
            for cc in c + 1..num_columns {
                right_score += 1;
                if matrix[r][cc] >= current {
                    break;
                }
            }

            let tot_score = left_score * right_score * top_score * bottom_score;
            if tot_score > max_score {
                max_score = tot_score;
            }
        }
    }
    println!("max score: {}", max_score);
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
