use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct BatchJob {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_line(line: Vec<String>) -> BatchJob {
    let count: usize = line[1].parse().expect("Bad number of moves");
    let from: usize = line[3].parse().expect("Bad from bin");
    let to: usize = line[5].parse().expect("Bad to bin");
    BatchJob {
        count,
        from: from - 1,
        to: to - 1,
    }
}

fn read_lines() -> impl Iterator<Item = BatchJob> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(move |line| line.split(" ").map(|str| String::from(str)).collect())
        .filter(|line: &Vec<String>| line[0] == "move")
        .map(parse_line)
}

fn split_jobs(batch_jobs: impl Iterator<Item = BatchJob>) -> impl Iterator<Item = BatchJob> {
    batch_jobs
        .map(|batch_job| {
            vec![
                BatchJob {
                    from: batch_job.from,
                    to: batch_job.to,
                    count: 1
                };
                batch_job.count
            ]
            .into_iter()
        })
        .flatten()
}

fn do_batch_job(mut state: Vec<Vec<char>>, job: BatchJob) -> Vec<Vec<char>> {
    let number_in_from_pile = state[job.from].len();
    let split_point = number_in_from_pile - job.count;
    let mut split_elements = state[job.from].split_off(split_point);
    state[job.to].append(&mut split_elements);
    state
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

    let batch_jobs = read_lines();
    let jobs = split_jobs(batch_jobs);
    let final_state_a = jobs.fold(init_state.clone(), do_batch_job);
    let final_letters_a = get_final_letters(final_state_a);
    println!("{}", final_letters_a);

    let batch_jobs = read_lines();
    let final_state_b = batch_jobs.fold(init_state, do_batch_job);
    let final_letters_b = get_final_letters(final_state_b);
    println!("{}", final_letters_b);
}
