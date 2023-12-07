use advent_of_code::utils::input::read_lines;
use clap::Parser;
use std::cmp::Ordering;

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

    let pt1: i32 = handle_pt1(&lines);
    // 247766195 too low
    // 247775171 too low
    // 247415731 too low
    println!("Part 1: {}", pt1);
    let pt2: i32 = handle_pt2(&lines);
    println!("Part 2: {}", pt2);
}

#[derive(Eq, PartialEq, Hash)]
pub struct Hand {
    display: String,
    cards: Vec<i32>,
    score: i32,
    bid: i32,
}

fn card_weight(c: char) -> i32 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn sorted_frequency(hand: &Vec<char>) -> Vec<(char, i32)> {
    let mut sorted_hand = hand.clone();
    sorted_hand.sort();

    let mut res: Vec<(char, i32)> = Vec::new();
    let mut running_count: i32 = 1;
    let mut current_card = *sorted_hand.iter().nth(0).unwrap();
    for i in 1..sorted_hand.len() {
        let c = sorted_hand[i];
        if c == current_card {
            running_count += 1;
        } else {
            res.push((current_card, running_count));
            running_count = 1;
            current_card = c;
        }
    }
    res.push((current_card, running_count));

    res.sort_by(|(_, count1), (_, count2)| count1.cmp(count2));
    res.reverse();

    res
}

fn sorted_frequency_2(hand: &Vec<char>) -> Vec<(char, i32)> {
    let mut sorted_hand = hand.clone();
    sorted_hand.sort();

    let mut res: Vec<(char, i32)> = Vec::new();
    let mut running_count: i32 = 1;
    let mut current_card = None;
    let mut joker_count = 0;
    for i in 0..sorted_hand.len() {
        let c = sorted_hand[i];
        match c {
            'J' => joker_count += 1,
            _ => {
                if let Some(current_c) = current_card {
                    if c == current_c {
                        running_count += 1;
                    } else {
                        res.push((current_c, running_count));
                        running_count = 1;
                        current_card = Some(c);
                    }
                } else {
                    current_card = Some(c);
                    running_count = 1;
                }
            }
        }
    }
    if let Some(current_c) = current_card {
        res.push((current_c, running_count));
    }

    res.sort_by(|(_, count1), (_, count2)| count1.cmp(count2));
    res.reverse();

    if res.len() > 0 {
        let (top, top_count) = res[0];
        res[0] = (top, top_count + joker_count);
    } else {
        res.push(('A', joker_count));
    }

    res
}

fn determine_type(hand: &Vec<char>) -> i32 {
    let frequencies = sorted_frequency(hand);
    match frequencies.len() {
        1 => return FIVES,
        2 => match frequencies.iter().nth(0).unwrap() {
            (_, 4) => return FOURS,
            _ => return FULL_HOUSE,
        },
        3 => match (
            frequencies.iter().nth(0).unwrap(),
            frequencies.iter().nth(1).unwrap(),
        ) {
            ((_, 2), (_, 2)) => return TWO_PAIRS,
            _ => return THREES,
        },
        4 => return PAIR,
        5 => return HIGH,
        _ => todo!(),
    }
}

fn determine_type_2(hand: &Vec<char>) -> i32 {
    let frequencies = sorted_frequency_2(hand);
    match frequencies.len() {
        1 => return FIVES,
        2 => match frequencies.iter().nth(0).unwrap() {
            (_, 4) => return FOURS,
            _ => return FULL_HOUSE,
        },
        3 => {
            match (
                frequencies.iter().nth(0).unwrap(),
                frequencies.iter().nth(1).unwrap(),
            ) {
                ((_, 2), (_, 2)) => return TWO_PAIRS,
                _ => return THREES,
            }
        }
        4 => return PAIR,
        5 => return HIGH,
        _ => todo!(),
    }
}

type HandType = i32;
const FIVES: HandType = 6;
const FOURS: HandType = 5;
const FULL_HOUSE: HandType = 4;
const THREES: HandType = 3;
const TWO_PAIRS: HandType = 2;
const PAIR: HandType = 1;
const HIGH: HandType = 0;

impl Hand {
    pub fn new(hand: Vec<char>, bid: i32) -> Self {
        let score = determine_type(&hand);
        Hand {
            display: hand.clone().into_iter().collect::<String>(),
            cards: hand.iter().map(|x| card_weight(*x)).collect::<Vec<i32>>(),
            score: score,
            bid: bid,
        }
    }

    pub fn new2(hand: Vec<char>, bid: i32) -> Self {
        let score = determine_type_2(&hand);
        Hand {
            display: hand.clone().into_iter().collect::<String>(),
            cards: hand.iter().map(|x| wild_weight(*x)).collect::<Vec<i32>>(),
            score: score,
            bid: bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.score.cmp(&other.score);
        match cmp {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let second_cmp = self.cards[i].cmp(&other.cards[i]);
                    match second_cmp {
                        Ordering::Equal => {}
                        _ => return second_cmp,
                    };
                }
                return Ordering::Equal;
            }
            _ => return cmp,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn handle_pt1(lines: &Vec<String>) -> i32 {
    let mut hands = lines
        .iter()
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<&str>>();
            Hand::new(
                parts[0].chars().collect::<Vec<char>>(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.reverse();

    let tot = hands.len();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = tot as i32 - i as i32;
            let value = rank * hand.bid;
            value
        })
        .sum()
}

fn wild_weight(c: char) -> i32 {
    match c {
        'J' => -1,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn handle_pt2(lines: &Vec<String>) -> i32 {
    let mut hands = lines
        .iter()
        .map(|l| {
            let parts = l.split(' ').collect::<Vec<&str>>();
            let cards = parts[0].chars().collect::<Vec<char>>();
            Hand::new2(cards, parts[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<Hand>>();
    hands.sort();
    hands.reverse();

    let tot = hands.len();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = tot as i32 - i as i32;
            let value = rank * hand.bid;
            value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_pt1() {
        let tests = [
            (
                vec![String::from("KK677 28"), String::from("KTJJT 220")],
                28 * 2 + 220,
            ),
            (
                vec![
                    String::from("32T3K 765"),
                    String::from("T55J5 684"),
                    String::from("KK677 28"),
                    String::from("KTJJT 220"),
                    String::from("QQQJA 483"),
                ],
                6440,
            ),
            (
                vec![
                    String::from("333KK 330"),
                    String::from("333JJ 134"),
                    String::from("33399 711"),
                ],
                711 + 134 * 2 + 330 * 3,
            ),
            (
                vec![
                    String::from("34568 427"),
                    String::from("24569 115"),
                    String::from("2346T 27"),
                    String::from("23469 754"),
                    String::from("23459 675"),
                ],
                427 * 5 + 115 * 4 + 27 * 3 + 754 * 2 + 675,
            ),
        ];

        for (input, want) in tests {
            assert_eq!(handle_pt1(&input), want, "for input\n{}", input.join("\n"));
        }
    }

    #[test]
    fn test_parsing_pt2() {
        let tests = [(
            vec![
                String::from("32T3K 765"),
                String::from("T55J5 684"),
                String::from("KK677 28"),
                String::from("KTJJT 220"),
                String::from("QQQJA 483"),
            ],
            5905,
        )];

        for (input, want) in tests {
            assert_eq!(handle_pt2(&input), want, "for input\n{}", input.join("\n"));
        }
    }
}
