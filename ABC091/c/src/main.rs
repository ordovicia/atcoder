#![allow(unused)]

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

fn main() {
    use std::cell::Cell;

    let n: usize = parse_line();

    struct Point {
        x: i32,
        y: i32,
    }

    struct RedPoint {
        point: Point,
        used: Cell<bool>,
    }

    let red_points = {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let p = parse_vec::<i32>();
            v.push(RedPoint {
                point: Point { x: p[0], y: p[1] },
                used: Cell::new(false),
            });
        }
        v.sort_by_key(|p| p.point.x);
        v
    };

    let blue_points = {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let p = parse_vec::<i32>();
            v.push(Point { x: p[0], y: p[1] });
        }
        v.sort_by_key(|p| p.x);
        v
    };

    let mut pair_count = 0;

    for blue in blue_points {
        let r = red_points
            .iter()
            .take_while(|r| r.point.x < blue.x)
            .filter(|r| r.point.y < blue.y && !r.used.get())
            .max_by_key(|r| r.point.y);

        if let Some(r) = r {
            r.used.set(true);
            pair_count += 1;
        }
    }

    println!("{}", pair_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
