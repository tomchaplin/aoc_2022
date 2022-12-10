//use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct RegisterState {
    cycle_number: usize,
    x: i32,
}

#[derive(Debug)]
struct TubeState {
    register: RegisterState,
    crt_position: (usize, usize),
    grid: Vec<Vec<bool>>,
}

#[derive(Debug)]
enum CycleOp {
    Hang,
    AddX(i32),
}

impl Default for RegisterState {
    fn default() -> Self {
        Self {
            cycle_number: 1,
            x: 1,
        }
    }
}

impl Default for TubeState {
    fn default() -> Self {
        Self {
            register: RegisterState::default(),
            crt_position: (0, 0),
            grid: vec![vec![false; 40]; 6],
        }
    }
}

fn parse_line(line: Vec<&str>) -> Vec<CycleOp> {
    match line[0] {
        "noop" => vec![CycleOp::Hang],
        "addx" => {
            let value: i32 = line[1].parse().expect("addx value was not an integer");
            vec![CycleOp::Hang, CycleOp::AddX(value)]
        }
        _ => panic!("Bad operation"),
    }
}

fn read_lines() -> impl Iterator<Item = CycleOp> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(|line| {
            let split_line = line.split(" ").collect();
            let move_vec = parse_line(split_line);
            return move_vec.into_iter();
        })
        .flatten()
}

fn update_register(state: RegisterState, op: CycleOp) -> RegisterState {
    let cycle_number = state.cycle_number + 1;
    let x = match op {
        CycleOp::Hang => state.x,
        CycleOp::AddX(x_to_add) => state.x + x_to_add,
    };
    RegisterState { cycle_number, x }
}

fn update_tube(state: TubeState, op: CycleOp) -> TubeState {
    // Destructure
    let mut current_grid = state.grid;
    let current_pos = state.crt_position;
    // Colour in grid
    current_grid[current_pos.0][current_pos.1] =
        (state.register.x - (current_pos.1 as i32)).abs() <= 1;
    // Update position
    let mut new_pos = (current_pos.0, current_pos.1 + 1);
    if new_pos.1 >= 40 {
        new_pos.1 = 0;
        new_pos.0 += 1;
    }
    // Return
    TubeState {
        register: update_register(state.register, op),
        crt_position: new_pos,
        grid: current_grid,
    }
}

fn get_strength(state: &RegisterState) -> i32 {
    state.x * (state.cycle_number as i32)
}

fn print_grid(grid: Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            if cell {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}

fn part_a() {
    let mut op_iter = read_lines();
    // We take 19 so that internal cycle number hits 20
    let mut state = op_iter
        .by_ref()
        .take(19)
        .fold(RegisterState::default(), update_register);
    // Then take 40 five more times
    let mut sum_of_strengths: i32 = get_strength(&state);
    for _ in 0..5 {
        state = op_iter.by_ref().take(40).fold(state, update_register);
        sum_of_strengths += get_strength(&state);
    }
    println!("{}", sum_of_strengths);
}

fn part_b() {
    let op_iter = read_lines();
    let state = op_iter.fold(TubeState::default(), update_tube);
    print_grid(state.grid);
}

fn main() {
    part_a();
    part_b();
}
