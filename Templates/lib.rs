use std::{cmp, collections};

use cmp::Ordering;
use collections::BinaryHeap;

// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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
#[macro_export]
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
#[macro_export]
macro_rules! d {
    ($val:expr) => {
        $val
    };
}

/// ```
/// # use lib::*;
/// assert_eq!(vmin!(0, 1), 0);
/// assert_eq!(vmin!(3, 2, 1), 1);
/// ```
#[macro_export]
macro_rules! vmin {
    ($x:expr, $y:expr) => { std::cmp::min($x, $y) };
    ($x:expr, $($args:expr),*) => { std::cmp::min($x, vmin!($($args),*)) };
}

/// ```
/// # use lib::*;
/// assert_eq!(vmax!(0, 1), 1);
/// assert_eq!(vmax!(3, 2, 1), 3);
/// ```
#[macro_export]
macro_rules! vmax {
    ($x:expr, $y:expr) => { std::cmp::max($x, $y) };
    ($x:expr, $($args:expr),*) => { std::cmp::max($x, vmax!($($args),*)) };
}

/// ```
/// # use lib::*;
/// let v = vec![2, 1, 0, 3];
/// assert_eq!(prefix!(v).collect::<Vec<i32>>(), vec![0, 2, 3, 3, 6]);
/// ```
#[macro_export]
macro_rules! prefix {
    ($iter: expr) => {{
        use std::iter;

        iter::once(Default::default())
            .chain($iter.into_iter())
            .scan(Default::default(), |acc, x| {
                *acc += x;
                Some(*acc)
            })
    }};
}

pub mod num {
    /// ```
    /// # use lib::*;
    /// assert_eq!(num::gcd(2, 5), 1);
    /// assert_eq!(num::gcd(4, 6), 2);
    /// ```
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    /// ```
    /// # use lib::*;
    /// assert_eq!(num::gcd_slice(&[3]), 3);
    /// assert_eq!(num::gcd_slice(&[2, 5]), 1);
    /// assert_eq!(num::gcd_slice(&[4, 6, 8]), 2);
    /// ```
    pub fn gcd_slice(a: &[u64]) -> u64 {
        a.iter().fold(a[0], |a, b| gcd(a, *b))
    }

    /// ```
    /// # use lib::*;
    /// assert_eq!(num::lcm(2, 5), 10);
    /// assert_eq!(num::lcm(4, 6), 12);
    /// ```
    pub fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    /// ```
    /// # use lib::*;
    /// assert_eq!(num::lcm_slice(&[3]), 3);
    /// assert_eq!(num::lcm_slice(&[2, 5]), 10);
    /// assert_eq!(num::lcm_slice(&[4, 6, 8]), 24);
    /// ```
    pub fn lcm_slice(a: &[u64]) -> u64 {
        a.iter().fold(a[0], |a, b| lcm(a, *b))
    }
}

pub mod bits {
    /// ```
    /// # use lib::*;
    /// assert!(!bits::is_high(0b0100, 0));
    /// assert!(bits::is_high(0b0100, 2));
    /// ```
    pub fn is_high(bits: u64, pos: usize) -> bool {
        bits & (1 << pos as u64) != 0
    }

    /// ```
    /// # use lib::*;
    /// assert_eq!(bits::len(0b1010), 4);
    /// assert_eq!(bits::len(0b1), 1);
    /// assert_eq!(bits::len(0b0), 0);
    /// ```
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
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    depth: Vec<usize>,
}

/// ```
/// # use lib::*;
/// let mut uf = UnionFind::new(4);
/// assert_eq!(uf.size(0), 1);
/// assert_eq!(uf.find(0), 0);
///
/// uf.union(0, 1);
/// assert!(uf.same(0, 1));
/// assert!(uf.find(0) == 0 || uf.find(0) == 1);
/// assert_eq!(uf.size(0), 2);
///
/// uf.union(1, 2);
/// assert!(uf.same(0, 2));
/// assert!(uf.find(0) == 0 || uf.find(0) == 1); // not 2
/// assert_eq!(uf.size(0), 3);
/// ```
impl UnionFind {
    pub fn new(capacity: usize) -> Self {
        UnionFind {
            parent: (0..capacity).collect(),
            size: vec![1; capacity],
            depth: vec![0; capacity],
        }
    }

    pub fn size(&mut self, a: usize) -> usize {
        let p = self.find(a);
        self.size[p]
    }

    pub fn find(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            a
        } else {
            let p = self.parent[a];
            self.parent[a] = self.find(p);
            self.parent[a]
        }
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn union(&mut self, a: usize, b: usize) {
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

// std::cmp::Reverse is stable since 1.19
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Rev<T>(T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

/// ```
/// # use lib::*;
/// use std::collections::BinaryHeap;
///
/// let heap = [1, 0, 2, 3].iter().cloned().collect::<BinaryHeap<_>>();
/// let sorted = heap.into_iter_sorted().collect::<Vec<_>>();;
/// assert_eq!(sorted, vec![3, 2, 1, 0]);
/// ```
// BinaryHeap::iter() iterates values in arbitrary order.
pub struct IterSorted<T: Ord> {
    heap: BinaryHeap<T>,
}

pub trait IntoIteratorSorted {
    type Item;
    type IntoIterSorted: Iterator<Item = Self::Item>;

    fn into_iter_sorted(self) -> Self::IntoIterSorted;
}

impl<T: Ord> IntoIteratorSorted for BinaryHeap<T> {
    type Item = T;
    type IntoIterSorted = IterSorted<T>;

    fn into_iter_sorted(self) -> IterSorted<T> {
        IterSorted { heap: self }
    }
}

impl<T: Ord> Iterator for IterSorted<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}
