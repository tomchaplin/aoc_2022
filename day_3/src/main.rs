use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn split_in_half(mut priorities: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let split_point: usize = priorities.len() / 2;
    let second_half = priorities.split_off(split_point);
    (priorities, second_half)
}

fn parse_line(string: String) -> Vec<usize> {
    string
        .chars()
        .map(|letter| {
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                .chars()
                .position(|elem| elem == letter)
                .expect("Weird letter")
                + 1
        })
        .collect()
}

fn read_lines() -> impl Iterator<Item = Vec<usize>> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(parse_line)
}

fn find_common_priority(pair: (Vec<usize>, Vec<usize>)) -> usize {
    let set0: HashSet<usize> = HashSet::from_iter(pair.0.into_iter());
    let set1: HashSet<usize> = HashSet::from_iter(pair.1.into_iter());
    let mut intersection = set0.intersection(&set1).map(|&elem| elem);
    intersection.next().expect("Ruckscaks have no intersection")
}

fn find_badge(chunk: Vec<Vec<usize>>) -> usize {
    let set0: HashSet<usize> = HashSet::from_iter(chunk[0].clone().into_iter());
    let set1: HashSet<usize> = HashSet::from_iter(chunk[1].clone().into_iter());
    let set2: HashSet<usize> = HashSet::from_iter(chunk[2].clone().into_iter());
    let intersection1: HashSet<usize> =
        HashSet::from_iter(set0.intersection(&set1).map(|&elem| elem));
    let mut intersection2 = intersection1.intersection(&set2).map(|&elem| elem);
    intersection2.next().expect("No badge")
}

fn main() {
    let lines = read_lines();
    let split_lines = lines.map(|line| split_in_half(line));
    let common = split_lines.map(|pair| find_common_priority(pair));
    println!("{}", common.sum::<usize>());

    let triples = read_lines().chunks(3);
    let triples_as_vecs = triples.into_iter().map(|chunk| chunk.collect());
    let badges = triples_as_vecs.map(find_badge);
    println!("{}", badges.sum::<usize>());
}
