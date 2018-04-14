#![allow(unused)]

#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, str};
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
    let n = parse_word::<usize>();
    let sum = parse_word::<u32>();

    if let Some((i, j, k)) = search_combination(n, sum) {
        println!("{} {} {}", i, j, k);
    } else {
        println!("-1 -1 -1");
    }
}

fn search_combination(n: usize, sum: u32) -> Option<(usize, usize, usize)> {
    for i in 0..(n + 1) {
        for j in 0..(n + 1 - i) {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == sum as usize {
                return Some((i, j, k));
            }
        }
    }

    None
}
