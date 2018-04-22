#![allow(unused)]

use std::{cmp, collections, fmt, io, iter, ops, str};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
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
    let a_price = parse_word::<i32>();
    let b_price = parse_word::<i32>();
    let ab_price = parse_word::<i32>();

    let a_num = parse_word::<i32>();
    let b_num = parse_word::<i32>();

    let calc_price = |a, b, ab| a * a_price + b * b_price + ab * ab_price;

    let min_total_price = (0..(cmp::max(a_num, b_num) + 1))
        .map(|ab_buy_half| {
            let a_buy = cmp::max(0, a_num - ab_buy_half);
            let b_buy = cmp::max(0, b_num - ab_buy_half);
            calc_price(a_buy, b_buy, ab_buy_half * 2)
        })
        .min()
        .unwrap();

    println!("{}", min_total_price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
