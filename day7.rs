use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::*;

type Step = char;
type StepDependancies = Vec<Step>;

fn day7_part1() {
    let file = File::open("/tmp/input").unwrap();
    // let mut buf = Vec::new();
    let mut br = BufReader::new(file);
    let mut line = String::new();

    let mut parent_to_children: HashMap<Step, StepDependancies> = HashMap::new();
    let mut parent_counts: HashMap<Step, u8> = HashMap::new();

    let mut childrens: HashSet<Step> = HashSet::new();
    let mut all_nodes: HashSet<Step> = HashSet::new();
    loop {
        match br.read_line(&mut line) {
            Ok(n) if n == 0 => break,
            Ok(n) if n > 0 => {
                let a = line.as_bytes();
                let child = char::from(a[36]);
                let parent = char::from(a[5]);
                all_nodes.insert(parent);
                all_nodes.insert(child);
                childrens.insert(child);

                parent_to_children.entry(parent).or_default().push(child);
                let counter = parent_counts.entry(child).or_default();
                *counter += 1;
                println!("{} depends on {}, parents {}", child, parent, counter);

                line.clear();
            }
            _ => panic!("error while read"),
        }
    }

    let mut nodes: Vec<char> = all_nodes
        .difference(&childrens)
        .into_iter()
        .cloned()
        .collect();
    nodes.sort_by(|a, b| b.cmp(a));

    let mut result = String::new();
    while let Some(node) = nodes.pop() {
        result.push(node);
        if let Some(childrens) = parent_to_children.get_mut(&node) {
            print!("{} childrens: [", node);
            childrens.iter_mut().for_each(|&mut c| {
                if let Some(counter) = parent_counts.get_mut(&c) {
                    print!("{}:{}, ", c, counter);
                    match *counter {
                        0 => return,
                        1 => nodes.push(c),
                        _ => (),
                    }
                    *counter -= 1;
                }
            });
            println!("]");
        }
        nodes.sort_by(|a, b| b.cmp(a));
        println!("nodes: {:?}", nodes);
    }

    println!("{}", result);
}

// start of day7 part 2
struct ElfWorkers {
    elfs: Vec<(char, u8)>,
}

impl ElfWorkers {
    const IDLE: char = '*';

    fn new(num: usize) -> Self {
        Self {
            elfs: vec![(ElfWorkers::IDLE, 0); num],
        }
    }

    fn add(&mut self, step: char) -> bool {
        if let Some(idle_elf) = self.elfs.iter_mut().find(|elf| elf.0 == ElfWorkers::IDLE) {
            idle_elf.0 = step;
            idle_elf.1 = step as u8 - 64 + 60;
            println!("adding {}: will take {}", idle_elf.0, idle_elf.1);
            true
        } else {
            false
        }
    }

    fn next_completed_steps(&mut self) -> Option<(Vec<char>, u8)> {
        if let Some(freed_elf) = self
            .elfs
            .iter_mut()
            .filter(|elf| elf.0 != ElfWorkers::IDLE)
            .min_by_key(|elf| elf.1)
        {
            let closet_complete_sec = freed_elf.1;
            let mut completed: Vec<char> = Vec::new();
            self.elfs
                .iter_mut()
                .filter(|elf| elf.0 != ElfWorkers::IDLE)
                .for_each(|elf| {
                    elf.1 -= closet_complete_sec;
                    if elf.1 == 0 {
                        completed.push(elf.0);
                        elf.0 = ElfWorkers::IDLE;
                    }
                });
            println!(
                "{} secs after {:?} completes",
                closet_complete_sec, completed
            );
            Some((completed, closet_complete_sec))
        } else {
            println!("no more tasks");
            None
        }
    }
}

fn day7_part2() {
    let file = File::open("/tmp/input").unwrap();
    // let mut buf = Vec::new();
    let mut br = BufReader::new(file);
    let mut line = String::new();

    let mut parent_to_children: HashMap<Step, StepDependancies> = HashMap::new();
    let mut parent_counts: HashMap<Step, u8> = HashMap::new();

    let mut childrens: HashSet<Step> = HashSet::new();
    let mut all_nodes: HashSet<Step> = HashSet::new();
    loop {
        match br.read_line(&mut line) {
            Ok(n) if n == 0 => break,
            Ok(n) if n > 0 => {
                let a = line.as_bytes();
                let child = char::from(a[36]);
                let parent = char::from(a[5]);
                all_nodes.insert(parent);
                all_nodes.insert(child);
                childrens.insert(child);

                parent_to_children.entry(parent).or_default().push(child);
                let counter = parent_counts.entry(child).or_default();
                *counter += 1;
                println!("{} depends on {}, parents {}", child, parent, counter);

                line.clear();
            }
            _ => panic!("error while read"),
        }
    }

    let mut nodes: Vec<char> = all_nodes
        .difference(&childrens)
        .into_iter()
        .cloned()
        .collect();

    let mut elf_workers = ElfWorkers::new(5);
    let mut result = 0;
    loop {
        while let Some(node) = nodes.pop() {
            if !elf_workers.add(node) {
                break;
            }
        }

        // poll the elfs for state
        if let Some((done_steps, secs_taken)) = elf_workers.next_completed_steps() {
            result += secs_taken as u16;
            done_steps.iter().for_each(|parent| {
                if let Some(childrens) = parent_to_children.get_mut(&parent) {
                    childrens.iter_mut().for_each(|&mut c| {
                        if let Some(counter) = parent_counts.get_mut(&c) {
                            print!("{}:{}, ", c, counter);
                            match *counter {
                                0 => return,
                                1 => nodes.push(c),
                                _ => (),
                            }
                            *counter -= 1;
                        }
                    });
                }
            });
        } else {
            break;
        }
    }

    println!("{}", result);
}

fn main() {
    day7_part1();
    day7_part2();
}
