use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct State {
    head_pos: (isize, isize),
    tail_pos: (isize, isize),
    tail_history: Vec<(isize, isize)>,
}

#[derive(Clone, Debug)]
struct KnotChain {
    length: usize,
    knot_states: Vec<State>,
}

impl KnotChain {
    fn new(length: usize) -> Self {
        Self {
            length,
            knot_states: vec![State::default(); length],
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            head_pos: (0, 0),
            tail_pos: (0, 0),
            tail_history: vec![(0, 0)],
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
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
    let count: isize = line[1].parse().expect("Bad number of moves");
    let head_move = move_parser(line[0]);
    vec![head_move; count as usize]
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

fn get_sign(input: isize) -> isize {
    if input >= 1 {
        1
    } else if input <= -1 {
        -1
    } else {
        0
    }
}

fn get_move(discrep: (isize, isize)) -> (isize, isize) {
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

fn update_tail(mut state: State) -> State {
    let discrep = (
        state.head_pos.0 - state.tail_pos.0,
        state.head_pos.1 - state.tail_pos.1,
    );
    let tail_move = get_move(discrep);
    state.tail_pos.0 += tail_move.0;
    state.tail_pos.1 += tail_move.1;
    state.tail_history.push(state.tail_pos.clone());
    state
}

fn update_state(mut current_state: State, head_move: Move) -> State {
    match head_move {
        Move::Up => current_state.head_pos.0 += 1,
        Move::Down => current_state.head_pos.0 -= 1,
        Move::Left => current_state.head_pos.1 -= 1,
        Move::Right => current_state.head_pos.1 += 1,
    }
    current_state = update_tail(current_state);
    return current_state;
}

// TODO: Avoid cloning in this function
fn update_knot_chain(mut current_chain: KnotChain, head_move: Move) -> KnotChain {
    // First we move the head
    current_chain.knot_states[0] = update_state(current_chain.knot_states[0].clone(), head_move);
    // Now we pass the tail of i-1 to the head of i and update the tail of i
    for i in 1..current_chain.length {
        current_chain.knot_states[i].head_pos = current_chain.knot_states[i - 1].tail_pos;
        current_chain.knot_states[i] = update_tail(current_chain.knot_states[i].clone());
    }
    current_chain
}

fn main() {
    let moves: Vec<Move> = read_lines().collect();

    let final_state_a = moves
        .clone()
        .into_iter()
        .fold(State::default(), update_state);
    let part_a = final_state_a.tail_history.into_iter().unique().count();
    println!("{}", part_a);

    let final_state_b = moves.into_iter().fold(KnotChain::new(9), update_knot_chain);
    let part_b = final_state_b.knot_states[8]
        .tail_history
        .iter()
        .unique()
        .count();
    println!("{}", part_b);
}
