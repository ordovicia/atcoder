#![allow(unused)]

#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, ops, str};
use cmp::Ordering::*;
use collections::{HashMap, HashSet};
use io::{Read, Write};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt: expr) => { eprintln!($fmt) };
    ($fmt: expr, $($arg: tt)*) => { eprintln!($fmt, $($arg)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt: expr) => {};
    ($fmt: expr, $($arg: tt)*) => {};
}

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn read_word() -> String {
    const CAPACITY: usize = 16;

    let mut stdin = io::stdin();
    let mut byte: [u8; 1] = [0];

    loop {
        let mut buf = Vec::with_capacity(CAPACITY);

        loop {
            let res = stdin.read(&mut byte);

            if res.unwrap_or(0) == 0 || byte[0] <= b' ' {
                break;
            } else {
                buf.push(byte[0]);
            }
        }

        if buf.len() > 0 {
            return String::from_utf8(buf).unwrap();
        }
    }
}

fn parse_line<T: str::FromStr>() -> T {
    read_line().parse().ok().unwrap()
}

fn parse_word<T: str::FromStr>() -> T {
    read_word().parse().ok().unwrap()
}

fn parse_vec<T: str::FromStr>() -> Vec<T> {
    read_line()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn flush() {
    io::stdout().flush();
}

#[derive(Clone, Default)]
struct Destination {
    time: i32,
    x: i32,
    y: i32,
}

impl ops::Sub for Destination {
    type Output = Destination;

    fn sub(self, rhs: Destination) -> Self {
        Destination {
            time: self.time - rhs.time,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() {
    let n = parse_line::<usize>();

    let mut prev_dest = Destination::default();

    for _ in 0..n {
        let dest = {
            let txy = parse_vec();
            Destination {
                time: txy[0],
                x: txy[1],
                y: txy[2],
            }
        };

        let dest = dest - prev_dest;
        prev_dest = dest.clone();

        let distance = dest.x.abs() + dest.y.abs();
        if dest.time < distance {
            println!("No");
            return;
        }

        if distance % 2 != dest.time % 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
