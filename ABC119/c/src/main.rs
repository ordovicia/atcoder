#![allow(unused)]

use std::{cell, cmp, collections, fmt, ops};

use cell::{Cell, RefCell};
use cmp::Ordering::{self, Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};
use ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
};

// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (src = $s:expr, $($r:tt)*) => {
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
        input!(src = s, $($r)*)
    };
}

macro_rules! inputln {
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
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
        let $var = _parse_value!($iter, $t);
        _input_inner!($iter $($r)*)
    };

    ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = _parse_value!($iter, $t);
        _input_inner!($iter $($r)*)
    };
}

macro_rules! _parse_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(_parse_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| _parse_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        _parse_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        _parse_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().expect("No item left").parse::<$t>().expect("Parse error")
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
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    pub fn gcd_list(a: &[u64]) -> u64 {
        a.iter().fold(a[0], |a, b| gcd(a, *b))
    }

    pub fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    pub fn lcm_list(a: &[u64]) -> u64 {
        a.iter().fold(a[0], |a, b| lcm(a, *b))
    }
}

macro_rules! vmin {
    ($x:expr, $y:expr) => { cmp::min($x, $y) };
    ($x:expr, $($args:expr),*) => { cmp::min($x, vmin!($($args),*)) };
}

macro_rules! vmax {
    ($x:expr, $y:expr) => { cmp::max($x, $y) };
    ($x:expr, $($args:expr),*) => { cmp::max($x, vmax!($($args),*)) };
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
        banboo_num: usize,
        target_lens: [i32; 3],
        banboo_lens: [i32; banboo_num]
    );

    let ans = dfs(&banboo_lens, &target_lens, 0, &[0; 1 + 3]).unwrap();
    println!("{}", ans);
}

fn dfs(
    banboo_lens: &[i32],
    target_lens: &[i32],
    idx: usize,
    concat_lens: &[i32; 1 + 3],
) -> Option<i32> {
    if idx == banboo_lens.len() {
        if concat_lens.iter().skip(1).all(|l| *l != 0) {
            let mp = target_lens
                .iter()
                .zip(concat_lens.iter().skip(1))
                .map(|(t, c)| (t - c).abs())
                .sum::<i32>()
                - 10 * 3;
            Some(mp)
        } else {
            None
        }
    } else {
        (0..4)
            .filter_map(|i| {
                let mut concat_lens = concat_lens.clone();
                concat_lens[i] += banboo_lens[idx];
                dfs(banboo_lens, target_lens, idx + 1, &concat_lens)
                    .map(|mp| mp + 10 * (i > 0) as i32)
            })
            .min()
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
