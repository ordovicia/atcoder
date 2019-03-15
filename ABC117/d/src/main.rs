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
    ($($args:tt)*) => { $($args)* };
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
    pub fn is_high(bits: u64, pos: usize) -> bool {
        bits & (1 << pos) != 0
    }

    pub fn len(mut bits: u64) -> usize {
        let mut p = 0;
        while bits != 0 {
            p += 1;
            bits >>= 1;
        }
        p
    }
}

fn main() {
    const BITS_LEN: usize = 40;

    input!(nums_num: usize, k: u64, nums: [u64; nums_num]);

    let ans_x_eq_k = (0..BITS_LEN)
        .map(|b| {
            let ones_num = nums.iter().filter(|&&n| bits::is_high(n, b)).count();
            let xor_cnt = if bits::is_high(k, b) {
                nums_num - ones_num
            } else {
                ones_num
            };
            (1 << b) * xor_cnt as u64
        })
        .sum::<u64>();

    let ans_x_lt_k = (0..BITS_LEN)
        .map(|diff_pos| {
            // Bits of X and K are same in positions [diff_pos + 1, k_len),
            // differ in diff_pos (i.e. k & (1 << diff_pos) = 1, x & (1 << diff_pos = 0)),
            // and may differ in [0, diff_pos).

            if !bits::is_high(k, diff_pos) {
                0
            } else {
                (0..BITS_LEN)
                    .map(|b| {
                        let ones_num = nums.iter().filter(|&&n| bits::is_high(n, b)).count();
                        let xor_cnt = if b < diff_pos {
                            cmp::max(ones_num, nums_num - ones_num)
                        } else if b == diff_pos {
                            ones_num
                        } else {
                            if bits::is_high(k, b) {
                                nums_num - ones_num
                            } else {
                                ones_num
                            }
                        };

                        (1 << b) * xor_cnt as u64
                    })
                    .sum::<u64>()
            }
        })
        .max()
        .unwrap();

    let ans = vmax!(ans_x_eq_k, ans_x_lt_k, nums.iter().sum::<u64>());
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
