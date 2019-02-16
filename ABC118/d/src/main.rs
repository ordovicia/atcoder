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
    static MATCHSTICK_NUM_NEEDS: [usize; 10] = [
        0, // unused
        2, 5, 5, 4, 5, 6, 3, 7, 6,
    ];

    input!(
        matchstick_num: usize,
        numbers_num: usize,
        mut numbers: [i32; numbers_num]
    );

    let digits_num_max = matchstick_num / 2;
    numbers.sort_by(|n1, n2| n2.cmp(&n1));

    // dp[i]: the max #digit of numbers that can be made with i matchsticks.
    let mut dp = vec![None; matchstick_num + 1];
    dp[0] = Some(0);
    for i in 1..(matchstick_num + 1) {
        dp[i] = numbers
            .iter()
            .filter_map(|n| {
                let needs = MATCHSTICK_NUM_NEEDS[*n as usize];
                if i < needs {
                    None
                } else {
                    dp[i - needs].map(|d| d + 1)
                }
            })
            .max();
    }

    let mut ans = Vec::with_capacity(digits_num_max);
    let mut matchsticks_num = matchstick_num;

    'for_digit: while matchsticks_num > 0 {
        for n in &numbers {
            let needs = MATCHSTICK_NUM_NEEDS[*n as usize];
            if matchsticks_num < needs {
                continue;
            }

            if let (Some(d1), Some(d2)) = (dp[matchsticks_num - needs], dp[matchsticks_num]) {
                if d1 == d2 - 1 {
                    ans.push(n);
                    matchsticks_num -= needs;
                    continue 'for_digit;
                }
            }
        }
    }

    for n in ans {
        print!("{}", n);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
