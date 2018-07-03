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
    let kind_num = parse_word::<usize>();
    let eat_num = parse_word::<usize>();

    #[derive(Clone, Debug, Default)]
    struct Cake {
        beaut: i64,
        yum: i64,
        pop: i64,
    }

    impl Cake {
        fn new(beaut: i64, yum: i64, pop: i64) -> Self {
            Cake {
                beaut: beaut,
                yum: yum,
                pop: pop,
            }
        }
    }

    impl<'a> ops::Add<&'a Cake> for Cake {
        type Output = Self;

        fn add(self, rhs: &Cake) -> Self::Output {
            Self::new(
                self.beaut + rhs.beaut,
                self.yum + rhs.yum,
                self.pop + rhs.pop,
            )
        }
    }

    impl<'a> iter::Sum<&'a Cake> for Cake {
        fn sum<I: Iterator<Item = &'a Cake>>(iter: I) -> Cake {
            iter.fold(Cake::default(), |a, b| a + b)
        }
    }

    let cakes = {
        let mut cakes = Vec::with_capacity(kind_num);
        for _ in 0..kind_num {
            let v = parse_vec::<i64>();
            cakes.push(Cake::new(v[0], v[1], v[2]));
        }
        cakes
    };

    fn pn(bits: u64, pos: u64) -> i64 {
        if bit_high(bits, pos) {
            1
        } else {
            -1
        }
    }

    let ans = (0..8)
        .map(|b| {
            let mut cakes = cakes.clone();
            cakes.sort_by_key(|c| pn(b, 0) * c.beaut + pn(b, 1) * c.yum + pn(b, 2) * c.pop);
            let sum = cakes.iter().take(eat_num).sum::<Cake>();
            sum.beaut.abs() + sum.yum.abs() + sum.pop.abs()
        })
        .max()
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

fn next_prime(x: i32) -> i32 {
    for i in (x + 1).. {
        if is_prime(i) {
            return i;
        }
    }

    panic!("attempt to add with overflow");
}

fn is_prime(x: i32) -> bool {
    let ceil = f64::from(x).sqrt().ceil() as i32;
    x == 2 || (2..(ceil + 1)).all(|i| x % i != 0)
}

fn bit_high(bits: u64, pos: u64) -> bool {
    bits & (1 << pos) != 0
}
