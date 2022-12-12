use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> Vec<char> {
    let file = File::open("./data/input").unwrap();
    let mut buffer = BufReader::new(file);
    let mut first_line = String::new();
    let _ = buffer.read_line(&mut first_line);
    first_line.trim().chars().collect()
}

fn find_packet(chars: &Vec<char>, w_size: usize) -> usize {
    let first_marker = chars
        .as_slice()
        .windows(w_size)
        .enumerate()
        .find(|(_, window)| all_unique(window, w_size))
        .unwrap();
    first_marker.0 + w_size
}

fn all_unique(window: &[char], w_size: usize) -> bool {
    window.to_vec().iter().unique().count() == w_size
}

fn main() {
    let chars = read_file();
    println!("{}", find_packet(&chars, 4));
    println!("{}", find_packet(&chars, 14));
}
