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

    const N: usize = 500;

    let (_, m, q) = {
        let nmq = parse_vec::<usize>();
        (nmq[0], nmq[1], nmq[2])
    };

    // #Trains running range [i, j]
    let mut trains_num = [[0; N + 1]; N + 1];
    for _ in 0..m {
        let lr = parse_vec::<usize>();
        trains_num[lr[0]][lr[1]] += 1;
    }

    // Cumulative #trains running in range [i, 1], [i, 2], .., [i, j]
    let mut trains_num_cum = [[0; N + 1]; N + 1];
    for i in 1..(N + 1) {
        trains_num_cum[i][1] = trains_num[i][1];
        for j in 2..(N + 1) {
            trains_num_cum[i][j] = trains_num_cum[i][j - 1] + trains_num[i][j];
        }
    }

    for _ in 0..q {
        let query = parse_vec::<usize>();
        let ans = (query[0]..(query[1] + 1))
            .map(|i| trains_num_cum[i][query[1]] - trains_num_cum[i][query[0] - 1])
            .sum::<usize>();

        println!("{}", ans);
    }
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
