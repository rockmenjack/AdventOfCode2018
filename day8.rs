use std::fs::File;
use std::io::*;
use std::str;

#[derive(Debug)]
struct Node1 {
    children: Vec<usize>,
    meta: usize,
}

impl Node1 {
    fn new(n_ch: usize, n_mt: usize) -> Self {
        Self {
            children: Vec::with_capacity(n_ch),
            meta: n_mt,
        }
    }
}

type NodeIndex = usize;

fn part1() {
    let file = File::open("/tmp/input").unwrap();
    let br = BufReader::new(file);

    let data: Vec<usize> = br
        .split(' ' as u8)
        .filter_map(|v| unsafe {
            str::from_utf8_unchecked(&v.unwrap())
                .trim_end()
                .parse::<usize>()
                .ok()
        })
        .collect();
    let mut data = data.iter();

    let mut nodes: Vec<Node1> = Vec::new();
    let mut stack: Vec<NodeIndex> = Vec::new();

    nodes.push(Node1::new(*data.next().unwrap(), *data.next().unwrap()));
    stack.push(0);

    let mut result = 0;
    while let Some(idx) = stack.pop() {
        let next_node_index = nodes.len();
        let node = &mut nodes[idx];
        let new_child_node = node.children.capacity() > node.children.len();

        if new_child_node {
            {
                node.children.push(next_node_index);
            }
            stack.push(idx);
            nodes.push(Node1::new(*data.next().unwrap(), *data.next().unwrap()));
            stack.push(next_node_index);
        } else {
            while node.meta > 0 {
                result += *data.next().unwrap() as u32;
                node.meta -= 1;
            }
        }
    }

    println!("{}", result);
}

struct Node2 {
    children: Vec<usize>,
    meta: (usize, u32),
}

impl Node2 {
    fn new(n_ch: usize, n_mt: usize) -> Self {
        Self {
            children: Vec::with_capacity(n_ch),
            meta: (n_mt, 0),
        }
    }
}

fn part2() {
    let file = File::open("/tmp/input").unwrap();
    let br = BufReader::new(file);

    let data: Vec<usize> = br
        .split(' ' as u8)
        .filter_map(|v| unsafe {
            str::from_utf8_unchecked(&v.unwrap())
                .trim_end()
                .parse::<usize>()
                .ok()
        })
        .collect();
    let mut data = data.iter();

    let mut nodes: Vec<Node2> = Vec::new();
    let mut stack: Vec<NodeIndex> = Vec::new();

    nodes.push(Node2::new(*data.next().unwrap(), *data.next().unwrap()));
    stack.push(0);

    while let Some(idx) = stack.pop() {
        let next_node_index = nodes.len();
        let new_child_node = nodes[idx].children.capacity() > nodes[idx].children.len();

        if new_child_node {
            nodes[idx].children.push(next_node_index);
            stack.push(idx);
            nodes.push(Node2::new(*data.next().unwrap(), *data.next().unwrap()));
            stack.push(next_node_index);
        } else {
            println!(
                "{} has {:?}, meta {}",
                idx, nodes[idx].children, nodes[idx].meta.0
            );
            while nodes[idx].meta.0 > 0 {
                let v = *data.next().unwrap();
                if nodes[idx].children.len() == 0 {
                    nodes[idx].meta.1 += v as u32;
                } else {
                    let nth_child = v - 1;
                    println!("{} meta refer to {}", idx, nth_child);
                    if nodes[idx].children.capacity() >= v {
                        let child_idx = nodes[idx].children[nth_child];
                        nodes[idx].meta.1 += nodes[child_idx].meta.1;
                    }
                }
                nodes[idx].meta.0 -= 1;
            }
            println!("{} value is {}", idx, nodes[idx].meta.1);
        }
    }

    println!("{}", nodes[0].meta.1);
}

fn main() {
    part1();
    part2();
}
