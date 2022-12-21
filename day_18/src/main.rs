use queues::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct AlgoState {
    bounds: BoundingBox,
    knowledge: HashMap<Voxel, VoxelState>,
    queue: Queue<Voxel>,
    external_voxels: Vec<Voxel>,
}

enum VoxelState {
    Rock,
    External,
    OutOfBounds,
}

type Voxel = (isize, isize, isize);

type BoundingBox = ((isize, isize), (isize, isize), (isize, isize));

trait SimpleNeighbour {
    fn is_neighbour(&self, other: &Voxel) -> bool;
    fn n_neighbours(&self, voxel_list: &Vec<Voxel>) -> usize;
}

impl SimpleNeighbour for Voxel {
    fn is_neighbour(&self, other: &Voxel) -> bool {
        let x_diff = self.0.abs_diff(other.0);
        let y_diff = self.1.abs_diff(other.1);
        let z_diff = self.2.abs_diff(other.2);
        x_diff + y_diff + z_diff == 1
    }

    fn n_neighbours(&self, voxel_list: &Vec<Voxel>) -> usize {
        voxel_list
            .iter()
            .filter(|other| self.is_neighbour(other))
            .count()
    }
}

impl AlgoState {
    // Initialise state with bounds and rocks
    fn new(bounds: BoundingBox, rock_voxels: &Vec<Voxel>) -> Self {
        let mut knowledge = HashMap::new();
        for voxel in rock_voxels {
            knowledge.insert(voxel.clone(), VoxelState::Rock);
        }
        Self {
            bounds,
            knowledge,
            queue: Queue::new(),
            external_voxels: vec![],
        }
    }

    fn run(&mut self) {
        let border = self.border_voxels();
        for voxel in border {
            self.visit_voxel(voxel);
        }
        while let Ok(next_voxel) = self.queue.remove() {
            self.visit_voxel(next_voxel);
        }
    }

    fn border_voxels(&self) -> Vec<Voxel> {
        let mut voxels = vec![];
        for i in self.bounds.1 .0..=self.bounds.1 .1 {
            for j in self.bounds.2 .0..=self.bounds.2 .1 {
                voxels.push((self.bounds.0 .0, i, j));
                voxels.push((self.bounds.0 .1, i, j));
            }
        }
        for i in self.bounds.0 .0..=self.bounds.0 .1 {
            for j in self.bounds.2 .0..=self.bounds.2 .1 {
                voxels.push((i, self.bounds.1 .0, j));
                voxels.push((i, self.bounds.1 .1, j));
            }
        }
        for i in self.bounds.0 .0..=self.bounds.0 .1 {
            for j in self.bounds.1 .0..=self.bounds.1 .1 {
                voxels.push((i, j, self.bounds.2 .0));
                voxels.push((i, j, self.bounds.2 .1));
            }
        }
        voxels
    }

    fn visit_voxel(&mut self, voxel: Voxel) {
        let knowledge = self.test_knowledge(&voxel);
        if knowledge.is_none() {
            self.knowledge.insert(voxel.clone(), VoxelState::External);
            self.external_voxels.push(voxel.clone());
            self.enqueue((voxel.0 + 1, voxel.1, voxel.2));
            self.enqueue((voxel.0 - 1, voxel.1, voxel.2));
            self.enqueue((voxel.0, voxel.1 + 1, voxel.2));
            self.enqueue((voxel.0, voxel.1 - 1, voxel.2));
            self.enqueue((voxel.0, voxel.1, voxel.2 + 1));
            self.enqueue((voxel.0, voxel.1, voxel.2 - 1));
        }
    }

    fn enqueue(&mut self, voxel: Voxel) {
        self.queue.add(voxel).expect("Couldn't push to voxel queue");
    }

    fn test_knowledge(&self, voxel: &Voxel) -> Option<&VoxelState> {
        if (voxel.0 < self.bounds.0 .0 || voxel.0 > self.bounds.0 .1)
            || (voxel.1 < self.bounds.1 .0 || voxel.1 > self.bounds.1 .1)
            || (voxel.2 < self.bounds.2 .0 || voxel.2 > self.bounds.2 .1)
        {
            Some(&VoxelState::OutOfBounds)
        } else {
            self.knowledge.get(voxel)
        }
    }
}

fn get_padded_min_max(coord_list: Vec<isize>) -> (isize, isize) {
    (
        *coord_list.iter().min().unwrap() - 1,
        *coord_list.iter().max().unwrap() + 1,
    )
}

fn compute_bounding_box(voxel_list: &Vec<Voxel>) -> BoundingBox {
    (
        get_padded_min_max(voxel_list.iter().map(|v| v.0).collect()),
        get_padded_min_max(voxel_list.iter().map(|v| v.1).collect()),
        get_padded_min_max(voxel_list.iter().map(|v| v.2).collect()),
    )
}

fn parse_voxel(line: String) -> Voxel {
    let as_vec: Vec<_> = line
        .split(",")
        .map(|entry| entry.parse().unwrap())
        .collect();
    (as_vec[0], as_vec[1], as_vec[2])
}

fn read_voxels() -> Vec<Voxel> {
    let file = File::open("./data/input").expect("Input file not found");
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(parse_voxel)
        .collect()
}

fn main() {
    let all_voxels = read_voxels();
    // Part A
    let surface_area: usize = all_voxels
        .iter()
        .map(|voxel| 6 - voxel.n_neighbours(&all_voxels))
        .sum();
    println!("{}", surface_area);
    // Part B
    let bounding_box = compute_bounding_box(&all_voxels);
    let mut algo = AlgoState::new(bounding_box, &all_voxels);
    algo.run();
    let external_voxels = algo.external_voxels;

    let external_surface_area: usize = external_voxels
        .iter()
        .map(|voxel| voxel.n_neighbours(&all_voxels))
        .sum();
    println!("{:?}", external_surface_area);
}
