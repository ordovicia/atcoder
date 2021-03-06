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
        _input_inner!(iter, $($r)*)
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        _input_inner!(iter, $($r)*)
    };
}

macro_rules! _input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = _read_value!($iter, $t);
        _input_inner!($iter $($r)*)
    };
}

macro_rules! _read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(_read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| _read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        _read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        _read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[cfg(feature = "debug")]
macro_rules! d {
    ($($args: tt)*) => { dbg!($($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! d {
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
    input!(n: usize, lens: [i32; n]);

    let max = lens.iter().max().unwrap();
    let sum = lens.iter().sum::<i32>();

    if *max < sum - max {
        println!("Yes");
    } else {
        println!("No");
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
