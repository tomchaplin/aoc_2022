use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

struct Accumulator {
    max_height: i32,
    visible_trees: Vec<(usize, usize)>,
}

impl Default for Accumulator {
    fn default() -> Self {
        Self {
            max_height: -1,
            visible_trees: vec![],
        }
    }
}

fn read_lines() -> Vec<Vec<i32>> {
    let file = File::open("./data/input").unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.unwrap())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| i32::try_from(c.to_digit(10).unwrap()).unwrap())
                .collect()
        })
        .collect()
}

// Returns indicies of visible trees from this direction
fn get_visible_from_iter<'a>(
    heights: impl Iterator<Item = ((usize, usize), &'a i32)>,
) -> Vec<(usize, usize)> {
    heights
        .fold(Default::default(), |accum: Accumulator, (index, height)| {
            if *height > accum.max_height {
                let mut visible_trees = accum.visible_trees;
                visible_trees.push(index);
                Accumulator {
                    max_height: height.clone(),
                    visible_trees,
                }
            } else {
                accum
            }
        })
        .visible_trees
}

fn get_num_visible(board: &Vec<Vec<i32>>) -> usize {
    let mut visible_trees: Vec<(usize, usize)> = vec![];
    // Add trees visible in each row
    for (row_index, row) in board.iter().enumerate() {
        let indexed_iterator = row
            .iter()
            .enumerate()
            .map(|(col_index, height)| ((row_index, col_index), height));
        let mut visible_from_left = get_visible_from_iter(indexed_iterator.clone());
        let mut visible_from_right = get_visible_from_iter(indexed_iterator.rev());
        visible_trees.append(&mut visible_from_left);
        visible_trees.append(&mut visible_from_right);
    }
    // Add trees visible in each column
    let row_size = board[0].len();
    for col_index in 0..row_size {
        let indexed_iterator = board
            .iter()
            .enumerate()
            .map(|(row_index, row)| ((row_index, col_index), &row[col_index]));
        let mut visible_from_top = get_visible_from_iter(indexed_iterator.clone());
        let mut visible_from_bottom = get_visible_from_iter(indexed_iterator.rev());
        visible_trees.append(&mut visible_from_top);
        visible_trees.append(&mut visible_from_bottom);
    }
    visible_trees.into_iter().unique().count()
}

fn compute_view_distance<'a>(iterator: impl Iterator<Item = &'a i32>, house_height: i32) -> usize {
    let as_vec: Vec<&i32> = iterator.collect();
    let size = as_vec.len();
    let first_position = as_vec
        .into_iter()
        .position(|height| *height >= house_height);
    // If first_position was 0 then we can see 1 tree
    match first_position {
        Some(first_position) => first_position + 1,
        None => size,
    }
}

// TODO: Could short-circuit to 0 if any score is 0?
fn compute_scenic_score(board: &Vec<Vec<i32>>, pos: (usize, usize)) -> usize {
    let col_size = board.len();
    let row_size = board[0].len();
    let row = pos.0;
    let col = pos.1;
    let house_height = board[row][col];
    let left_score = compute_view_distance(
        (0..col).rev().map(|col_index| &board[row][col_index]),
        house_height,
    );
    let right_score = if col < col_size - 1 {
        compute_view_distance(
            (col + 1..col_size).map(|col_index| &board[row][col_index]),
            house_height,
        )
    } else {
        0
    };
    let up_score = compute_view_distance(
        (0..row).rev().map(|row_index| &board[row_index][col]),
        house_height,
    );
    let down_score = if row < row_size - 1 {
        compute_view_distance(
            (row + 1..row_size).map(|row_index| &board[row_index][col]),
            house_height,
        )
    } else {
        0
    };
    left_score * right_score * up_score * down_score
}

fn compute_max_score(board: &Vec<Vec<i32>>) -> usize {
    let col_size = board.len();
    let row_size = board[0].len();
    let index_iteator = (0..row_size)
        .map(|row| (0..col_size).map(move |col| (row, col)))
        .flatten();
    index_iteator
        .map(|pos| compute_scenic_score(board, pos))
        .max()
        .unwrap()
}

fn main() {
    let board = read_lines();
    let visible = get_num_visible(&board);
    println!("{}", visible);
    let max_score = compute_max_score(&board);
    println!("{}", max_score);
}
