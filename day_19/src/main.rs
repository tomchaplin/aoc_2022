use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, Clone, EnumIter)]
enum BuildTarget {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct MinerState {
    inventory: (u32, u32, u32),
    robots_made: (u32, u32, u32, u32),
    robots_in_prod: (u32, u32, u32, u32),
    time_remaining: u32,
}

impl MinerState {
    fn new(time: u32) -> Self {
        Self {
            inventory: (0, 0, 0),
            robots_made: (1, 0, 0, 0),
            robots_in_prod: (0, 0, 0, 0),
            time_remaining: time,
        }
    }

    fn has_sufficient_inventory(&self, cost: &RobotCost) -> bool {
        self.inventory.0 >= cost.0 && self.inventory.1 >= cost.1 && self.inventory.2 >= cost.2
    }

    fn possible_builds<'a>(
        &'a self,
        costs: &'a Costs,
    ) -> impl Iterator<Item = Option<BuildTarget>> + 'a {
        let mut some_targets: Vec<_> = BuildTarget::iter()
            .filter(|target| self.has_sufficient_inventory(costs.get(&target).unwrap()))
            .map(|target| Some(target))
            .collect();
        if some_targets.len() != 4 {
            some_targets.push(None)
        }
        some_targets.into_iter()
    }

    // Remove invetory and add robot to production
    fn initiate_build(&mut self, target: &BuildTarget, costs: &Costs) {
        let costs = costs.get(target).unwrap();
        self.inventory.0 -= costs.0;
        self.inventory.1 -= costs.1;
        self.inventory.2 -= costs.2;
        match target {
            BuildTarget::Ore => self.robots_in_prod.0 += 1,
            BuildTarget::Clay => self.robots_in_prod.1 += 1,
            BuildTarget::Obsidian => self.robots_in_prod.2 += 1,
            BuildTarget::Geode => self.robots_in_prod.3 += 1,
        }
    }

    // Collect resources, add robots_in_prod to robots_made and decrement time
    // Return additional geodes mined
    fn advance(&mut self) -> u32 {
        // Collect
        self.inventory.0 += self.robots_made.0;
        self.inventory.1 += self.robots_made.1;
        self.inventory.2 += self.robots_made.2;
        let geodes_mined = self.robots_made.3;
        // Build robots
        self.robots_made.0 += self.robots_in_prod.0;
        self.robots_made.1 += self.robots_in_prod.1;
        self.robots_made.2 += self.robots_in_prod.2;
        self.robots_made.3 += self.robots_in_prod.3;
        // Clear production
        self.robots_in_prod.0 = 0;
        self.robots_in_prod.1 = 0;
        self.robots_in_prod.2 = 0;
        self.robots_in_prod.3 = 0;
        // Decrement time
        self.time_remaining -= 1;
        return geodes_mined;
    }

    fn compute_max_additional_geodes(
        &self,
        costs: &Costs,
        scratchpad: &mut HashMap<MinerState, u32>,
    ) -> u32 {
        if self.time_remaining == 0 {
            return 0;
        }
        if let Some(add_geodes) = scratchpad.get(self) {
            return *add_geodes;
        }
        // TODO: Add a short-circuit along the lines of
        // If this is a strictly worse scenario than we've seen before, return 0
        // TODO: Add a short-circuit along the lines of
        // If we will never be able to build another geode miner then we can figure out our final scenario
        let max_geodes = self
            .possible_builds(costs)
            .map(|target| {
                let mut after_building = self.clone();
                if let Some(target) = target {
                    after_building.initiate_build(&target, costs);
                }
                let geodes_from_this_round = after_building.advance();
                let geodes_from_future_rounds =
                    after_building.compute_max_additional_geodes(costs, scratchpad);
                return geodes_from_this_round + geodes_from_future_rounds;
            })
            .max()
            .unwrap();
        scratchpad.insert(self.clone(), max_geodes);
        max_geodes
    }
}

type Costs = HashMap<BuildTarget, RobotCost>;

type RobotCost = (u32, u32, u32);

fn n_geodes(idx: usize, time: u32, costs: &Costs) -> u32 {
    let mut scratchpad = HashMap::new();
    let miner = MinerState::new(time);
    let max_n_geodes = miner.compute_max_additional_geodes(&costs, &mut scratchpad);
    println!("Finished idx {}", idx);
    max_n_geodes
}

fn parse_blueprint(line: String) -> Costs {
    let costs: Vec<u32> = line
        .split(" ")
        .filter_map(|word| word.parse().ok())
        .collect();
    HashMap::from([
        (BuildTarget::Ore, (costs[0], 0, 0)),
        (BuildTarget::Clay, (costs[1], 0, 0)),
        (BuildTarget::Obsidian, (costs[2], costs[3], 0)),
        (BuildTarget::Geode, (costs[4], 0, costs[5])),
    ])
}

fn read_blueprints() -> Vec<Costs> {
    let file = File::open("./data/input").expect("Input file not found");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(parse_blueprint)
        .collect()
}

fn part_a() {
    let blueprints = read_blueprints();
    println!("{:?}", blueprints);
    let n_geode_vec: Vec<_> = blueprints
        .clone()
        .into_par_iter()
        .enumerate()
        .map(|(idx, costs)| (idx + 1, n_geodes(idx + 1, 24, &costs)))
        .collect();
    println!("{:?}", n_geode_vec);
    let qualities: Vec<_> = n_geode_vec
        .iter()
        .map(|&(idx, n_g)| idx as u32 * n_g)
        .collect();
    println!("{:?}", qualities);
    let sum_qualities: u32 = qualities.iter().sum();
    println!("{}", sum_qualities);
}

fn part_b() {
    let blueprints: Vec<_> = read_blueprints().into_iter().take(3).collect();
    println!("{:?}", blueprints);
    let n_geode_vec: Vec<_> = blueprints
        .clone()
        .into_par_iter()
        .enumerate()
        .map(|(idx, costs)| n_geodes(idx + 1, 32, &costs))
        .collect();
    println!("{:?}", n_geode_vec);
    let prod_n_genodes: u32 = n_geode_vec.iter().product();
    println!("{}", prod_n_genodes);
}

fn main() {
    //part_a()
    part_b()
}
