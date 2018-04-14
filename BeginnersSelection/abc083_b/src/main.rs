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
    let n = parse_word::<u32>();
    let a = parse_word::<u32>();
    let b = parse_word::<u32>();

    let mut sum = 0;
    for i in 1..(n + 1) {
        let digit_sum = digit_sum(i);
        if digit_sum >= a && digit_sum <= b {
            sum += i;
        }
    }

    println!("{}", sum);
}

fn digit_sum(mut x: u32) -> u32 {
    let mut sum = 0;

    while x != 0 {
        sum += x % 10;
        x /= 10;
    }

    sum
}
