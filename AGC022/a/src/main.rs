#![allow(unused)]

use std::{cmp, collections, fmt, io, iter, ops, str};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use io::{Read, Write};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt: expr) => { eprintln!($fmt) };
    ($fmt: expr, $($args: tt)*) => { eprintln!($fmt, $($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt: expr) => {};
    ($fmt: expr, $($args: tt)*) => {};
}

macro_rules! vmax {
    ($x: expr, $y: expr) => { cmp::max($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::max($x, vmax!($($args),*)) };
}

macro_rules! vmin {
    ($x: expr, $y: expr) => { cmp::min($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::min($x, vmin!($($args),*)) };
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
    let s = read_line();
    let s_chars = s.chars().collect::<Vec<_>>();

    let s_next = if s.len() < 26 {
        let mut used_char = (b'a'..b'z' + 1)
            .map(|c| (c as char, false))
            .collect::<BTreeMap<_, _>>();

        for ref c in s_chars {
            let c = used_char.get_mut(c).unwrap();
            *c = true;
        }

        let (min_unused_char, _) = used_char.iter().filter(|c| !*c.1).min().unwrap();
        format!("{}{}", s, min_unused_char)
    } else if s == "zyxwvutsrqponmlkjihgfedcba" {
        String::from("-1")
    } else {
        //    a ... v w z y x
        //              ^^^^^ descending suffix. x = min. [z, y, x] s.t. > w
        // -> a ... v x
        //
        //    a ... v w y z x
        //                ^^^ descending suffix. z = min [z, x] s.t. > y
        // -> a ... v w z

        let mut desc_idx = 25;

        {
            let mut desc_char = s_chars[desc_idx];
            loop {
                assert!(desc_idx > 0);

                let c = s_chars[desc_idx - 1];
                if c < desc_char {
                    break;
                }

                desc_char = c;
                desc_idx -= 1;
            }
        }

        let desc_prev_char = s_chars[desc_idx - 1];
        let desc_char_min = s_chars[desc_idx..]
            .iter()
            .filter(|c| **c > desc_prev_char)
            .min()
            .unwrap();

        let mut s = s;
        s.truncate(desc_idx - 1);

        format!("{}{}", s, desc_char_min)
    };

    println!("{}", s_next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
