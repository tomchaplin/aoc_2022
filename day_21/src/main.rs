#![feature(int_roundings)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq)]
enum MonkeyJob {
    Multiply(String, String),
    Divide(String, String),
    Add(String, String),
    Subtract(String, String),
    Number(i64),
}

impl MonkeyJob {
    fn get_pairs(&self) -> (&String, &String) {
        match &self {
            MonkeyJob::Multiply(k1, k2) => (k1, k2),
            MonkeyJob::Divide(k1, k2) => (k1, k2),
            MonkeyJob::Add(k1, k2) => (k1, k2),
            MonkeyJob::Subtract(k1, k2) => (k1, k2),
            MonkeyJob::Number(_) => panic!(),
        }
    }
}

fn read_line(line: String) -> (String, MonkeyJob) {
    let split: Vec<String> = line.split(" ").map(|s| s.to_owned()).collect();
    let name = split[0].replace(":", "");
    let job = if split[1].chars().all(|c| c.is_numeric()) {
        MonkeyJob::Number(split[1].parse().unwrap())
    } else {
        match split[2].as_str() {
            "*" => MonkeyJob::Multiply(split[1].clone(), split[3].clone()),
            "/" => MonkeyJob::Divide(split[1].clone(), split[3].clone()),
            "+" => MonkeyJob::Add(split[1].clone(), split[3].clone()),
            "-" => MonkeyJob::Subtract(split[1].clone(), split[3].clone()),
            _ => panic!(),
        }
    };
    (name, job)
}

fn read_input() -> HashMap<String, MonkeyJob> {
    let file = File::open("./data/input").expect("Input file not found");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(read_line)
        .collect()
}

fn get_output(
    key: &String,
    jobs: &HashMap<String, MonkeyJob>,
    output_cache: &mut HashMap<String, i64>,
) -> i64 {
    let cached = output_cache.get(key);
    if let Some(output) = cached {
        return *output;
    }
    let job = jobs.get(key).unwrap();
    let output = match job {
        MonkeyJob::Multiply(k1, k2) => {
            get_output(k1, jobs, output_cache) * get_output(k2, jobs, output_cache)
        }
        MonkeyJob::Divide(k1, k2) => {
            get_output(k1, jobs, output_cache) / get_output(k2, jobs, output_cache)
        }
        MonkeyJob::Add(k1, k2) => {
            get_output(k1, jobs, output_cache) + get_output(k2, jobs, output_cache)
        }
        MonkeyJob::Subtract(k1, k2) => {
            get_output(k1, jobs, output_cache) - get_output(k2, jobs, output_cache)
        }
        MonkeyJob::Number(num) => *num,
    };
    output_cache.insert(key.clone(), output);
    return output;
}

fn get_output_with_human_input(
    key: &String,
    jobs: &HashMap<String, MonkeyJob>,
    human_input: i64,
    output_cache: &mut HashMap<String, i64>,
) -> i64 {
    if key == "humn" {
        return human_input;
    }
    let cached = output_cache.get(key);
    if let Some(output) = cached {
        return *output;
    }
    let job = jobs.get(key).unwrap();
    let output = match job {
        MonkeyJob::Multiply(k1, k2) => {
            get_output_with_human_input(k1, jobs, human_input, output_cache)
                * get_output_with_human_input(k2, jobs, human_input, output_cache)
        }
        MonkeyJob::Divide(k1, k2) => {
            get_output_with_human_input(k1, jobs, human_input, output_cache)
                / get_output_with_human_input(k2, jobs, human_input, output_cache)
        }
        MonkeyJob::Add(k1, k2) => {
            get_output_with_human_input(k1, jobs, human_input, output_cache)
                + get_output_with_human_input(k2, jobs, human_input, output_cache)
        }
        MonkeyJob::Subtract(k1, k2) => {
            get_output_with_human_input(k1, jobs, human_input, output_cache)
                - get_output_with_human_input(k2, jobs, human_input, output_cache)
        }
        MonkeyJob::Number(num) => *num,
    };
    output_cache.insert(key.clone(), output);
    return output;
}

fn cached_output_from_human_input(
    key: &String,
    jobs: &HashMap<String, MonkeyJob>,
    human_input: i64,
) -> i64 {
    let mut output_cache = HashMap::new();
    get_output_with_human_input(key, jobs, human_input, &mut output_cache)
}

fn compute_rhs_minus_lhs(
    pair: (&String, &String),
    jobs: &HashMap<String, MonkeyJob>,
    human_input: i64,
    cache: &mut HashMap<i64, i64>,
) -> i64 {
    if let Some(val) = cache.get(&human_input) {
        return *val;
    }
    let rhs = cached_output_from_human_input(pair.1, jobs, human_input);
    let lhs = cached_output_from_human_input(pair.0, jobs, human_input);
    let val = rhs - lhs;
    cache.insert(human_input, val);
    val
}

fn root_find(
    pair: (&String, &String),
    jobs: &HashMap<String, MonkeyJob>,
    initial: (i64, i64),
) -> i64 {
    let mut interval = initial;
    let mut cache = HashMap::new();
    loop {
        println!("{:?}", interval);
        let midpoint = (interval.1 + interval.0).div_floor(2);
        let val_at_mid = compute_rhs_minus_lhs(pair, jobs, midpoint, &mut cache);
        // We've found a valid zero now we just scan for smallest
        if val_at_mid == 0 {
            return scan_for_smallest_zero(pair, jobs, (interval.0, midpoint), &mut cache);
        }
        let val_at_left = compute_rhs_minus_lhs(pair, jobs, interval.0, &mut cache);
        let val_at_right = compute_rhs_minus_lhs(pair, jobs, interval.1, &mut cache);
        if val_at_left.signum() == val_at_right.signum() {
            panic!("Same sign")
        }
        if val_at_mid.signum() == val_at_right.signum() {
            interval.1 = midpoint;
            continue;
        }
        if val_at_mid.signum() == val_at_left.signum() {
            interval.0 = midpoint;
            continue;
        }
        panic!()
    }
}

fn scan_for_smallest_zero(
    pair: (&String, &String),
    jobs: &HashMap<String, MonkeyJob>,
    interval: (i64, i64),
    cache: &mut HashMap<i64, i64>,
) -> i64 {
    for i in (interval.0)..=(interval.1) {
        if compute_rhs_minus_lhs(pair, jobs, i, cache) == 0 {
            return i;
        }
    }
    return interval.1;
}

fn part_a() {
    let mut outputs: HashMap<String, i64> = HashMap::new();
    let input = read_input();
    let root_output = get_output(&String::from("root"), &input, &mut outputs);
    println!("{}", root_output);
}

// Root finding approach
fn part_b() {
    let input = read_input();
    let root_pairs = input.get(&String::from("root")).unwrap().get_pairs();
    println!("{:?}", root_pairs);
    let correct_input = root_find(root_pairs, &input, (0, 1000000000000000));
    println!("{}", correct_input);
}

fn main() {
    part_a();
    part_b();
}
