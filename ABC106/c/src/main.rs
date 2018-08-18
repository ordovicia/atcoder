#![allow(unused)]

use std::{cell, cmp, collections, fmt, ops};

use cell::{Cell, RefCell};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};

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

macro_rules! vmin {
    ($x: expr, $y: expr) => { cmp::min($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::min($x, vmin!($($args),*)) };
}

macro_rules! vmax {
    ($x: expr, $y: expr) => { cmp::max($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::max($x, vmax!($($args),*)) };
}

fn main() {
    use io::*;

    let s = read_line();
    let k = parse_line::<usize>();

    let ans = s
        .chars()
        .filter_map(|d| d.to_digit(10))
        .enumerate()
        .skip_while(|&(i, d)| d == 1 && i + 1 < k)
        .next()
        .unwrap()
        .1;

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

mod io {
    use std::io::{self, Read, Write};
    use std::str;

    pub fn read_line() -> String {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_owned()
    }

    pub fn read_word() -> String {
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

    pub fn parse_line<T: str::FromStr>() -> T {
        read_line().parse().ok().unwrap()
    }

    pub fn parse_word<T: str::FromStr>() -> T {
        read_word().parse().ok().unwrap()
    }

    pub fn parse_vec<T: str::FromStr>() -> Vec<T> {
        read_line()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect()
    }

    pub fn flush() {
        io::stdout().flush();
    }
}

mod num {
    use std::{cmp, ops};

    pub fn imin<T, I>(v: &T) -> &I
    where
        T: ops::Deref<Target = [I]>,
        I: cmp::Ord + Copy,
    {
        v.iter().min().unwrap()
    }

    pub fn imax<T, I>(v: &T) -> &I
    where
        T: ops::Deref<Target = [I]>,
        I: cmp::Ord + Copy,
    {
        v.iter().max().unwrap()
    }

    pub fn next_prime(x: i32) -> i32 {
        for i in (x + 1).. {
            if is_prime(i) {
                return i;
            }
        }

        panic!("attempt to add with overflow");
    }

    pub fn is_prime(x: i32) -> bool {
        let ceil = f64::from(x).sqrt().ceil() as i32;
        x == 2 || (2..(ceil + 1)).all(|i| x % i != 0)
    }
}

mod bits {
    pub fn bit_high(bits: u64, pos: u64) -> bool {
        bits & (1 << pos) != 0
    }
}
