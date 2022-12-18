use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day09/input.txt";
    println!("In file {}", file_path);

    let lines: Vec<String> = read_lines(file_path)
        .expect("where is the file")
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .collect();

    // Part 1
    {
        let mut head_row: i32 = 0;
        let mut head_col: i32 = 0;
        let mut tail_row: i32 = 0;
        let mut tail_col: i32 = 0;
        let mut tail_visited: HashSet<String> = HashSet::new();
        tail_visited.insert(key_position(0, 0));
        for line in lines.iter() {
            let (dir, num_steps) = parse_line(line.as_str());
            for _i in 0..num_steps {
                match dir {
                    'U' => head_row += 1,
                    'D' => head_row -= 1,
                    'L' => head_col -= 1,
                    'R' => head_col += 1,
                    _ => (),
                }

                if head_col == tail_col && (tail_row - head_row).abs() > 1 {
                    tail_row += (head_row - tail_row) / (tail_row - head_row).abs();
                } else if head_row == tail_row && (tail_col - head_col).abs() > 1 {
                    tail_col += (head_col - tail_col) / (head_col - tail_col).abs();
                } else if head_row != tail_row && head_col != tail_col {
                    if (tail_col - head_col).abs() > 1 || (tail_row - head_row).abs() > 1 {
                        tail_row += (head_row - tail_row) / (tail_row - head_row).abs();
                        tail_col += (head_col - tail_col) / (head_col - tail_col).abs();
                    }
                }
                tail_visited.insert(key_position(tail_row, tail_col));
            }
        }
        println!("{} visits", tail_visited.len());
    }

    // Part 2
    // Part 1
    {
        let mut rows: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut cols: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut tail_visited: HashSet<String> = HashSet::new();
        tail_visited.insert(key_position(0, 0));
        for line in lines.iter() {
            let (dir, num_steps) = parse_line(line.as_str());
            for _i in 0..num_steps {
                match dir {
                    'U' => rows[0] += 1,
                    'D' => rows[0] -= 1,
                    'L' => cols[0] -= 1,
                    'R' => cols[0] += 1,
                    _ => (),
                }

                for i in 1..10 {
                    if cols[i - 1] == cols[i] && (rows[i] - rows[i - 1]).abs() > 1 {
                        rows[i] += (rows[i - 1] - rows[i]) / (rows[i] - rows[i - 1]).abs();
                    } else if rows[i - 1] == rows[i] && (cols[i] - cols[i - 1]).abs() > 1 {
                        cols[i] += (cols[i - 1] - cols[i]) / (cols[i - 1] - cols[i]).abs();
                    } else if rows[i - 1] != rows[i] && cols[i - 1] != cols[i] {
                        if (cols[i] - cols[i - 1]).abs() > 1 || (rows[i] - rows[i - 1]).abs() > 1 {
                            rows[i] += (rows[i - 1] - rows[i]) / (rows[i] - rows[i - 1]).abs();
                            cols[i] += (cols[i - 1] - cols[i]) / (cols[i - 1] - cols[i]).abs();
                        }
                    }
                }
                tail_visited.insert(key_position(rows[9], cols[9]));
            }
        }
        println!("{} visits", tail_visited.len());
    }
}

fn key_position(row: i32, col: i32) -> String {
    let mut ret = row.to_string();
    ret.push('x');
    ret.push_str(col.to_string().as_str());
    return ret;
}

fn parse_line(line: &str) -> (char, i32) {
    let parts: Vec<&str> = line.split(" ").collect();
    let first_part: Vec<char> = parts[0].chars().collect();
    assert!(first_part.len() == 1);

    return (first_part[0], parts[1].parse::<i32>().unwrap());
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
