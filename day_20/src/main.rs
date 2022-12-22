use std::fs::File;
use std::io::{self, BufRead};

fn read_signal() -> Vec<i64> {
    let file = File::open("./data/input").expect("Input file not found");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn handle_element(signal: &mut Vec<i64>, positions: &mut Vec<usize>, pos_idx: usize) {
    let removal_pos = positions[pos_idx];
    let signal_len = signal.len();
    // Remove signal[next_pos]
    let elem = signal.remove(removal_pos);
    // Put in new correct position in signal
    let insertion_pos = (elem + removal_pos as i64).rem_euclid(signal_len as i64 - 1);
    signal.insert(insertion_pos as usize, elem);
    // Update all positions
    for pos in positions.iter_mut() {
        if *pos > removal_pos {
            *pos -= 1;
        }
        if *pos >= insertion_pos as usize {
            *pos += 1;
        }
        *pos = pos.rem_euclid(signal_len as usize);
    }
    positions[pos_idx] = insertion_pos as usize;
}

fn run(decypt: i64, round_count: usize) -> i64 {
    // Keeps track of signal
    let mut signal = read_signal();
    let signal_len = signal.len();
    for sig in signal.iter_mut() {
        *sig = (*sig) * decypt;
    }
    // Keep tracks of the positions of original inputs
    let mut positions: Vec<usize> = (0..signal.len()).collect();
    for _ in 0..round_count {
        for pos_idx in 0..signal_len {
            handle_element(&mut signal, &mut positions, pos_idx);
        }
    }
    let zero_index = signal.iter().position(|&elem| elem == 0).unwrap();
    let mut sum = 0;
    for i in 1..=3 {
        let idx = (zero_index + i * 1000).rem_euclid(signal_len);
        let val = signal[idx];
        sum += val;
    }
    return sum;
}

fn main() {
    println!("{}", run(1, 1));
    println!("{}", run(811589153, 10));
}
