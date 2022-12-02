use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Target {
    Win,
    Draw,
    Lose,
}

fn abc_parser(letter: char) -> Move {
    match letter {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!(),
    }
}

fn xyz_parser(letter: char) -> Move {
    match letter {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!(),
    }
}

fn xyz_parser2(letter: char) -> Target {
    match letter {
        'X' => Target::Lose,
        'Y' => Target::Draw,
        'Z' => Target::Win,
        _ => panic!(),
    }
}

fn determine_my_move(game: (Move, Target)) -> (Move, Move) {
    let my_move = match &game {
        (Move::Rock, Target::Win) => Move::Paper,
        (Move::Rock, Target::Draw) => Move::Rock,
        (Move::Rock, Target::Lose) => Move::Scissors,
        (Move::Paper, Target::Win) => Move::Scissors,
        (Move::Paper, Target::Draw) => Move::Paper,
        (Move::Paper, Target::Lose) => Move::Rock,
        (Move::Scissors, Target::Win) => Move::Rock,
        (Move::Scissors, Target::Draw) => Move::Scissors,
        (Move::Scissors, Target::Lose) => Move::Paper,
    };
    (game.0, my_move)
}

fn score_game(game: (Move, Move)) -> u32 {
    let (_, my_move) = &game;
    let move_score = match my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let win_score = match game {
        (Move::Rock, Move::Rock) => 3,
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Paper) => 3,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        (Move::Scissors, Move::Scissors) => 3,
    };
    move_score + win_score
}

fn read_lines() -> impl Iterator<Item = Vec<char>> {
    let file = File::open("./data/guide").unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|entry| entry.parse().unwrap())
                .collect()
        })
}

fn parse_guide() -> impl Iterator<Item = (Move, Move)> {
    read_lines().map(|vec_of_entries: Vec<char>| {
        (abc_parser(vec_of_entries[0]), xyz_parser(vec_of_entries[1]))
    })
}

fn parse_guide2() -> impl Iterator<Item = (Move, Move)> {
    read_lines()
        .map(|vec_of_entries: Vec<char>| {
            (
                abc_parser(vec_of_entries[0]),
                xyz_parser2(vec_of_entries[1]),
            )
        })
        .map(|game| determine_my_move(game))
}

fn main() {
    let total1: u32 = parse_guide().map(|g| score_game(g)).sum();
    let total2: u32 = parse_guide2().map(|g| score_game(g)).sum();
    println!("{}", total1);
    println!("{}", total2);
}
