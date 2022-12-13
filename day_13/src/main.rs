use std::fs::File;
use std::io::{self, BufRead};
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
enum SignalType {
    Num(u32),
    Array(Vec<SignalType>),
}

type SignalPair = (SignalType, SignalType);

fn ensure_array(num: SignalType) -> SignalType {
    match num {
        SignalType::Num(n) => SignalType::Array(vec![SignalType::Num(n)]),
        array => array,
    }
}

fn in_correct_order(pair: SignalPair) -> Option<bool> {
    match pair {
        (SignalType::Num(x), SignalType::Num(y)) => {
            if x < y {
                Some(true)
            } else if x > y {
                Some(false)
            } else {
                None
            }
        }
        (SignalType::Array(x), SignalType::Array(y)) => {
            let x_len = x.len();
            let y_len = y.len();
            for (x_entry, y_entry) in x.into_iter().zip(y.into_iter()) {
                if let Some(res) = in_correct_order((x_entry, y_entry)) {
                    return Some(res);
                }
            }
            if x_len < y_len {
                Some(true)
            } else if x_len > y_len {
                Some(false)
            } else {
                None
            }
        }
        (x, y) => {
            let x_array = ensure_array(x);
            let y_array = ensure_array(y);
            return in_correct_order((x_array, y_array));
        }
    }
}

// This is horrendous, sorry
fn parse_one_signal(signal: &mut Chars) -> Option<SignalType> {
    let mut next_char = signal.next().unwrap();
    if next_char == ',' {
        next_char = signal.next().unwrap();
    }
    if next_char.is_digit(10) {
        let mut num_chars: Vec<char> = vec![next_char];
        loop {
            next_char = signal.next().unwrap();
            if next_char.is_digit(10) {
                num_chars.push(next_char);
            } else {
                // This will always terminate on a , because we replaced ] with ,]
                break;
            }
        }
        let num_str: String = num_chars.into_iter().collect();
        let num: u32 = num_str.parse().unwrap();
        return Some(SignalType::Num(num));
    }
    if next_char == ']' {
        return None;
    }
    // next_char must be [ so we try and parse an array
    let mut ret_signal = vec![];
    while let Some(next_sig) = parse_one_signal(signal) {
        ret_signal.push(next_sig)
    }
    return Some(SignalType::Array(ret_signal));
}

// I replace ] with ,] so that I can detect the end of an integer when parsing char by char
// without consuming the end bracket
// Gross, I know
fn parse_signal(signal_str: &String) -> SignalType {
    parse_one_signal(&mut signal_str.replace("]", ",]").chars()).unwrap()
}

fn read_pairs() -> Vec<SignalPair> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());
    let lines_vec: Vec<String> = lines.collect();
    lines_vec
        .split(|line| line.is_empty())
        .map(|line_group| (parse_signal(&line_group[0]), parse_signal(&line_group[1])))
        .collect()
}

fn read_all() -> Vec<SignalType> {
    let file = File::open("./data/input").expect("Input file not found");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| parse_signal(&line))
        .collect()
}

fn part_a() {
    let pairs = read_pairs();
    let good_idxs: Vec<usize> = pairs
        .into_iter()
        .enumerate()
        .map(|(idx, pair)| (idx, in_correct_order(pair)))
        .filter(|(_, cor)| cor.unwrap())
        .map(|(idx, _)| idx + 1)
        .collect();
    let sum_of_idxs: usize = good_idxs.into_iter().sum();
    println!("{}", sum_of_idxs);
}

fn part_b() {
    let mut signals = read_all();
    let div_pack_1 = SignalType::Array(vec![SignalType::Array(vec![SignalType::Num(2)])]);
    let div_pack_2 = SignalType::Array(vec![SignalType::Array(vec![SignalType::Num(6)])]);
    signals.push(div_pack_1.clone());
    signals.push(div_pack_2.clone());
    signals.sort_by(|a, b| {
        if in_correct_order((a.clone(), b.clone())).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let idx_1 = signals.iter().position(|elem| *elem == div_pack_1).unwrap() + 1;
    let idx_2 = signals.iter().position(|elem| *elem == div_pack_2).unwrap() + 1;
    println!("{}", idx_1 * idx_2);
}

fn main() {
    part_a();
    part_b();
}
