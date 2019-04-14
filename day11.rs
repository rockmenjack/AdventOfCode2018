use regex::Regex;
use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::*;
use std::rc::Rc;

struct LineReader {
    buf: String,
    reader: BufReader<File>,
}

impl LineReader {
    pub fn new() -> Self {
        let file = File::open("/tmp/input").unwrap();
        LineReader {
            buf: String::new(),
            reader: BufReader::new(file),
        }
    }

    pub fn next_line(&mut self) -> bool {
        self.buf.clear();
        match self.reader.read_line(&mut self.buf) {
            Ok(n) => {
                if n == 0 {
                    false
                } else {
                    self.buf.trim_end();
                    true
                }
            }
            Err(_) => false,
        }
    }
}

const LENGTH_X: usize = 300;
const LENGTH_Y: usize = 300;
const SERIAL_NUMBER: i32 = 7315;

fn point_at(idx: usize) -> (usize, usize) {
    (idx % LENGTH_Y, idx / LENGTH_Y)
}

fn index_at(x: usize, y: usize) -> usize {
    x + y * LENGTH_X
}

fn power_level(x: usize, y: usize, sn: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let level = (rack_id * y as i32 + sn) * rack_id;
    let upper_digit = level / 1000;
    let power_level = level / 100 - upper_digit * 10 - 5;
    power_level
}

fn part1() {
    //let mut r = LineReader::new();

    let mut grid: Vec<i32> = vec![0; LENGTH_X * LENGTH_Y];
    grid.iter_mut().enumerate().for_each(|(i, cell)| {
        let (x, y) = point_at(i);
        let p = power_level(x, y, SERIAL_NUMBER);
        *cell = p;
    });

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_val = 0;

    grid.iter()
        .enumerate()
        .map(|(idx, _)| point_at(idx))
        .filter(|&(x, y)| x < LENGTH_X - 3 && y < LENGTH_Y - 3)
        .for_each(|(x, y)| {
            let mut sum = 0;
            for i in 0..=2 {
                for j in 0..=2 {
                    sum += grid[index_at(x + i, y + j)];
                }
            }
            if sum > max_val {
                max_val = sum;
                max_x = x;
                max_y = y;
            }
        });

    println!("x: {}, y: {}, max: {}", max_x, max_y, max_val);
}

fn part2() {
    //let mut r = LineReader::new();

    let mut grid: Vec<i32> = vec![0; LENGTH_X * LENGTH_Y];
    grid.iter_mut().enumerate().for_each(|(i, cell)| {
        let (x, y) = point_at(i);
        let p = power_level(x, y, SERIAL_NUMBER);
        *cell = p;
    });

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_val = 0;
    let mut size = 3;

    let mut last_sum = 0;
    for s in size..LENGTH_X {
        let mut last_y = LENGTH_Y + 1;
        grid.iter()
            .enumerate()
            .filter_map(|(idx, _)| {
                let (x, y) = point_at(idx);
                if x < LENGTH_X - s && y < LENGTH_Y - s {
                    Some((x, y))
                } else {
                    None
                }
            })
            .for_each(|(x, y)| {
                let mut sum;
                if last_y == y {
                    /*
                        if we are at the same row, we can just reuse previous result by subtracting and adding a column:
                        x-1 x   x+1 x+2 (assume: size = 3)
                        [-] [v] [v] [+]
                        [-] [v] [v] [+]
                        [-] [v] [v] [+]
                    */
                    sum = last_sum;
                    for j in 0..s {
                        sum += grid[index_at(x + s - 1, y + j)] - grid[index_at(x - 1, y + j)];
                    }
                } else {
                    // starting at a new row, compute full sum
                    sum = 0;
                    for i in 0..s {
                        for j in 0..s {
                            sum += grid[index_at(x + i, y + j)];
                        }
                    }
                }
                last_y = y;
                last_sum = sum;

                if sum > max_val {
                    max_val = sum;
                    max_x = x;
                    max_y = y;
                    size = s;
                }
            });
    }

    println!(
        "x: {}, y: {}, max: {}, size: {}",
        max_x, max_y, max_val, size
    );
}

fn main() {
    part1();
    part2();
}

#[cfg(test)]
#[test]
fn test_power_level() {
    let data = vec![(122, 79, 57, -5), (217, 196, 39, 0), (101, 153, 71, 4)];
    data.iter().for_each(|&(x, y, sn, expected)| {
        let p = power_level(x, y, sn);
        assert_eq!(p, expected, "expect {}, got {}", expected, p);
    });
}
