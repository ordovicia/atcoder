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

    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = _read_value!($iter, $t);
        _input_inner!($iter $($r)*)
    };
}

macro_rules! _read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(_read_value!($iter, $t)),* )
    };

    // ($iter:expr, [ $t:tt ; $len:expr ]) => {
    //     (0..$len).map(|_| _read_value!($iter, $t)).collect::<Vec<_>>()
    // };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len)
            .map(|_| _read_value_option!($iter, $t))
            .take_while(|v| v.is_some())
            .map(|v| v.unwrap())
            .collect::<Vec<_>>()
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

macro_rules! _read_value_option {
    ($iter:expr, $t:ty) => {
        $iter.next().map(|v| v.parse::<$t>().expect("Parse error"))
    };
}

#[cfg(feature = "debug")]
macro_rules! d {
    ($val:expr) => {
        match $val {
            tmp => {
                eprintln!("[{}] {} = {:#?}", line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
}

#[cfg(not(feature = "debug"))]
macro_rules! d {
    ($val:expr) => {
        $val
    };
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

    /// len(0b1010) = 4
    /// len(0b1) = 1
    /// len(0b0) = 0
    pub fn len(bits: u64) -> usize {
        for b in 0..64 {
            if is_high(bits, 63 - b) {
                return (64 - b) as usize;
            }
        }

        return 0;
    }
}

fn main() {
    input!(
        people_num: usize,
        food_num: usize,
        favorites: [usize; people_num * (food_num + 1)]
    );
    let mut favorites = favorites.iter();

    const FOOD_NUM_MAX: usize = 30;
    let mut fav_count = [0_usize; FOOD_NUM_MAX];

    for _ in 0..people_num {
        let k = favorites.next().expect("no people left");
        for _ in 0..*k {
            let f = favorites.next().expect("no food left");
            fav_count[*f - 1] += 1;
        }
    }

    let ans = fav_count.iter().filter(|c| **c == people_num).count();
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
