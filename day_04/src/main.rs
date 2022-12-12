use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct ElfRange {
    from: u32,
    to: u32,
}

fn parse_range(range_str: &String) -> ElfRange {
    let range_vec: Vec<u32> = range_str
        .split("-")
        .map(|num| num.parse().expect("Bad ID"))
        .collect();
    ElfRange {
        from: range_vec[0],
        to: range_vec[1],
    }
}

fn parse_line(line: Vec<String>) -> (ElfRange, ElfRange) {
    (parse_range(&line[0]), parse_range(&line[1]))
}

fn read_lines() -> impl Iterator<Item = (ElfRange, ElfRange)> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(move |line| line.split(",").map(|str| String::from(str)).collect())
        .map(parse_line)
}

fn is_overlapping(pair: &(ElfRange, ElfRange)) -> bool {
    if pair.0.to < pair.1.from {
        false
    } else if pair.1.to < pair.0.from {
        false
    } else {
        true
    }
}

fn is_contained(pair: &(ElfRange, ElfRange)) -> bool {
    (pair.0.from <= pair.1.from && pair.0.to >= pair.1.to)
        || (pair.1.from <= pair.0.from && pair.1.to >= pair.0.to)
}

fn main() {
    let pairs: Vec<(ElfRange, ElfRange)> = read_lines().collect();
    let number_contained = pairs.iter().filter(|pair| is_contained(&pair)).count();
    let number_overlapping = pairs.iter().filter(|pair| is_overlapping(&pair)).count();

    println!("{}", number_contained);
    println!("{}", number_overlapping);
}
