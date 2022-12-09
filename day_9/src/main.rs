use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct KnotState {
    head_pos: (i32, i32),
    tail_pos: (i32, i32),
    tail_history: Vec<(i32, i32)>,
}

#[derive(Clone, Debug)]
struct KnotChain {
    length: usize,
    knot_states: Vec<KnotState>,
}

impl Default for KnotState {
    fn default() -> Self {
        Self {
            head_pos: (0, 0),
            tail_pos: (0, 0),
            tail_history: vec![(0, 0)],
        }
    }
}

impl KnotChain {
    fn new(length: usize) -> Self {
        Self {
            length,
            knot_states: vec![KnotState::default(); length],
        }
    }
}

fn move_parser(move_str: &str) -> Move {
    match move_str {
        "U" => Move::Up,
        "D" => Move::Down,
        "L" => Move::Left,
        "R" => Move::Right,
        _ => panic!("Invalid move"),
    }
}

fn parse_line(line: Vec<&str>) -> Vec<Move> {
    let count: usize = line[1].parse().expect("Bad number of moves");
    let head_move = move_parser(line[0]);
    vec![head_move; count]
}

fn read_lines() -> impl Iterator<Item = Move> {
    let file = File::open("./data/input").unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.unwrap())
        .map(|line| {
            let split_line = line.split(" ").collect();
            let move_vec = parse_line(split_line);
            return move_vec.into_iter();
        })
        .flatten()
}

fn get_sign(input: i32) -> i32 {
    if input >= 1 {
        1
    } else if input <= -1 {
        -1
    } else {
        0
    }
}

fn get_move(discrep: (i32, i32)) -> (i32, i32) {
    let has_horiz_discrep = discrep.0 != 0;
    let has_vert_discrep = discrep.1 != 0;
    let horiz_differ_by_1 = discrep.0.abs() == 1;
    let vert_differ_by_1 = discrep.1.abs() == 1;
    // Tail should stay fixed in these scenarrios
    let overlapping = !has_horiz_discrep && !has_vert_discrep;
    let touching_laterally =
        (!has_horiz_discrep && vert_differ_by_1) || (!has_vert_discrep && horiz_differ_by_1);
    let touching_diagonally = horiz_differ_by_1 && vert_differ_by_1;
    if overlapping || touching_laterally || touching_diagonally {
        return (0, 0);
    };
    // We need to make a move, moving by 1 in each of the directions with discrep
    (get_sign(discrep.0), get_sign(discrep.1))
}

fn update_head(state: &mut KnotState, head_move: Move) {
    match head_move {
        Move::Up => state.head_pos.0 += 1,
        Move::Down => state.head_pos.0 -= 1,
        Move::Left => state.head_pos.1 -= 1,
        Move::Right => state.head_pos.1 += 1,
    }
}

fn update_tail(state: &mut KnotState) {
    let discrep = (
        state.head_pos.0 - state.tail_pos.0,
        state.head_pos.1 - state.tail_pos.1,
    );
    let tail_move = get_move(discrep);
    state.tail_pos.0 += tail_move.0;
    state.tail_pos.1 += tail_move.1;
}

fn update_history(state: &mut KnotState) {
    state.tail_history.push(state.tail_pos.clone());
}

fn update_state(mut state: KnotState, head_move: Move) -> KnotState {
    update_head(&mut state, head_move);
    update_tail(&mut state);
    update_history(&mut state);
    state
}

fn update_knot_chain(mut current_chain: KnotChain, head_move: Move) -> KnotChain {
    // First we move the head
    update_head(&mut current_chain.knot_states[0], head_move);
    update_tail(&mut current_chain.knot_states[0]);
    // Now we pass the tail of i-1 to the head of i and update the tail of i
    for i in 1..current_chain.length {
        current_chain.knot_states[i].head_pos = current_chain.knot_states[i - 1].tail_pos;
        update_tail(&mut current_chain.knot_states[i]);
    }
    update_history(&mut current_chain.knot_states[current_chain.length - 1]);
    current_chain
}

fn main() {
    let moves: Vec<Move> = read_lines().collect();

    let final_state_a = moves
        .clone()
        .into_iter()
        .fold(KnotState::default(), update_state);
    let part_a = final_state_a.tail_history.iter().unique().count();
    println!("{}", part_a);

    let final_state_b = moves.into_iter().fold(KnotChain::new(9), update_knot_chain);
    let part_b = final_state_b.knot_states[8]
        .tail_history
        .iter()
        .unique()
        .count();
    println!("{}", part_b);
}
