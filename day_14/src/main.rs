use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct GameState {
    grid: HashMap<(isize, isize), Contents>,
    max_y: isize,
    num_sand: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            grid: Default::default(),
            max_y: 0,
            num_sand: 0,
        }
    }
}

enum Contents {
    Rock,
    Sand,
}

type RockPath = Vec<(isize, isize)>;

impl GameState {
    fn get_contents_a(&self, pos: &(isize, isize)) -> Option<&Contents> {
        self.grid.get(pos)
    }

    fn get_contents_b(&self, pos: &(isize, isize)) -> Option<&Contents> {
        if pos.1 >= self.max_y + 2 {
            Some(&Contents::Rock)
        } else {
            self.grid.get(pos)
        }
    }

    fn add_sand(&mut self, pos: (isize, isize)) {
        self.grid.insert(pos, Contents::Sand);
        self.num_sand += 1;
    }

    fn add_rock(&mut self, pos: (isize, isize)) {
        if pos.1 > self.max_y {
            self.max_y = pos.1
        }
        self.grid.insert(pos, Contents::Rock);
    }

    fn add_rock_path(&mut self, path: RockPath) {
        path.as_slice().windows(2).for_each(|piece| {
            let start = piece.iter().nth(0).unwrap();
            let end = piece.iter().nth(1).unwrap();
            if end.0 > start.0 {
                let x_diff = end.0 - start.0;
                for x_offset in 0..=x_diff {
                    self.add_rock((start.0 + x_offset, start.1));
                }
            } else {
                let x_diff = start.0 - end.0;
                for x_offset in 0..=x_diff {
                    self.add_rock((end.0 + x_offset, end.1));
                }
            }
            if end.1 > start.1 {
                let y_diff = end.1 - start.1;
                for y_offset in 0..=y_diff {
                    self.add_rock((start.0, start.1 + y_offset));
                }
            } else {
                let y_diff = start.1 - end.1;
                for y_offset in 0..=y_diff {
                    self.add_rock((end.0, end.1 + y_offset));
                }
            }
        });
    }

    fn in_abyss(&self, pos: &(isize, isize)) -> bool {
        pos.1 > self.max_y
    }

    // Given sand is at current current pos, what direction does sand go in?
    // None if stays put
    fn determine_direction_a(&self, pos: &(isize, isize)) -> Option<(isize, isize)> {
        if self.get_contents_a(&(pos.0, pos.1 + 1)).is_none() {
            Some((0, 1))
        } else if self.get_contents_a(&(pos.0 - 1, pos.1 + 1)).is_none() {
            Some((-1, 1))
        } else if self.get_contents_a(&(pos.0 + 1, pos.1 + 1)).is_none() {
            Some((1, 1))
        } else {
            None
        }
    }

    // Returns where a sand particle starting at pos will come to rest
    // Returns None if falls into abyss
    fn determine_sand_fate_a(&self, starting_pos: (isize, isize)) -> Option<(isize, isize)> {
        let mut working_pos = starting_pos;
        while let Some(dir) = self.determine_direction_a(&working_pos) {
            working_pos.0 += dir.0;
            working_pos.1 += dir.1;
            if self.in_abyss(&working_pos) {
                return None;
            }
        }
        Some(working_pos)
    }

    // Given sand is at current current pos, what direction does sand go in?
    // None if stays put
    fn determine_direction_b(&self, pos: &(isize, isize)) -> Option<(isize, isize)> {
        if self.get_contents_b(&(pos.0, pos.1 + 1)).is_none() {
            Some((0, 1))
        } else if self.get_contents_b(&(pos.0 - 1, pos.1 + 1)).is_none() {
            Some((-1, 1))
        } else if self.get_contents_b(&(pos.0 + 1, pos.1 + 1)).is_none() {
            Some((1, 1))
        } else {
            None
        }
    }

    // Returns where a sand particle starting at pos will come to rest
    // Returns None if falls into abyss
    // Since we have infinite floor this should always return
    fn determine_sand_fate_b(&self, starting_pos: (isize, isize)) -> Option<(isize, isize)> {
        let mut working_pos = starting_pos;
        while let Some(dir) = self.determine_direction_b(&working_pos) {
            working_pos.0 += dir.0;
            working_pos.1 += dir.1;
        }
        Some(working_pos)
    }
}

fn read_paths() -> impl Iterator<Item = RockPath> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());
    lines.map(|line| {
        line.split(" -> ")
            .map(|line_as_iter| {
                let entries: Vec<&str> = line_as_iter.split(",").collect();
                (entries[0].parse().unwrap(), entries[1].parse().unwrap())
            })
            .collect()
    })
}

fn part_a() {
    // Build board
    let paths = read_paths();
    let mut board = GameState::default();
    for path in paths {
        board.add_rock_path(path);
    }
    // Start adding sand
    for _ in 0.. {
        let next_sand_pos = board.determine_sand_fate_a((500, 0));
        match next_sand_pos {
            Some(pos) => {
                board.add_sand(pos);
            }
            None => {
                break;
            }
        }
    }
    println!("{}", board.num_sand);
}

fn part_b() {
    // Build board
    let paths = read_paths();
    let mut board = GameState::default();
    for path in paths {
        board.add_rock_path(path);
    }
    // Start adding sand
    for _ in 0.. {
        let next_sand_pos = board.determine_sand_fate_b((500, 0)).unwrap();
        board.add_sand(next_sand_pos);
        if next_sand_pos == (500, 0) {
            break;
        }
    }
    println!("{}", board.num_sand);
}

fn main() {
    part_a();
    part_b();
}
