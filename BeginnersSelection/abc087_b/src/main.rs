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
    let num_500 = parse_line::<u32>();
    let num_100 = parse_line::<u32>();
    let num_50 = parse_line::<u32>();
    let x = parse_word::<u32>();

    let mut count = 0;
    let max_num_500 = cmp::min(num_500, x / 500);

    for i in 0..(max_num_500 + 1) {
        let remain_x = x - 500 * i;
        let max_num_100 = cmp::min(num_100, remain_x / 100);

        for j in 0..(max_num_100 + 1) {
            let remain_x = remain_x - 100 * j;

            if remain_x / 50 <= num_50 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
