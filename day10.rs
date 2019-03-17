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

#[derive(Debug)]
struct Star {
    curr_x: i64,
    curr_y: i64,
    x_acc: i8,
    y_acc: i8,
}

impl Star {
    fn new(curr_x: i64, curr_y: i64, x_acc: i8, y_acc: i8) -> Self {
        Self {
            curr_x,
            curr_y,
            x_acc,
            y_acc,
        }
    }

    fn next_move(&mut self) {
        self.curr_x += self.x_acc as i64;
        self.curr_y += self.y_acc as i64;
    }

    fn index_from(&self, left: i64, top: i64, width: i64) -> Option<usize> {
        let val = (self.curr_x - left) + width * (self.curr_y - top);
        if val < 0 {
            None
        } else {
            Some(val as usize)
        }
    }
}

fn set_bounds(lower: &mut i64, upper: &mut i64, v: i64) {
    if v < *lower {
        *lower = v;
    } else if v > *upper {
        *upper = v;
    }
}

fn part12() {
    let mut r = LineReader::new();

    const X_START: usize = 10;
    const POS_END: usize = 6;
    const Y_START: usize = 18;
    const X_ACC_START: usize = 36;
    const Y_ACC_START: usize = 40;
    const ACC_END: usize = 2;

    let mut stars: Vec<Star> = Vec::new();

    while r.next_line() {
        if r.buf.len() < Y_ACC_START + ACC_END {
            continue;
        }
        println!("{}", r.buf);
        let x = r.buf[X_START..X_START + POS_END]
            .trim_start()
            .parse::<i64>()
            .unwrap();
        let y = r.buf[Y_START..Y_START + POS_END]
            .trim_start()
            .parse::<i64>()
            .unwrap();
        let x_acc = r.buf[X_ACC_START..X_ACC_START + ACC_END]
            .trim_start()
            .parse::<i8>()
            .unwrap();
        let y_acc = r.buf[Y_ACC_START..Y_ACC_START + ACC_END]
            .trim_start()
            .parse::<i8>()
            .unwrap();

        stars.push(Star::new(x, y, x_acc, y_acc));
    }

    println!("{}", stars.len());

    let mut output = File::create("/tmp/output.txt").unwrap();
    let mut message_observed = false;
    let mut canvas: Vec<char> = Vec::new();
    let mut count = 0;

    loop {
        let mut left = std::i64::MAX;
        let mut right = 0;
        let mut top = std::i64::MAX;
        let mut bottom = 0;

        count += 1;
        stars.iter_mut().for_each(|star| {
            star.next_move();
            set_bounds(&mut left, &mut right, star.curr_x);
            set_bounds(&mut top, &mut bottom, star.curr_y);
        });

        let hight = (bottom - top + 1) as usize;
        if hight / 2 < stars.len() {
            let width = (right - left + 1) as usize;
            message_observed = true;
            canvas.resize(width * hight, ' ');

            stars.iter().for_each(|star| {
                if let Some(idx) = star.index_from(left, top, width as i64) {
                    canvas[idx] = '#';
                }
            });

            canvas.iter_mut().enumerate().for_each(|(idx, v)| {
                if idx % width == 0 {
                    write!(output, "\n");
                }
                write!(output, "{}", v);
                *v = ' ';
            });
            write!(output, "\n{} seconds\n", count);
        } else {
            if message_observed {
                break;
            }
        }
    }
}

fn main() {
    part12();
}
