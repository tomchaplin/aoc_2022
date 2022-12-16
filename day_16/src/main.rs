use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeReferences;

use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashMap;

struct GameState {
    elephant_pos: usize,
    my_pos: usize,
    closed_valves: Vec<usize>,
    time_remaining: u32,
}

#[derive(Debug)]
struct TunnelNode {
    idx: usize,
    name: String,
    flow: u32,
    neighbours: Vec<usize>,
}

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

fn compute_additional_flow(
    flows: &Vec<u32>,
    distances: &Vec<Vec<u32>>,
    start: usize,
    remaining: Vec<usize>,
    time_remaining: u32,
    scratchpad: &mut HashMap<(usize, Vec<usize>, u32), u32>,
) -> u32 {
    // No more valves to turn
    if remaining.is_empty() || time_remaining == 0 {
        return 0;
    }
    // Check memos
    if let Some(add_flow) = scratchpad.get(&(start, remaining.clone(), time_remaining)) {
        return *add_flow;
    }
    let max_add_flow = remaining
        .iter()
        .map(|next_valve| {
            let mut remaining_after_valve = remaining.clone();
            remaining_after_valve.retain(|valve| valve != next_valve);
            let time_needed = distances[start][*next_valve] + 1;
            if time_needed > time_remaining {
                return 0;
            }
            let time_with_valve_on = time_remaining - time_needed;
            let total_release = time_with_valve_on * flows[*next_valve];
            total_release
                + compute_additional_flow(
                    flows,
                    distances,
                    *next_valve,
                    remaining_after_valve,
                    time_with_valve_on,
                    scratchpad,
                )
        })
        .max()
        .unwrap();
    scratchpad.insert((start, remaining.clone(), time_remaining), max_add_flow);
    max_add_flow
}

fn main() {
    let nodes = read_network();
    let graph = build_graph(&nodes);
    let (flows, distances) = compute_critical(graph);
    let start = flows.len() - 1;
    let remaining = (0..flows.len()).collect();
    let time_remaining = 30;
    let mut scratchpad = HashMap::default();
    let max_flow = compute_additional_flow(
        &flows,
        &distances,
        start,
        remaining,
        time_remaining,
        &mut scratchpad,
    );
    println!("{}", max_flow);
}
