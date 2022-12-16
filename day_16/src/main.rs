use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;

use std::fs::File;
use std::io::{self, BufRead};

use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct TunnelNode {
    idx: usize,
    name: String,
    flow: u32,
    neighbours: Vec<usize>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct GameState {
    start: usize,
    remaining: Vec<usize>,
    time_remaining: u32,
}

// TODO: Rephrase with sscanf
fn read_network() -> Vec<TunnelNode> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let mut nodes: Vec<TunnelNode> = lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let split_line: Vec<&str> = line.split(" ").collect();
            let name = split_line[1].to_owned();
            let flow = split_line[4]
                .replace("rate=", "")
                .replace(";", "")
                .parse()
                .unwrap();
            TunnelNode {
                idx,
                name,
                flow,
                neighbours: vec![],
            }
        })
        .collect();
    let neighbours: Vec<Vec<usize>> = nodes
        .iter()
        .map(|node| {
            let line = &lines[node.idx];
            let split_line: Vec<&str> = line.split(" ").collect();
            let neighbour_names = split_line[9..].iter().map(|name| name.replace(",", ""));
            let mut neighbour_idxs = vec![];
            for name in neighbour_names {
                let neighbour_node = nodes.iter().find(|node| node.name == name).unwrap();
                neighbour_idxs.push(neighbour_node.idx);
            }
            neighbour_idxs
        })
        .collect();
    for (idx, node) in nodes.iter_mut().enumerate() {
        node.neighbours = neighbours[idx].clone();
    }
    nodes
}

// Returns vector of flows for nodes where flow > 0
// Second entry is distance grid for these nodes
// Also include start node at the beginning
fn compute_critical(graph: DiGraph<(String, u32), ()>) -> (Vec<u32>, Vec<Vec<u32>>) {
    let node_refs = graph.node_references();
    let critical_nodes: Vec<_> = node_refs.filter(|node| (node.1).1 > 0).collect();
    let mut flows: Vec<_> = critical_nodes.iter().map(|node| (node.1).1).collect();
    let n_nodes = critical_nodes.len();
    let mut distances = vec![vec![0; n_nodes + 1]; n_nodes + 1];
    for (idx_start, start) in critical_nodes.iter().enumerate() {
        for (idx_end, end) in critical_nodes.iter().enumerate() {
            let res = dijkstra(&graph, start.0, Some(end.0), |_| 1);
            let dist = res.get(&end.0).unwrap();
            distances[idx_start][idx_end] = *dist as u32;
        }
    }
    let start_node = graph
        .node_references()
        .find(|node| (node.1).0 == "AA")
        .unwrap();
    flows.push(0);
    for (idx, node) in critical_nodes.iter().enumerate() {
        let res = dijkstra(&graph, start_node.0, Some(node.0), |_| 1);
        let dist = res.get(&node.0).unwrap();
        distances[n_nodes][idx] = *dist as u32;
        let res = dijkstra(&graph, node.0, Some(start_node.0), |_| 1);
        let dist = res.get(&start_node.0).unwrap();
        distances[idx][n_nodes] = *dist as u32;
    }
    (flows, distances)
}

fn build_graph(nodes: &Vec<TunnelNode>) -> DiGraph<(String, u32), ()> {
    let mut graph = DiGraph::<(String, u32), ()>::new();
    let mut graph_nodes = vec![];
    for node in nodes.iter() {
        graph_nodes.push(graph.add_node((node.name.clone(), node.flow)));
    }
    for node in nodes.iter() {
        for neighbour in node.neighbours.iter() {
            graph.add_edge(graph_nodes[node.idx], graph_nodes[*neighbour], ());
        }
    }
    graph
}

fn compute_additional_flow_immut(
    flows: &Vec<u32>,
    distances: &Vec<Vec<u32>>,
    state: &GameState,
    scratchpad: &HashMap<GameState, u32>,
) -> u32 {
    let remaining = &state.remaining;
    let time_remaining = state.time_remaining;
    // No more valves to turn
    if remaining.is_empty() || time_remaining == 0 {
        return 0;
    }
    // Check memos
    if let Some(add_flow) = scratchpad.get(&state) {
        return *add_flow;
    }
    let max_add_flow = remaining
        .iter()
        .map(|next_valve| {
            let mut remaining_after_valve = remaining.clone();
            remaining_after_valve.retain(|valve| valve != next_valve);
            let time_needed = distances[state.start][*next_valve] + 1;
            if time_needed > time_remaining {
                return 0;
            }
            let time_with_valve_on = time_remaining - time_needed;
            let total_release = time_with_valve_on * flows[*next_valve];
            total_release
                + compute_additional_flow_immut(
                    flows,
                    distances,
                    &GameState {
                        start: *next_valve,
                        remaining: remaining_after_valve,
                        time_remaining: time_with_valve_on,
                    },
                    scratchpad,
                )
        })
        .max()
        .unwrap();
    max_add_flow
}

fn compute_additional_flow(
    flows: &Vec<u32>,
    distances: &Vec<Vec<u32>>,
    state: GameState,
    scratchpad: &mut HashMap<GameState, u32>,
) -> u32 {
    let max_add_flow = compute_additional_flow_immut(flows, distances, &state, scratchpad);
    scratchpad.insert(state, max_add_flow);
    max_add_flow
}

fn partition_by_mask<T: Copy>(input_vec: &Vec<T>, mask: u64) -> (Vec<T>, Vec<T>) {
    let vec_len = input_vec.len();
    let mut left_vec = vec![];
    let mut right_vec = vec![];
    for idx in 0..(vec_len - 1) {
        let include_in_left = mask >> idx & 1 == 1;
        if include_in_left {
            left_vec.push(input_vec[idx]);
        } else {
            right_vec.push(input_vec[idx]);
        }
    }
    // To avoid double counting, we always push last element to left
    left_vec.push(input_vec[vec_len - 1]);
    (left_vec, right_vec)
}

fn partitions<T: Copy>(input_vec: Vec<T>) -> impl Iterator<Item = (Vec<T>, Vec<T>)> {
    let vec_len = input_vec.len();
    let n_partitions = 1 << (vec_len - 1); // 2^vec_len
    (0..=n_partitions).map(move |mask| partition_by_mask(&input_vec, mask))
}

fn main() {
    // Part A
    let nodes = read_network();
    let graph = build_graph(&nodes);
    let (flows, distances) = compute_critical(graph);
    let n_nodes = flows.len();
    let mut scratchpad = HashMap::default();
    let init_state = GameState {
        start: n_nodes - 1,
        remaining: (0..flows.len()).collect(),
        time_remaining: 30,
    };
    let max_flow = compute_additional_flow(&flows, &distances, init_state, &mut scratchpad);
    println!("{}", max_flow);
    // Part B
    let max_two_flow = partitions((0..flows.len()).collect())
        .par_bridge()
        .map(|(left, right)| {
            let left_state = GameState {
                start: n_nodes - 1,
                remaining: left,
                time_remaining: 26,
            };
            let right_state = GameState {
                start: n_nodes - 1,
                remaining: right,
                time_remaining: 26,
            };
            let left_flow =
                compute_additional_flow_immut(&flows, &distances, &left_state, &scratchpad);
            let right_flow =
                compute_additional_flow_immut(&flows, &distances, &right_state, &scratchpad);
            left_flow + right_flow
        })
        .max()
        .unwrap();
    println!("{}", max_two_flow);
}
