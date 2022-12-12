use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Node<T> {
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug, Default)]
struct ArenaTree<T> {
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T> {
    fn add_node(&mut self, val: T, parent: Option<usize>) -> usize {
        // Add new node to tree
        let new_idx = self.arena.len();
        let new_node = Node {
            idx: new_idx,
            val,
            parent,
            children: vec![],
        };
        self.arena.push(new_node);
        // Link up to parent
        if let Some(parent_idx) = parent {
            self.arena[parent_idx].children.push(new_idx);
        }
        return new_idx;
    }

    fn get_node(&self, idx: usize) -> &Node<T> {
        &self.arena[idx]
    }

    fn get_parent(&self, idx: usize) -> Option<usize> {
        self.arena[idx].parent.clone()
    }
}

#[derive(Debug)]
enum ElfOperation {
    MoveUp,
    ListFiles,
    ChangeDirectory(String),
}

#[derive(Debug)]
enum ParsedLine {
    Operation(ElfOperation),
    Listing(ElfFile),
}

#[derive(Debug, PartialEq)]
enum ElfType {
    File,
    Dir,
}

#[derive(Debug, PartialEq)]
pub struct ElfFile {
    name: String,
    t: ElfType,
    size: Option<u32>,
}

fn parse_operation(line: Vec<String>) -> ParsedLine {
    ParsedLine::Operation(match line[1].as_str() {
        "ls" => ElfOperation::ListFiles,
        "cd" => match line[2].as_str() {
            ".." => ElfOperation::MoveUp,
            _ => ElfOperation::ChangeDirectory(line[2].clone()),
        },
        _ => panic!(),
    })
}

fn parse_listing(line: Vec<String>) -> ParsedLine {
    ParsedLine::Listing(match line[0].as_str() {
        "dir" => ElfFile {
            name: line[1].clone(),
            t: ElfType::Dir,
            size: None,
        },
        other => {
            let size: u32 = other.parse().unwrap();
            ElfFile {
                name: line[1].clone(),
                t: ElfType::File,
                size: Some(size),
            }
        }
    })
}

fn parse_line(line: Vec<String>) -> ParsedLine {
    match line[0].as_str() {
        "$" => parse_operation(line),
        _ => parse_listing(line),
    }
}

fn read_lines<'a>() -> Vec<ParsedLine> {
    let file = File::open("./data/input").unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
        .map(|line| line.unwrap().split(" ").map(|s| s.to_string()).collect())
        .map(parse_line)
        .collect()
}

fn handle_listing(tree: &mut ArenaTree<ElfFile>, working_node: usize, listing: ElfFile) {
    tree.add_node(listing, Some(working_node));
}

fn build_tree(lines: Vec<ParsedLine>) -> ArenaTree<ElfFile> {
    let mut tree: ArenaTree<ElfFile> = ArenaTree { arena: vec![] };
    let mut working_node = tree.add_node(
        ElfFile {
            name: "/".to_string(),
            t: ElfType::Dir,
            size: None,
        },
        None,
    );
    for line in lines {
        match line {
            ParsedLine::Operation(op) => match op {
                ElfOperation::MoveUp => {
                    working_node = tree.get_parent(working_node).unwrap();
                }
                ElfOperation::ListFiles => {
                    // Nothing to do
                }
                ElfOperation::ChangeDirectory(dir_name) => {
                    if dir_name == "/" {
                        working_node = 0;
                        continue;
                    }
                    let children = &tree.get_node(working_node).children;
                    let new_working_node = children
                        .iter()
                        .find(|&idx| tree.get_node(*idx).val.name == dir_name)
                        .unwrap();
                    working_node = *new_working_node;
                }
            },
            ParsedLine::Listing(listing) => handle_listing(&mut tree, working_node, listing),
        }
    }
    return tree;
}

fn compute_node_size(tree: &ArenaTree<ElfFile>, node: &Node<ElfFile>) -> u32 {
    if let Some(size) = node.val.size {
        size
    } else {
        node.children
            .iter()
            .map(|idx| compute_node_size(tree, tree.get_node(*idx)))
            .sum()
    }
}

fn main() {
    let lines = read_lines();
    let tree = build_tree(lines);
    let sizes = tree
        .arena
        .iter()
        .filter(|node| node.val.t == ElfType::Dir)
        .map(|node| compute_node_size(&tree, &node));
    let part_a: u32 = sizes.clone().filter(|&size| size <= 100000).sum();

    let total_used = compute_node_size(&tree, tree.get_node(0));
    let total_space = 70000000;
    let needed_size = 30000000;
    let need_to_delete = total_used - (total_space - needed_size);

    let mut sizes_vec: Vec<u32> = sizes.collect();
    sizes_vec.sort();
    println!("{:#?}", sizes_vec);
    let part_b = sizes_vec
        .into_iter()
        .find(|&size| size >= need_to_delete)
        .unwrap();

    println!("{}", part_a);
    println!("{}", need_to_delete);
    println!("{}", part_b);
}
