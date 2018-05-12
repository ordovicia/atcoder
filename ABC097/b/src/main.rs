#![allow(unused)]

use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use io::{Read, Write};
use std::{cmp, collections, fmt, io, iter, ops, str};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt: expr) => { eprintln!($fmt) };
    ($fmt: expr, $($args: tt)*) => { eprintln!($fmt, $($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt:expr) => {};
    ($fmt:expr, $($args:tt)*) => {};
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
    const MAX: i32 = 1000;

    let x = parse_line::<i32>();

    let mut is_pow = vec![false; MAX as usize + 1];
    is_pow[1] = true;

    for b in 2..(MAX + 1) {
        for p in (2..).take_while(|p| b.pow(*p) <= MAX) {
            is_pow[b.pow(p) as usize] = true;
        }
    }

    let ans = (1..(x + 1))
        .rev()
        .filter(|&i| is_pow[i as usize])
        .next()
        .unwrap();
    println!("{}", ans);
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

fn next_prime(x: i32) -> i32 {
    for i in (x + 1).. {
        if is_prime(i) {
            return i;
        }
    }

    panic!("attempt to add with overflow");
}

fn is_prime(x: i32) -> bool {
    let ceil = (x as f64).sqrt().ceil() as i32;
    x == 2 || (2..(ceil + 1)).all(|i| x % i != 0)
}
