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
    let n = parse_word::<usize>();
    let k = parse_word::<usize>();

    // N (> 0) ranges cover (K + (N - 1) * (K - 1)) length.
    // Thus, N = ceil((len - K) / (K - 1)) + 1.
    println!("{}", if n == k { 1 } else { (n - k - 1) / (k - 1) + 2 });
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
