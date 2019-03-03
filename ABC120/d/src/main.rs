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

#[derive(Clone, Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    depth: Vec<usize>,
}

impl UnionFind {
    fn new(capacity: usize) -> Self {
        UnionFind {
            parent: (0..capacity).collect(),
            size: vec![1; capacity],
            depth: vec![0; capacity],
        }
    }

    fn size(&mut self, a: usize) -> usize {
        let p = self.find(a);
        self.size[p]
    }

    fn find(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            a
        } else {
            let p = self.parent[a];
            self.parent[a] = self.find(p);
            self.parent[a]
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return;
        }

        if self.depth[a] < self.depth[b] {
            self.parent[a] = b;
            self.size[b] += self.size[a];
        } else {
            self.parent[b] = a;
            self.size[a] += self.size[b];

            if self.depth[a] == self.depth[b] {
                self.depth[a] += 1;
            }
        }
    }
}

fn main() {
    inputln!(island_num: usize, bridge_num: usize);

    let mut bridges = Vec::with_capacity(bridge_num);
    for _ in 0..bridge_num {
        inputln!(from: usize, to: usize);
        bridges.push((from - 1, to - 1));
    }

    let mut islands_connected = UnionFind::new(island_num);

    let mut ans = Vec::with_capacity(bridge_num);
    ans.push(island_num * (island_num - 1) / 2);

    while let Some(bridge) = bridges.pop() {
        let last = *ans.last().unwrap();

        if islands_connected.same(bridge.0, bridge.1) {
            ans.push(last);
        } else {
            let size_0 = islands_connected.size(bridge.0);
            let size_1 = islands_connected.size(bridge.1);
            ans.push(last - size_0 * size_1);
        }

        islands_connected.union(bridge.0, bridge.1);
    }

    for a in ans.iter().rev().skip(1) {
        println!("{}", a);
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
