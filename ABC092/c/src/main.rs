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

fn main() {
    let n = parse_line::<usize>();
    let spot_pos = {
        let mut v = Vec::with_capacity(n + 2);
        v.push(0);
        v.append(&mut parse_vec::<i32>());
        v.push(0);
        v
    };

    let sum = spot_pos
        .windows(2)
        .map(|path| (path[0] - path[1]).abs())
        .sum::<i32>();

    for path in spot_pos.windows(3) {
        if range_contains(path[0], path[2], path[1]) {
            println!("{}", sum);
        } else {
            let new_sum = sum - 2 * cmp::min((path[0] - path[1]).abs(), (path[1] - path[2]).abs());
            println!("{}", new_sum);
        }
    }
}

fn range_contains(from: i32, to: i32, target: i32) -> bool {
    (from <= target && target <= to) || (to <= target && target <= from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
