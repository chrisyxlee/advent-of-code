use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day20/input.txt";
    println!("In file {}", file_path);

    {
        let mut file = parse_file(file_path, 1);
        decrypt(&mut file, 1);
        println!("Part 1: {}", get_score(&file));
    }
    {
        let mut file = parse_file(file_path, 811589153);
        decrypt(&mut file, 10);
        println!("Part 2: {}", get_score(&file));
    }
}

fn get_score(v: &Vec<i64>) -> i64 {
    // 1000th, 2000th, and 3000th
    let p0 = v.iter().position(|&x| x == 0).unwrap();
    let p1k = (p0 + 1000) % v.len();
    let p2k = (p0 + 2000) % v.len();
    let p3k = (p0 + 3000) % v.len();
    return v[p1k] + v[p2k] + v[p3k];
}

fn decrypt(vals: &mut Vec<i64>, num_rounds: usize) {
    let mut positions: Vec<usize> = vals.iter().enumerate().map(|(i, _)| i).collect();
    for _round in 0..num_rounds {
        for position in 0..positions.len() {
            let src = positions.iter().position(|&y| y == position).unwrap();
            let val = vals[position];
            if val == 0 {
                continue;
            }

            positions.remove(src);
            let dst = (src as i64 + val).rem_euclid(positions.len() as i64) as usize;
            positions.insert(dst, position);
        }
    }

    let copy = vals.clone();
    for (i, &p) in positions.iter().enumerate() {
        vals[i] = copy[p];
    }
}

fn parse_file<P>(filename: P, key: i64) -> Vec<i64>
where
    P: AsRef<Path>,
{
    return io::BufReader::new(File::open(filename).expect("where is the file"))
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap().parse::<i64>().unwrap() * key)
        .collect::<Vec<i64>>();
}

fn _circular_eq(a: &Vec<i64>, b: &Vec<i64>) {
    assert_eq!(a.len(), b.len());
    let mut ai = a.iter().position(|&x| x == 0).unwrap();
    let mut bi = b.iter().position(|&x| x == 0).unwrap();
    for _i in 0..a.len() {
        assert_eq!(
            a[ai], b[bi],
            "a and b differ at index: a[{}] has {}, b[{}] has {}\na: {:?}\nb: {:?}\n",
            ai, a[ai], bi, b[bi], a, b
        );

        ai = (ai + 1) % a.len();
        bi = (bi + 1) % b.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_works() {
        let tests = [
            /*
            Initial arrangement:
            1, 2, -3, 3, -2, 0, 4
            0, 1,  2, 3,  4, 5, 6
            1 moves between 2 and -3:
            2, 1, -3, 3, -2, 0, 4
            1, 0,  2, 3,  4, 5, 6
            2 moves between -3 and 3:
            1, -3, 2, 3, -2, 0, 4
            0,  2, 1, 3,  4, 5, 6
            -3 moves between -2 and 0:
            1, 2, 3, -2, -3, 0, 4
            0, 1, 3,  4,  2, 5, 6
            3 moves between 0 and 4:
            1, 2, -2, -3, 0, 3, 4
            0, 1,  4,  2, 5, 3, 6
            -2 moves between 4 and 1:
            1, 2, -3, 0, 3, 4, -2
            0, 1,  2, 5, 3, 6,  4
            0 does not move:
            1, 2, -3, 0, 3, 4, -2
            0, 1,  2, 5, 3, 6,  4
            4 moves between -3 and 0:
            1, 2, -3, 4, 0, 3, -2
            0, 1,  2, 6, 5, 3,  4
            */
            (
                Vec::from([1, 2, -3, 3, -2, 0, 4]),
                Vec::from([1, 2, -3, 4, 0, 3, -2]),
                3,
            ),
        ];
        for (f, want_file, want) in tests {
            let mut file: Vec<i64> = f.clone();
            decrypt(&mut file);
            _circular_eq(&file, &want_file);
            assert_eq!(pt1(&file), want);
        }
    }
}
