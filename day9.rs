use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::*;
use std::rc::Rc;

#[derive(Debug)]
struct Marble {
    value: u32,
    prev: usize,
    next: usize,
}

impl Marble {
    fn new(value: u32) -> Self {
        Self {
            value,
            prev: 0,
            next: 0,
        }
    }
}

#[derive(Debug)]
struct Cycle {
    max_value: u32,
    current_value: u32,
    current_marble: usize,
    storage: Vec<Marble>,
    tombs: Vec<usize>,
}

impl Cycle {
    fn new(max: usize) -> Self {
        let mut v = Self {
            max_value: max as u32 + 1,
            current_value: 0,
            current_marble: 0,
            storage: Vec::with_capacity((max * 22 / 24) as usize),
            tombs: Vec::new(),
        };
        let marble = Marble::new(0);
        v.storage.push(marble);
        v
    }

    fn put_new(&mut self) {
        let prev_idx = self.storage[self.current_marble].next;
        let next_idx = self.storage[prev_idx].next;

        let new_idx = if let Some(reused_idx) = self.tombs.pop() {
            self.storage[reused_idx].value = self.current_value;
            reused_idx
        } else {
            self.storage.push(Marble::new(self.current_value));
            self.storage.len() - 1
        };

        let prev = &mut self.storage[prev_idx];
        prev.next = new_idx;

        let new = &mut self.storage[new_idx];
        new.prev = prev_idx;
        new.next = next_idx;

        let next = &mut self.storage[next_idx];
        next.prev = new_idx;

        self.current_marble = new_idx;
    }

    fn take_another(&mut self) -> u32 {
        let mut remove_idx = self.current_marble;
        for _i in 0..7 {
            remove_idx = self.storage[remove_idx].prev;
        }

        let prev_idx = self.storage[remove_idx].prev;
        let next_idx = self.storage[remove_idx].next;

        self.tombs.push(remove_idx);

        let prev = &mut self.storage[prev_idx];
        prev.next = next_idx;

        let next = &mut self.storage[next_idx];
        next.prev = prev_idx;

        self.current_marble = next_idx;

        self.storage[remove_idx].value
    }
}

impl Iterator for Cycle {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_value += 1;
        if self.current_value < self.max_value {
            if self.current_value % 23 == 0 {
                let mut score = self.current_value;
                score += self.take_another();
                Some(score)
            } else {
                self.put_new();
                Some(0)
            }
        } else {
            None
        }
    }
}

fn part1() {
    const PLAYERS: usize = 432;
    const MARBLES: usize = 7101900;

    let mut scores: Vec<u32> = vec![0; PLAYERS];
    let cycle = Cycle::new(MARBLES);

    let mut player = (0..PLAYERS).cycle();

    let mut result = 0;
    for score in cycle {
        let id = player.next().unwrap();
        scores[id] += score;
        result = max(scores[id], result);
    }

    println!("{}", result);
}

fn main() {
    part1();
}
