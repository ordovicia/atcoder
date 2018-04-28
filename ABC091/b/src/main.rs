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
    // (string, count) pair
    let mut strings = HashMap::new();

    let blue_num = parse_line::<usize>();
    for _ in 0..blue_num {
        let s = read_line();
        let e = strings.entry(s).or_insert(0);
        *e += 1;
    }

    let red_num = parse_line::<usize>();
    for _ in 0..red_num {
        let s = read_line();
        let e = strings.entry(s).or_insert(0);
        *e -= 1;
    }

    debug!("{:?}", strings);

    let (mut max_string, mut max_count) = (None, 0);
    for (s, c) in &strings {
        if *c > max_count {
            max_string = Some(s);
            max_count = *c;
        }
    }

    println!("{}", max_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
