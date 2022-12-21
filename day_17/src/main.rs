#![feature(int_roundings)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Piece {
    Horiz,
    Vert,
    Elbow,
    Cross,
    Square,
}

struct FallingPiece {
    coords: (usize, usize),
    piece_type: Piece,
}

#[derive(Debug, Clone)]
struct Chamber {
    grid: Vec<Vec<bool>>,
    max_height: isize,
    offset: isize,
}

impl Chamber {
    fn get_next_coords(&self) -> (usize, usize) {
        ((self.max_height + 4) as usize, 2)
    }

    // Assumes chamber is big enough
    fn piece_fits(&self, piece: &FallingPiece) -> bool {
        let piece_size = piece.piece_type.get_size();
        let piece_grid = piece.piece_type.get_grid();
        for i in 0..piece_size.0 {
            for j in 0..piece_size.1 {
                let y_check = piece.coords.0 + i;
                let x_check = piece.coords.1 + j;
                if self.grid[y_check][x_check] && piece_grid[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    // Assumes chamber is big enough
    fn add_piece(&mut self, piece: FallingPiece) {
        let piece_size = piece.piece_type.get_size();
        let piece_grid = piece.piece_type.get_grid();
        for i in 0..piece_size.0 {
            for j in 0..piece_size.1 {
                let y_pos = piece.coords.0 + i;
                let x_pos = piece.coords.1 + j;
                if piece_grid[i][j] {
                    self.grid[y_pos][x_pos] = true;
                    self.max_height = self.max_height.max(y_pos as isize);
                }
            }
        }
        self.setup_for_next_round()
    }

    // Add rows to the top of the grid if needed
    // If any full rows detected then trim grid
    fn setup_for_next_round(&mut self) {
        // Add new rows to top
        let grid_height = self.grid.len();
        let height_needed = self.max_height + 4 + 4;
        let extra_needed = (height_needed - grid_height as isize).max(0) as usize;
        for _ in 0..extra_needed {
            self.grid.push(vec![false; 7]);
        }
        // Trim any full rows
        let grid_height = self.grid.len();
        let mut trim_row = None;
        for i in (0..grid_height).rev() {
            if self.grid[i].iter().all(|&cell| cell) {
                trim_row = Some(i);
                break;
            }
        }
        if let Some(idx) = trim_row {
            self.offset += (idx + 1) as isize;
            self.max_height -= (idx + 1) as isize;
            self.grid = self.grid[(idx + 1)..].to_vec();
        }
    }

    fn move_to_resting(
        &self,
        mut piece: FallingPiece,
        jet_factory: &mut impl Iterator<Item = (usize, (isize, isize))>,
    ) -> FallingPiece {
        loop {
            let jet_dir = jet_factory.next().unwrap().1;
            let after_jet_coords = piece.move_in_dir(jet_dir, &self).unwrap_or(piece.coords);
            piece.coords = after_jet_coords;
            let down_dir = (-1, 0);
            let after_down_coords = piece.move_in_dir(down_dir, &self);
            match after_down_coords {
                Some(new_coords) => {
                    piece.coords = new_coords;
                    continue;
                }
                None => break piece,
            }
        }
    }

    fn get_next_falling_piece(
        &self,
        piece_factory: &mut impl Iterator<Item = Piece>,
    ) -> FallingPiece {
        FallingPiece {
            coords: self.get_next_coords(),
            piece_type: piece_factory.next().unwrap(),
        }
    }

    fn new(num_pieces: usize) -> Self {
        Self {
            grid: vec![vec![false; 7]; num_pieces * 4],
            max_height: -1,
            offset: 0,
        }
    }

    fn print(&self) {
        for line in self.grid.iter().rev() {
            for cell in line {
                if *cell {
                    print!("#");
                } else {
                    print!("*");
                }
            }
            print!("\n");
        }
    }

    fn get_rock_configuration(&self) -> Vec<Vec<bool>> {
        let empty_idx = self
            .grid
            .iter()
            .position(|row| row.iter().all(|&cell| !cell))
            .unwrap();
        self.grid[0..empty_idx].to_vec()
    }
}

impl Default for Chamber {
    fn default() -> Self {
        Self {
            grid: vec![vec![false; 7]],
            max_height: -1,
            offset: 0,
        }
    }
}

impl Piece {
    // Returns piece as bool grid from bottom to top
    fn get_grid(&self) -> Vec<Vec<bool>> {
        match self {
            Piece::Horiz => vec![vec![true; 4]],
            Piece::Cross => vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
            Piece::Elbow => vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
            Piece::Vert => vec![vec![true]; 4],
            Piece::Square => vec![vec![true; 2]; 2],
        }
    }

    fn factory() -> impl Iterator<Item = Self> {
        let order = vec![
            Self::Horiz,
            Self::Cross,
            Self::Elbow,
            Self::Vert,
            Self::Square,
        ];
        order.into_iter().cycle()
    }

    fn get_size(&self) -> (usize, usize) {
        match self {
            Piece::Horiz => (1, 4),
            Piece::Cross => (3, 3),
            Piece::Elbow => (3, 3),
            Piece::Vert => (4, 1),
            Piece::Square => (2, 2),
        }
    }
}

impl FallingPiece {
    // Attempts to move in direction dir
    // If cannot then returns None
    fn move_in_dir(&self, dir: (isize, isize), chamber: &Chamber) -> Option<(usize, usize)> {
        // Check whether it fits in the column
        let new_y = self.coords.0 as isize + dir.0;
        let new_x = self.coords.1 as isize + dir.1;
        if new_x < 0 || new_y < 0 {
            return None;
        }
        let new_x_end = new_x + (self.piece_type.get_size().1 as isize) - 1;
        if new_x_end > 6 {
            return None;
        }
        // Check whether it fits with respect to other rocks
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        let new_piece = FallingPiece {
            coords: (new_y, new_x),
            piece_type: self.piece_type,
        };
        if chamber.piece_fits(&new_piece) {
            Some((new_y, new_x))
        } else {
            None
        }
    }
}

fn get_jet_factory() -> impl Iterator<Item = (usize, (isize, isize))> {
    let file = File::open("./data/input").expect("Input file not found");
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line).unwrap();
    let jet_order: Vec<_> = line
        .trim()
        .chars()
        .enumerate()
        .map(|(idx, c)| match c {
            '>' => (idx, (0, 1)),
            '<' => (idx, (0, -1)),
            _ => panic!(),
        })
        .collect();
    jet_order.into_iter().cycle()
}

fn part_a() {
    let count: i64 = 2022;
    let mut chamber = Chamber::new(100);
    let mut piece_factory = Piece::factory().peekable();
    let mut jets = get_jet_factory().peekable();
    // Search for a repetition
    for _ in 0..count {
        let next_piece = chamber.get_next_falling_piece(&mut piece_factory);
        let ending = chamber.move_to_resting(next_piece, &mut jets);
        chamber.add_piece(ending);
    }
    println!("{}", chamber.max_height);
    println!("{}", chamber.offset);
    println!("{}", chamber.max_height + chamber.offset + 1);
}

// Horrible mess :(
fn part_b() {
    let count: i64 = 1000000000000;
    //let count: i64 = 2022;
    let mut chamber = Chamber::new(100);
    let mut piece_factory = Piece::factory().peekable();
    let mut jets = get_jet_factory().peekable();
    let mut scratchpad = HashMap::new();
    let mut repetition = None;
    let mut heights = vec![];
    // Search for a repetition
    for i in 0..count {
        // Get information about the next run
        let next_piece = chamber.get_next_falling_piece(&mut piece_factory);
        let piece_type = next_piece.piece_type;
        let scract_key = (
            chamber.get_rock_configuration(),
            piece_type,
            jets.peek().unwrap().0,
        );
        // Check to see if we've seen this before
        let insert_res = scratchpad.insert(scract_key, i);
        // If we have then we've hit a loop
        if insert_res.is_some() {
            let previous_idx = insert_res.unwrap();
            repetition = Some((previous_idx, i));
            //repetition = Some((i, chamber.max_height, chamber.offset, insert_res.unwrap()));
        }
        // Run the loop
        let ending = chamber.move_to_resting(next_piece, &mut jets);
        chamber.add_piece(ending);
        let new_height = chamber.max_height + chamber.offset;
        heights.push(new_height);
        // If we've hit a repetion then we can break
        if insert_res.is_some() {
            break;
        }
    }
    let repetition = repetition.unwrap();
    println!("{:?}", repetition);
    let cycle_len = repetition.1 - repetition.0;
    let added_height_from_cycle = heights[repetition.1 as usize] - heights[repetition.0 as usize];
    let mut base_idx = (count - 1) % cycle_len;
    let mut n_cycles = (count - 1).div_floor(cycle_len);
    if base_idx < repetition.0 {
        base_idx += cycle_len;
        n_cycles -= 1;
    }
    let total_height =
        heights[base_idx as usize] as i64 + added_height_from_cycle as i64 * n_cycles;
    println!("{}", total_height + 1);
}

fn main() {
    part_a();
    part_b();
}
