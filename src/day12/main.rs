use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // --snip--
    let file_path = "tmp/day12/input.txt";
    println!("In file {}", file_path);

    let graph: Vec<Vec<char>> = read_lines(file_path)
        .expect("where is the file")
        .filter(|x| x.is_ok())
        .map(|x| x.expect("bad lines should be filtered"))
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let dim = get_dimensions(&graph);
    let start = find(&graph, 'S');
    let target = find(&graph, 'E');
    println!("Graph:  {} x {} ", dim.0, dim.1);
    println!("Start:  {} x {}", start.0, start.1);
    println!("Target: {} x {}", target.0, target.1);

    let dist = dijkstra(&graph, target, dim);
    println!("Distance to start: {}", dist[&key(start)]);

    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for r in 0..dim.0 {
        for c in 0..dim.1 {
            pairs.push((r, c));
        }
    }
    pairs.retain(|x| get_height(graph[x.0][x.1]) == get_height('a'));
    let to_a = pairs.iter().map(|x| dist[&key(*x)]).min().unwrap();
    println!("Distance to lowest elevation: {}", to_a);
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

fn get_dimensions(lines: &Vec<Vec<char>>) -> (usize, usize) {
    let num_rows = lines.len() as usize;
    let num_cols = lines.iter().map(|x| x.len() as usize).max().unwrap();
    return (num_rows, num_cols);
}

fn find(lines: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    return lines.iter().enumerate().fold((0, 0), |acc, (row, x)| {
        match x.iter().position(|c| *c == target) {
            Some(col) => return (row as usize, col as usize),
            _ => return acc,
        };
    });
}

fn key((r, c): (usize, usize)) -> String {
    let mut res = r.to_string();
    res.push('x');
    res.push_str(c.to_string().as_str());
    return res;
}

fn get_height(c: char) -> i32 {
    let mut ch = c;
    match c {
        'E' => ch = 'z',
        'S' => ch = 'a',
        _ => (),
    }

    return (ch as u8) as i32;
}

fn check_neighbor(
    graph: &Vec<Vec<char>>,
    neighbor: (usize, usize),
    current: (usize, usize),
    distances: &HashMap<String, usize>,
) -> (Option<usize>, bool) {
    let height = get_height(graph[current.0][current.1]);
    let neighbor_height = get_height(graph[neighbor.0][neighbor.1]);
    let neighbor_dist = distances[&key(neighbor)];
    let dist = distances[&key(current)];
    let mut reachable = false;
    let mut new_dist = None;
    if neighbor_height >= height - 1 {
        reachable = true;
        if dist + 1 < neighbor_dist {
            new_dist = Some(dist + 1);
        }
    }
    return (new_dist, reachable);
}

fn dijkstra(
    graph: &Vec<Vec<char>>,
    target: (usize, usize),
    dim: (usize, usize),
) -> HashMap<String, usize> {
    let mut curr = target;
    let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
    let mut distances: HashMap<String, usize> = HashMap::new();
    for r in 0..dim.0 {
        for c in 0..dim.1 {
            unvisited.insert((r, c));
            if curr.0 == r && curr.1 == c {
                distances.entry(key((r, c))).or_insert(0);
            } else {
                distances.entry(key((r, c))).or_insert(usize::MAX - 1);
            }
        }
    }
    let mut next: Vec<(usize, usize)> = Vec::new();

    loop {
        // Up
        if curr.0 > 0 {
            let cmp = (curr.0 - 1, curr.1);
            let (to_set, to_queue) = check_neighbor(graph, cmp, curr, &distances);
            match to_set {
                Some(new_dist) => *(distances.entry(key(cmp)).or_insert(usize::MAX - 1)) = new_dist,
                _ => (),
            };
            if to_queue && unvisited.contains(&cmp) && !next.contains(&cmp) {
                next.push(cmp);
            }
        }

        // Down
        if curr.0 < dim.0 - 1 {
            let cmp = (curr.0 + 1, curr.1);
            let (to_set, to_queue) = check_neighbor(graph, cmp, curr, &distances);
            match to_set {
                Some(new_dist) => *(distances.entry(key(cmp)).or_insert(usize::MAX - 1)) = new_dist,
                _ => (),
            };
            if to_queue && unvisited.contains(&cmp) && !next.contains(&cmp) {
                next.push(cmp);
            }
        }

        // Left
        if curr.1 > 0 {
            let cmp = (curr.0, curr.1 - 1);
            let (to_set, to_queue) = check_neighbor(graph, cmp, curr, &distances);
            match to_set {
                Some(new_dist) => *(distances.entry(key(cmp)).or_insert(usize::MAX - 1)) = new_dist,
                _ => (),
            };
            if to_queue && unvisited.contains(&cmp) && !next.contains(&cmp) {
                next.push(cmp);
            }
        }

        // Right
        if curr.1 < dim.1 - 1 {
            let cmp = (curr.0, curr.1 + 1);
            let (to_set, to_queue) = check_neighbor(graph, cmp, curr, &distances);
            match to_set {
                Some(new_dist) => *(distances.entry(key(cmp)).or_insert(usize::MAX - 1)) = new_dist,
                _ => (),
            };
            if to_queue && unvisited.contains(&cmp) && !next.contains(&cmp) {
                next.push(cmp);
            }
        }

        next.sort_by(|a, b| distances[&key(*b)].cmp(&distances[&key(*a)]));

        unvisited.remove(&curr);
        if next.len() == 0 {
            break;
        }
        curr = next.pop().unwrap();
    }

    return distances;
}
