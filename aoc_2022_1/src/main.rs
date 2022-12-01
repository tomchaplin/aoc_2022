use std::fs::File;
use std::io::{self, BufRead};

struct Accumulator {
    top: Vec<u32>,
    working: u32,
}

fn fold_in_working(accum: Accumulator) -> Accumulator {
    let Accumulator { mut top, working } = accum;
    // The first element is guaranteed to be the smallest
    if top[0] < working {
        top[0] = working;
    }
    top.sort();
    Accumulator { top, working: 0 }
}

fn find_top_n(n_to_find: usize) -> Vec<u32> {
    let file = File::open("./data/input1").unwrap();
    let lines = io::BufReader::new(file).lines();
    let result = lines.fold(
        Accumulator {
            top: vec![0; n_to_find],
            working: 0,
        },
        |accum, line| {
            let contents = line.unwrap();
            if contents.is_empty() {
                fold_in_working(accum)
            } else {
                let calories: u32 = contents.parse().unwrap();
                Accumulator {
                    top: accum.top,
                    working: accum.working + calories,
                }
            }
        },
    );
    result.top
}

fn add_top_n(n_to_find: usize) -> u32 {
    find_top_n(n_to_find).iter().sum()
}

fn main() {
    println!("{}", add_top_n(1));
    println!("{}", add_top_n(3));
}
