use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Job {
    from: usize,
    to: usize,
}

#[derive(Debug, Clone, Copy)]
struct BatchJob {
    count: usize,
    from: usize,
    to: usize,
}

// TODO: Convert from batch to iterator of jobs

fn parse_line(line: Vec<String>) -> Vec<Job> {
    let count: usize = line[1].parse().expect("Bad number of moves");
    let from: usize = line[3].parse().expect("Bad from bin");
    let to: usize = line[5].parse().expect("Bad to bin");
    vec![
        Job {
            from: from - 1,
            to: to - 1
        };
        count
    ]
}

fn parse_line_as_batch(line: Vec<String>) -> BatchJob {
    let count: usize = line[1].parse().expect("Bad number of moves");
    let from: usize = line[3].parse().expect("Bad from bin");
    let to: usize = line[5].parse().expect("Bad to bin");
    BatchJob {
        count,
        from: from - 1,
        to: to - 1,
    }
}

fn read_lines() -> impl Iterator<Item = Job> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(move |line| line.split(" ").map(|str| String::from(str)).collect())
        .filter(|line: &Vec<String>| line[0] == "move")
        .map(|line| {
            let job_vec = parse_line(line);
            return job_vec.into_iter();
        })
        .flatten()
}

fn read_lines_as_batch() -> impl Iterator<Item = BatchJob> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(move |line| line.split(" ").map(|str| String::from(str)).collect())
        .filter(|line: &Vec<String>| line[0] == "move")
        .map(parse_line_as_batch)
}

fn do_batch_job(mut state: Vec<Vec<char>>, job: BatchJob) -> Vec<Vec<char>> {
    let number_in_from_pile = state[job.from].len();
    let split_point = number_in_from_pile - job.count;
    let mut split_elements = state[job.from].split_off(split_point);
    state[job.to].append(&mut split_elements);
    state
}

fn do_job(mut state: Vec<Vec<char>>, job: Job) -> Vec<Vec<char>> {
    let popped_element = state[job.from].pop().expect("Popped to many");
    state[job.to].push(popped_element);
    return state;
}

fn get_final_letters(state: Vec<Vec<char>>) -> String {
    state
        .into_iter()
        .map(|mut vec_of_chars| vec_of_chars.pop().expect("No letters in bucket"))
        .collect()
}

fn main() {
    let init_state: Vec<Vec<char>> = vec![
        vec!['S', 'M', 'R', 'N', 'W', 'J', 'V', 'T'],
        vec!['B', 'W', 'D', 'J', 'Q', 'P', 'C', 'V'],
        vec!['B', 'J', 'F', 'H', 'D', 'R', 'P'],
        vec!['F', 'R', 'P', 'B', 'M', 'N', 'D'],
        vec!['H', 'V', 'R', 'P', 'T', 'B'],
        vec!['C', 'B', 'P', 'T'],
        vec!['B', 'J', 'R', 'P', 'L'],
        vec!['N', 'C', 'S', 'L', 'T', 'Z', 'B', 'W'],
        vec!['L', 'S', 'G'],
    ];

    let jobs = read_lines();
    let final_state_a = jobs.fold(init_state.clone(), do_job);
    let final_letters_a = get_final_letters(final_state_a);
    println!("{}", final_letters_a);

    let batch_jobs = read_lines_as_batch();
    let final_state_b = batch_jobs.fold(init_state, do_batch_job);
    let final_letters_b = get_final_letters(final_state_b);
    println!("{}", final_letters_b);
}
