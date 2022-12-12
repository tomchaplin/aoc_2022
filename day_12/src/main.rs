use petgraph::algo::dijkstra;
use petgraph::prelude::*;

use std::fs::File;
use std::io::{self, BufRead};

struct HeightNode {
    is_start: bool,
    is_end: bool,
    height: usize,
}

struct BuildOutput {
    graph: DiGraph<(usize, usize), ()>,
    start_node: Option<NodeIndex<u32>>,
    end_node: Option<NodeIndex<u32>>,
}

fn parse_line(string: String) -> Vec<HeightNode> {
    string
        .chars()
        .map(|letter| match letter {
            'S' => HeightNode {
                is_start: true,
                is_end: false,
                height: 0,
            },
            'E' => HeightNode {
                is_start: false,
                is_end: true,
                height: 25,
            },
            letter => HeightNode {
                is_start: false,
                is_end: false,
                height: "abcdefghijklmnopqrstuvwxyz"
                    .chars()
                    .position(|elem| elem == letter)
                    .expect("Weird letter"),
            },
        })
        .collect()
}

fn read_heightmap() -> Vec<Vec<HeightNode>> {
    let file = File::open("./data/input").expect("Input file not found");
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.expect("Couldn't read line of input"))
        .map(parse_line)
        .collect()
}

fn get_neighbours(pos: (usize, usize), dims: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if pos.0 > 0 {
        neighbours.push((pos.0 - 1, pos.1));
    }
    if pos.0 + 1 < dims.0 {
        neighbours.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        neighbours.push((pos.0, pos.1 - 1));
    }
    if pos.1 + 1 < dims.1 {
        neighbours.push((pos.0, pos.1 + 1));
    }
    neighbours
}

fn build_graph(heightmap: &Vec<Vec<HeightNode>>) -> BuildOutput {
    let rows = heightmap.len();
    let cols = heightmap[0].len();
    //let mut edges: Vec<((usize, usize), (usize, usize))> = vec![];
    let mut graph = DiGraph::<(usize, usize), ()>::new();
    let mut nodes = vec![];
    let mut start_node = None;
    let mut end_node = None;
    for i in 0..rows {
        nodes.push(vec![]);
        for j in 0..cols {
            let new_node = graph.add_node((i, j));
            nodes[i].push(new_node);
            if heightmap[i][j].is_start {
                start_node = Some(new_node);
            }
            if heightmap[i][j].is_end {
                end_node = Some(new_node);
            }
        }
    }
    for i in 0..rows {
        for j in 0..cols {
            let my_height = heightmap[i][j].height;
            for neighbour in get_neighbours((i, j), (rows, cols)) {
                let neighbour_height = heightmap[neighbour.0][neighbour.1].height;
                if neighbour_height <= my_height + 1 {
                    graph.add_edge(nodes[i][j], nodes[neighbour.0][neighbour.1], ());
                }
            }
        }
    }
    BuildOutput {
        graph,
        start_node,
        end_node,
    }
}

fn main() {
    // Build graph and get references to important nodes
    let heightmap = read_heightmap();
    let build_output: BuildOutput = build_graph(&heightmap);
    let mut g = build_output.graph;
    let start_node = build_output.start_node.expect("Didn't find start node");
    let end_node = build_output.end_node.expect("Didn't find end node");
    // Part A
    let res_a = dijkstra(&g, start_node, Some(end_node), |_| 1);
    let cost = res_a.get(&end_node).expect("Couldn't reach end_node");
    println!("{:#?}", cost);
    // Part B
    // Reverse the graph
    g.reverse();
    // Find the cost to all nodes from E
    let res_b = dijkstra(&g, end_node, None, |_| 1);
    // Find the lowest cost to a node with height 0
    let min_node = res_b
        .keys()
        .filter(|&key| {
            let pos = g.node_weight(*key).unwrap();
            let height = heightmap[pos.0][pos.1].height;
            height == 0
        })
        .min_by_key(|key| res_b.get(key).unwrap())
        .expect("No minimum");
    let min_node_cost = res_b.get(min_node).unwrap();
    println!("{:#?}", min_node_cost);
}
