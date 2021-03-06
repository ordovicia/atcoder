#![allow(unused)]

use std::{cell, cmp, collections, fmt, ops};

use cell::{Cell, RefCell};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};

// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[cfg(feature = "debug")]
macro_rules! dbg {
    ($($args: tt)*) => { eprint!($($args)*) };
}

#[cfg(feature = "debug")]
macro_rules! dbgln {
    ($($args: tt)*) => { eprintln!($($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! dbg {
    ($($args:tt)*) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! dbgln {
    ($($args:tt)*) => {};
}

mod num {
    //
}

macro_rules! vmin {
    ($x: expr, $y: expr) => { cmp::min($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::min($x, vmin!($($args),*)) };
}

macro_rules! vmax {
    ($x: expr, $y: expr) => { cmp::max($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::max($x, vmax!($($args),*)) };
}

mod bits {
    pub fn is_high(bits: u64, pos: u64) -> bool {
        bits & (1 << pos) != 0
    }
}

fn main() {
    input!(n: i32);

    // N % 2^k
    //  mod_eq
    // (Sum_{i=0}^{k} { (-2)^i S_i }) % 2^k

    let mut s = Vec::with_capacity(32);
    let mut sum = 0;

    for i in 0.. {
        let pow_k = 1 << (i + 1);
        let pow_minus2 = (1 << i) * if i % 2 == 0 { 1 } else { -1 };

        if mod_euc(sum + pow_minus2, pow_k) == mod_euc(n, pow_k) {
            sum += pow_minus2;
            s.push(1);
        } else {
            s.push(0);
        }

        dbg!("{}, {:?}", sum, s);
        if sum == n {
            break;
        }
    }

    let s = s.iter().rev().map(|i| i.to_string()).collect::<String>();
    println!("{}", s);
}

fn mod_euc(n: i32, rhs: i32) -> i32 {
    let r = n % rhs;
    if r < 0 {
        if rhs > 0 {
            r + rhs
        } else {
            r - rhs
        }
    } else {
        r
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
