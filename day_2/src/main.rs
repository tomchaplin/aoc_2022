use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn move_parser(letter: char) -> Move {
    match letter {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!(),
    }
}

fn outcome_parser(letter: char) -> Outcome {
    match letter {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn determine_my_move(game: (Move, Outcome)) -> (Move, Move) {
    let my_move = match &game {
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Lose) => Move::Scissors,
        (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Scissors, Outcome::Win) => Move::Rock,
        (Move::Scissors, Outcome::Draw) => Move::Scissors,
        (Move::Scissors, Outcome::Lose) => Move::Paper,
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
    read_lines().map(|vec_of_entries| {
        (
            move_parser(vec_of_entries[0]),
            move_parser(vec_of_entries[1]),
        )
    })
}

fn parse_guide2() -> impl Iterator<Item = (Move, Move)> {
    read_lines()
        .map(|vec_of_entries| {
            (
                move_parser(vec_of_entries[0]),
                outcome_parser(vec_of_entries[1]),
            )
        })
        .map(|game| determine_my_move(game))
}

fn score_guide(guide: impl Iterator<Item = (Move, Move)>) -> u32 {
    guide.map(|g| score_game(g)).sum()
}

fn main() {
    let total1 = score_guide(parse_guide());
    let total2 = score_guide(parse_guide2());
    println!("{}", total1);
    println!("{}", total2);
}
