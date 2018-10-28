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
    input!(n: usize, nums: [i64; n]);

    let mut nums = nums;
    nums.sort();

    let mut aligned = VecDeque::new();;
    for i in (0..).take_while(|&i| i < n - 1 - i) {
        if i % 2 == 0 {
            aligned.push_front(nums[i]);
            aligned.push_back(nums[n - 1 - i]);
        } else {
            aligned.push_front(nums[n - 1 - i]);
            aligned.push_back(nums[i]);
        }
    }
    dbgln!("aligned: {:?}", aligned);

    let ans = if n % 2 == 0 {
        diff_sum(&aligned)
    } else {
        let rem = nums[n / 2];

        let mut aligned_l = aligned.clone();
        aligned_l.push_front(rem);
        dbgln!("aligned_l: {:?}", aligned_l);

        let mut aligned_r = aligned;
        aligned_r.push_back(rem);
        dbgln!("aligned_r: {:?}", aligned_r);

        cmp::max(diff_sum(&aligned_l), diff_sum(&aligned_r))
    };

    println!("{}", ans);
}

fn diff_sum(nums: &VecDeque<i64>) -> i64 {
    let mut sum = 0;
    for i in (0..).take_while(|i| i + 1 < nums.len()) {
        sum += (nums[i] - nums[i + 1]).abs();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
