use advent_of_code::utils::input::read_lines;
use clap::Parser;
use regex::Regex;
use std::fmt;

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

    let pt1: i64 = handle_pt1(&lines);
    println!("Part 1: {}", pt1);
    let pt2: i64 = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Translation {
    source: i64,
    destination: i64,
    size: i64,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn get_overlap(&self, r: Range) -> Option<Range> {
        let maybe_res = Range {
            start: *[self.start, r.start].iter().max().unwrap(),
            end: *[self.end, r.end].iter().min().unwrap(),
        };
        if maybe_res.end < maybe_res.start {
            return None;
        }

        return Some(maybe_res);
    }

    pub fn get_non_overlap(&self, r: Range) -> Vec<Range> {
        let mut res: Vec<Range> = Vec::new();
        for x in [
            Range {
                start: *[self.start, r.start].iter().min().unwrap(),
                end: *[self.start, r.start].iter().max().unwrap() - 1,
            },
            Range {
                start: *[self.end, r.end].iter().min().unwrap() + 1,
                end: *[self.end, r.end].iter().max().unwrap(),
            },
        ] {
            if x.start <= x.end && x.start >= self.start && x.end <= self.end {
                res.push(x);
            }
        }
        res
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}
pub struct Map {
    from: String,
    to: String,
    translations: Vec<Translation>,
}

fn translate_seed(seed: i64, map: &Vec<Map>) -> i64 {
    let mut value = seed;
    let mut source = "seed";
    while source != "location" {
        let mut acted = false;
        for m in map {
            if m.from.as_str() == source {
                for t in m.translations.iter() {
                    if value >= t.source && value < t.source + t.size {
                        value = t.destination + (value - t.source);
                        break;
                    }
                }

                source = m.to.as_str();
                acted = true;
            }
        }
        assert!(acted);
    }
    value
}

fn handle_pt1(lines: &Vec<String>) -> i64 {
    let mut seeds: Vec<i64> = Vec::new();
    let mut map: Vec<Map> = Vec::new();

    let mut from = String::from("");
    let mut to = String::from("");
    let mut translations: Vec<Translation> = Vec::new();

    let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let translation_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    for line in lines {
        if let Some(seeds_str) = line.strip_prefix("seeds: ") {
            seeds = seeds_str
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            continue;
        }

        if line.is_empty() && !from.as_str().is_empty() {
            map.push(Map {
                from: from.clone(),
                to: to.clone(),
                translations: translations.clone(),
            });

            translations = Vec::new();
            continue;
        }

        for m in map_re.captures_iter(line) {
            for (i, capt) in m.iter().enumerate() {
                if let Some(sub) = capt {
                    match i {
                        1 => from = String::from(sub.as_str()),
                        2 => to = String::from(sub.as_str()),
                        _ => {}
                    }
                }
            }
        }

        if translation_re.is_match(line) {
            let mut translation = Translation {
                destination: 0,
                source: 0,
                size: 0,
            };
            for m in translation_re.captures_iter(line) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        match i {
                            1 => translation.destination = sub.as_str().parse::<i64>().unwrap(),
                            2 => translation.source = sub.as_str().parse::<i64>().unwrap(),
                            3 => translation.size = sub.as_str().parse::<i64>().unwrap(),
                            _ => {}
                        }
                    }
                }
            }
            translations.push(translation);
        }
    }
    map.push(Map {
        from: from.clone(),
        to: to.clone(),
        translations: translations.clone(),
    });

    let mut min_location = None;
    for seed in seeds {
        let location = translate_seed(seed, &map);
        if let Some(mlocation) = min_location {
            if mlocation > location {
                min_location = Some(location);
            }
        } else {
            min_location = Some(location);
        }
    }

    min_location.unwrap()
}

fn translate_seed_range(seed: i64, seed_range: i64, map: &Vec<Map>) -> i64 {
    let mut values = vec![Range {
        start: seed,
        end: seed + seed_range - 1,
    }];
    let mut source = "seed";
    while source != "location" {
        let mut acted = false;
        for m in map {
            if m.from.as_str() == source {
                //  println!(
                //      "translating {} to {}: {}",
                //      m.from.as_str(),
                //      m.to.as_str(),
                //      values[0]
                //  );
                let mut translated: Vec<Range> = Vec::new();
                while !values.is_empty() {
                    let v_range = values.pop().unwrap();
                    let mut did = false;
                    for t in m.translations.iter() {
                        let t_range = Range {
                            start: t.source,
                            end: t.source + t.size - 1,
                        };

                        if let Some(overlap) = v_range.get_overlap(t_range) {
                            //  println!("{} and {} overlap = {}", v_range, t_range, overlap);
                            translated.push(Range {
                                start: t.destination + (overlap.start - t.source),
                                end: t.destination + (overlap.end - t.source),
                            });
                            for non_overlap in v_range.get_non_overlap(t_range) {
                                values.push(non_overlap);
                                //   println!("- with not overlapping {}", non_overlap);
                            }
                            did = true;
                            break;
                        }
                    }
                    if !did {
                        translated.push(v_range);
                    }
                }

                values = translated.clone();
                source = m.to.as_str();
                acted = true;
            }
        }
        assert!(acted);
    }
    values.iter().map(|x| x.start).min().unwrap()
}

fn handle_pt2(lines: &Vec<String>) -> i64 {
    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
    let mut map: Vec<Map> = Vec::new();

    let mut from = String::from("");
    let mut to = String::from("");
    let mut translations: Vec<Translation> = Vec::new();

    let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
    let translation_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let seed_re = Regex::new(r"(\d+ \d+)+").unwrap();
    for line in lines {
        if let Some(seeds_str) = line.strip_prefix("seeds: ") {
            for m in seed_re.captures_iter(seeds_str) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        if i == 0 {
                            let parts = sub.as_str().split(" ").collect::<Vec<&str>>();
                            let seed_start = parts[0].parse::<i64>().unwrap();
                            let seed_range = parts[1].parse::<i64>().unwrap();
                            seed_ranges.push((seed_start, seed_range));
                            //  println!("seed range: {} {}", seed_start, seed_range);
                        }
                    }
                }
            }
            continue;
        }

        if line.is_empty() && !from.as_str().is_empty() {
            map.push(Map {
                from: from.clone(),
                to: to.clone(),
                translations: translations.clone(),
            });

            translations = Vec::new();
            continue;
        }

        for m in map_re.captures_iter(line) {
            for (i, capt) in m.iter().enumerate() {
                if let Some(sub) = capt {
                    match i {
                        1 => from = String::from(sub.as_str()),
                        2 => to = String::from(sub.as_str()),
                        _ => {}
                    }
                }
            }
        }

        if translation_re.is_match(line) {
            let mut translation = Translation {
                destination: 0,
                source: 0,
                size: 0,
            };
            for m in translation_re.captures_iter(line) {
                for (i, capt) in m.iter().enumerate() {
                    if let Some(sub) = capt {
                        match i {
                            1 => translation.destination = sub.as_str().parse::<i64>().unwrap(),
                            2 => translation.source = sub.as_str().parse::<i64>().unwrap(),
                            3 => translation.size = sub.as_str().parse::<i64>().unwrap(),
                            _ => {}
                        }
                    }
                }
            }
            translations.push(translation);
        }
    }
    map.push(Map {
        from: from.clone(),
        to: to.clone(),
        translations: translations.clone(),
    });

    let mut min_location = None;
    for (start, size) in seed_ranges {
        let location = translate_seed_range(start, size, &map);
        // println!("seed {} has location {}", seed, location);
        if let Some(mlocation) = min_location {
            if mlocation > location {
                min_location = Some(location);
            }
        } else {
            min_location = Some(location);
        }
    }

    min_location.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [(
            vec![
                String::from("seeds: 79 14 55 13"),
                String::from(""),
                String::from("seed-to-soil map:"),
                String::from("50 98 2"),
                String::from("52 50 48"),
                String::from(""),
                String::from("soil-to-fertilizer map:"),
                String::from("0 15 37"),
                String::from("37 52 2"),
                String::from("39 0 15"),
                String::from(""),
                String::from("fertilizer-to-water map:"),
                String::from("49 53 8"),
                String::from("0 11 42"),
                String::from("42 0 7"),
                String::from("57 7 4"),
                String::from(""),
                String::from("water-to-light map:"),
                String::from("88 18 7"),
                String::from("18 25 70"),
                String::from(""),
                String::from("light-to-temperature map:"),
                String::from("45 77 23"),
                String::from("81 45 19"),
                String::from("68 64 13"),
                String::from(""),
                String::from("temperature-to-humidity map:"),
                String::from("0 69 1"),
                String::from("1 0 69"),
                String::from(""),
                String::from("humidity-to-location map:"),
                String::from("60 56 37"),
                String::from("56 93 4"),
            ],
            35,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_overlap() {
        let tests = [
            (
                (Range { start: 10, end: 15 }, Range { start: 11, end: 20 }),
                Some(Range { start: 11, end: 15 }),
                vec![Range { start: 10, end: 10 }],
            ),
            (
                (Range { start: 12, end: 15 }, Range { start: 11, end: 20 }),
                Some(Range { start: 12, end: 15 }),
                vec![],
            ),
            (
                (Range { start: 12, end: 15 }, Range { start: 12, end: 20 }),
                Some(Range { start: 12, end: 15 }),
                vec![],
            ),
            (
                (Range { start: 12, end: 15 }, Range { start: 11, end: 15 }),
                Some(Range { start: 12, end: 15 }),
                vec![],
            ),
        ];
        for ((a, b), overlap, non_overlap) in tests {
            assert_eq!(a.get_overlap(b), overlap);
            assert_eq!(a.get_non_overlap(b), non_overlap);
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [(
            vec![
                String::from("seeds: 79 14 55 13"),
                String::from(""),
                String::from("seed-to-soil map:"),
                String::from("50 98 2"),
                String::from("52 50 48"),
                String::from(""),
                String::from("soil-to-fertilizer map:"),
                String::from("0 15 37"),
                String::from("37 52 2"),
                String::from("39 0 15"),
                String::from(""),
                String::from("fertilizer-to-water map:"),
                String::from("49 53 8"),
                String::from("0 11 42"),
                String::from("42 0 7"),
                String::from("57 7 4"),
                String::from(""),
                String::from("water-to-light map:"),
                String::from("88 18 7"),
                String::from("18 25 70"),
                String::from(""),
                String::from("light-to-temperature map:"),
                String::from("45 77 23"),
                String::from("81 45 19"),
                String::from("68 64 13"),
                String::from(""),
                String::from("temperature-to-humidity map:"),
                String::from("0 69 1"),
                String::from("1 0 69"),
                String::from(""),
                String::from("humidity-to-location map:"),
                String::from("60 56 37"),
                String::from("56 93 4"),
            ],
            46,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
