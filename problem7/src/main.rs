use std::{fs::File, io::Read, ops::Sub};

#[derive(Copy, Clone, PartialEq)]
enum FileType {
    Directory,
    File,
}

#[derive(Clone)]
struct FileData {
    node_type: FileType,
    total_size: usize,
    name: String,
}

type NodeId = usize;

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    children: Vec<NodeId>,
    parent: Option<NodeId>,
}

pub struct Arena<T: Clone> {
    nodes: Vec<Node<T>>,
}

impl Arena<FileData> {
    fn get_child_by_name(&self, node_id: NodeId, name: &str) -> Option<NodeId> {
        let node = self.get(node_id);
        for &child in &node.children {
            let child_node = self.get(child);
            if child_node.data.name.eq(name) {
                return Some(child);
            }
        }
        return None;
    }
}

impl<T: Clone> Arena<T> {
    fn spawn(&mut self, data: T, parent: Option<NodeId>) -> NodeId {
        let next_idx = self.nodes.len();
        self.nodes.push(Node {
            data,
            children: vec![],
            parent,
        });
        return next_idx;
    }
    fn get(&self, id: NodeId) -> Node<T> {
        return self.nodes[id].clone();
    }
    fn update(&mut self, id: NodeId, node: Node<T>) {
        self.nodes[id] = node;
    }
}

fn main() {
    let mut f = File::open("problem7/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut arena = Arena::<FileData> { nodes: vec![] };
    let root = arena.spawn(
        FileData {
            node_type: FileType::Directory,
            total_size: 0,
            name: "/".to_owned(),
        },
        None,
    );

    let mut curr_node_id = root;

    for line in buf.lines().skip(1) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                let name = tokens[2];
                if name == ".." {
                    curr_node_id = arena.get(curr_node_id).parent.unwrap();
                } else {
                    curr_node_id = arena.get_child_by_name(curr_node_id, name).unwrap();
                }
            }
        } else {
            // we know this is ls output
            let ftype = match tokens[0] {
                "dir" => FileType::Directory,
                _ => FileType::File,
            };

            let fname = tokens[1];
            if arena.get_child_by_name(curr_node_id, fname).is_none() {
                let fsize = match ftype {
                    FileType::Directory => 0,
                    FileType::File => str::parse(tokens[0]).unwrap(),
                };
                let new_node = arena.spawn(
                    FileData {
                        node_type: ftype,
                        name: fname.into(),
                        total_size: fsize,
                    },
                    Some(curr_node_id),
                );

                let mut curr_node = arena.get(curr_node_id);
                curr_node.children.push(new_node);
                arena.update(curr_node_id, curr_node);

                let mut recursive = Some(curr_node_id);
                while let Some(r) = recursive {
                    let mut updated = arena.get(r);
                    recursive = updated.parent.clone();
                    updated.data.total_size += fsize;
                    arena.update(r, updated);
                }
            }
        }
    }
    let ans_1: usize = arena
        .nodes
        .iter()
        .filter_map(|n| {
            match matches!(n.data.node_type, FileType::Directory) && n.data.total_size <= 100000 {
                true => Some(n.data.total_size),
                false => None,
            }
        })
        .sum();
    println!("Answer 1: {ans_1}");

    // Answer 2:
    let free_space: usize = 7e7 as usize - arena.get(root).data.total_size;
    let diff: usize = 3e7 as usize - free_space;

    let ans2 = arena
        .nodes
        .iter()
        .filter_map(|n| match matches!(n.data.node_type, FileType::Directory) {
            true => Some(n.data.total_size),
            false => None,
        })
        .filter(|n| n >= &diff)
        .min()
        .unwrap();
    println!("Answer 2: {ans2}");
}
