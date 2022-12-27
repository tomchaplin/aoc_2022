use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
    facing: usize,
}

#[derive(PartialEq, Eq)]
enum Filling {
    Valid,
    Wall,
    Invalid,
}

type Grid = HashMap<(usize, usize), Filling>;

struct Problem {
    grid: Grid,
    grid_size: (usize, usize),
    pos: Position,
}

#[derive(Clone)]
enum Move {
    Forward,
    Left,
    Right,
}

impl Position {
    fn turn_right(&mut self) {
        self.facing = (self.facing as isize + 1).rem_euclid(4) as usize;
    }

    fn turn_left(&mut self) {
        self.facing = (self.facing as isize - 1).rem_euclid(4) as usize;
    }

    // (Row velocity, Col velocity)
    fn velocity(&self) -> (isize, isize) {
        match self.facing {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => panic!("Unknown facing"),
        }
    }

    fn final_password(&self) -> usize {
        1000 * (self.row + 1) + 4 * (self.col + 1) + self.facing
    }
}

impl Problem {
    fn move_once(&mut self) {
        let vel = self.pos.velocity();
        let mut new_pos = self.pos.clone();
        loop {
            new_pos.row =
                (new_pos.row as isize + vel.0).rem_euclid(self.grid_size.0 as isize) as usize;
            new_pos.col =
                (new_pos.col as isize + vel.1).rem_euclid(self.grid_size.1 as isize) as usize;
            let grid_at_new_pos = self
                .grid
                .get(&(new_pos.row, new_pos.col))
                .unwrap_or(&Filling::Invalid);
            match *grid_at_new_pos {
                Filling::Valid => break,      // Let's move to new pos
                Filling::Wall => return,      // Can't move because of wall
                Filling::Invalid => continue, // Haven't found plase to move, loop
            }
        }
        self.pos = new_pos;
    }

    fn play_instructions<T: Iterator<Item = Move>>(&mut self, instructions: T) {
        for instruction in instructions {
            match instruction {
                Move::Forward => self.move_once(),
                Move::Left => self.pos.turn_left(),
                Move::Right => self.pos.turn_right(),
            }
        }
    }
}

fn parse_problem(lines: Vec<String>) -> Problem {
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let mut grid = HashMap::new();
    let mut initial_col = None;
    for row_idx in 0..n_rows {
        for (col_idx, c) in lines[row_idx].chars().enumerate() {
            let filling = match c {
                ' ' => Filling::Invalid,
                '.' => Filling::Valid,
                '#' => Filling::Wall,
                _ => panic!(),
            };
            if row_idx == 0 && initial_col.is_none() && filling == Filling::Valid {
                initial_col = Some(col_idx)
            };
            grid.insert((row_idx, col_idx), filling);
        }
    }
    Problem {
        grid,
        grid_size: (n_rows, n_cols),
        pos: Position {
            row: 0,
            col: initial_col.unwrap(),
            facing: 0,
        },
    }
}

fn parse_instructions(instructions: String) -> impl Iterator<Item = Move> {
    let mut moves = vec![];
    let mut working_chars: Vec<char> = vec![];
    let chars = instructions.chars();
    for c in chars {
        if c == 'L' || c == 'R' {
            // Push chars as a number of moves
            let s: String = working_chars.clone().into_iter().collect();
            let n_forwards: usize = s.parse().unwrap();
            working_chars = vec![];
            moves.extend(vec![Move::Forward; n_forwards].into_iter());
        } else {
            working_chars.push(c)
        }
        if c == 'L' {
            moves.push(Move::Left);
        }
        if c == 'R' {
            moves.push(Move::Right);
        }
    }
    // Push chars as a number of moves
    let s: String = working_chars.clone().into_iter().collect();
    let n_forwards: usize = s.parse().unwrap();
    moves.extend(vec![Move::Forward; n_forwards].into_iter());
    moves.into_iter()
}

fn read_input() -> (Problem, impl Iterator<Item = Move>) {
    let file = File::open("./data/input").expect("Input file not found");
    let mut lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let instructions = parse_instructions(lines.pop().unwrap());
    lines.pop();
    let problem = parse_problem(lines);
    (problem, instructions)
}

fn main() {
    let (mut problem, instructions) = read_input();
    problem.play_instructions(instructions);
    let final_password = problem.pos.final_password();
    println!("{}", final_password);
}
