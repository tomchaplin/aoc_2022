use num::integer::lcm;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};
extern crate queues;
use queues::*;

struct MonkeyTest {
    divisor: u64,
    target_if_true: usize,
    target_if_false: usize,
}

enum MonkeyOp {
    Multiply(u64),
    Add(u64),
    Square,
}

struct Monkey {
    items: Queue<u64>,
    op: MonkeyOp,
    test: MonkeyTest,
    inspected: u64,
}

struct MonkeyMob {
    mob: Vec<Monkey>,
    current: usize,
    round: u64,
}

fn parse_monkey(manifest: &[String]) -> Monkey {
    // Determine items
    let item_list = manifest[1].trim().replace(",", "");
    let split_item_list: Vec<&str> = item_list.split(" ").collect();
    // Throw away first two words
    let mut items: Queue<u64> = queue![];
    for (i, item) in split_item_list.iter().enumerate() {
        if i <= 1 {
            continue;
        }
        items.add(item.parse().unwrap()).unwrap();
    }
    // Determine the operation
    let op_line: Vec<&str> = manifest[2].trim().split(" ").collect();
    let op = match op_line[5] {
        "old" => MonkeyOp::Square,
        num_as_str => {
            let num: u64 = num_as_str.parse().unwrap();
            match op_line[4] {
                "*" => MonkeyOp::Multiply(num),
                "+" => MonkeyOp::Add(num),
                _ => panic!(),
            }
        }
    };
    // Determine the test
    let divisor: u64 = manifest[3].split(" ").last().unwrap().parse().unwrap();
    let target_if_true: usize = manifest[4].split(" ").last().unwrap().parse().unwrap();
    let target_if_false: usize = manifest[5].split(" ").last().unwrap().parse().unwrap();
    let test = MonkeyTest {
        divisor,
        target_if_true,
        target_if_false,
    };
    Monkey {
        items,
        op,
        test,
        inspected: 0,
    }
}

fn parse_mob() -> MonkeyMob {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    let lines_vec: Vec<String> = lines
        .map(|line| line.expect("Couldn't read line of input"))
        .collect();
    let mob = lines_vec
        .split(|line| line.is_empty())
        .map(parse_monkey)
        .collect();
    MonkeyMob {
        mob,
        current: 0,
        round: 1,
    }
}

fn apply_operation(item: u64, op: &MonkeyOp) -> u64 {
    match op {
        MonkeyOp::Multiply(n) => item * n,
        MonkeyOp::Add(n) => item + n,
        MonkeyOp::Square => item * item,
    }
}

fn get_bored(item: u64) -> u64 {
    (item as f32 / 3.0).floor() as u64
}

fn determine_target(item: u64, test: &MonkeyTest) -> usize {
    if item % test.divisor == 0 {
        test.target_if_true
    } else {
        test.target_if_false
    }
}

fn play(mob: &mut MonkeyMob, should_bore: bool, mob_lcm: u64) {
    let working_monkey = &mut mob.mob[mob.current];
    if let Ok(item) = working_monkey.items.remove() {
        let mut new_worry = apply_operation(item, &working_monkey.op);
        working_monkey.inspected += 1;
        if should_bore {
            new_worry = get_bored(new_worry);
        }
        // Modulo by all divisiblity test to prevent blow up
        new_worry = new_worry % mob_lcm;
        let target = determine_target(new_worry, &working_monkey.test);
        let target_monkey = &mut mob.mob[target];
        target_monkey
            .items
            .add(new_worry)
            .expect("Couldn't add item to monkey's list");
    } else {
        // The working_monkey has no more items, move onto next
        mob.current += 1;
    }
    // Figure out whether we've finished the round
    if mob.current >= mob.mob.len() {
        mob.current = 0;
        mob.round += 1;
    }
}

fn get_lcm(mob: &MonkeyMob) -> u64 {
    mob.mob
        .iter()
        .map(|monkey| monkey.test.divisor)
        .fold(1, |working_lcm, next_divisor| {
            lcm(working_lcm, next_divisor)
        })
}

fn compute_monkey_business(mob: &MonkeyMob) -> u64 {
    let mut inspecteds: Vec<u64> = mob.mob.iter().map(|monkey| monkey.inspected).collect();
    inspecteds.sort_by_key(|w| Reverse(*w)); // Decreasing sort
    inspecteds[0] * inspecteds[1]
}

fn part_a() {
    let mut mob = parse_mob();
    let mob_lcm = get_lcm(&mob);
    while mob.round <= 20 {
        play(&mut mob, true, mob_lcm);
    }
    println!("{}", compute_monkey_business(&mob));
}

fn part_b() {
    let mut mob = parse_mob();
    let mob_lcm = get_lcm(&mob);
    while mob.round <= 10000 {
        play(&mut mob, false, mob_lcm);
    }
    println!("{}", compute_monkey_business(&mob));
}

fn main() {
    part_a();
    part_b();
}
