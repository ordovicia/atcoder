#![allow(unused)]

use std::{cell, cmp, collections, convert, fmt, io, iter, ops, str};

use cell::{Cell, RefCell};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use io::{Read, Write};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($($args: tt)*) => { eprint!($($args)*) };
}

#[cfg(feature = "debug")]
macro_rules! debugln {
    ($($args: tt)*) => { eprintln!($($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($($args:tt)*) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! debugln {
    ($($args:tt)*) => {};
}

macro_rules! vmax {
    ($x: expr, $y: expr) => { cmp::max($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::max($x, vmax!($($args),*)) };
}

macro_rules! vmin {
    ($x: expr, $y: expr) => { cmp::min($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::min($x, vmin!($($args),*)) };
}

fn main() {
    let withdrawal = parse_line::<usize>();

    // dp[0] = 0
    // dp[i] = 1 + min(dp[i - 1],
    //                 dp[i - 6], dp[i - 6^2], ...,
    //                 dp[i - 9], dp[i - 9^2], ...)

    let mut ops_num_min = [0; 100001];

    for money in 1..(withdrawal + 1) {
        let mut ops_num = ops_num_min[money - 1];

        let mut m = 6;
        while money >= m {
            ops_num = cmp::min(ops_num, ops_num_min[money - m]);
            m *= 6;
        }

        m = 9;
        while money >= m {
            ops_num = cmp::min(ops_num, ops_num_min[money - m]);
            m *= 9;
        }

        ops_num_min[money] = ops_num + 1;
        debugln!("{:6}: {}", money, ops_num_min[money]);
    }

    println!("{}", ops_num_min[withdrawal]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
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

        if !buf.is_empty() {
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

fn next_prime(x: i32) -> i32 {
    for i in (x + 1).. {
        if is_prime(i) {
            return i;
        }
    }

    panic!("attempt to add with overflow");
}

fn is_prime(x: i32) -> bool {
    let ceil = f64::from(x).sqrt().ceil() as i32;
    x == 2 || (2..(ceil + 1)).all(|i| x % i != 0)
}

fn bit_high(bits: u64, pos: u64) -> bool {
    bits & (1 << pos) != 0
}
