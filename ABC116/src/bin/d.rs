#![allow(unused)]

use std::{cell, cmp, collections, fmt, iter, ops};

use cell::{Cell, RefCell};
use cmp::Ordering;
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};
use ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
};

use lib::*;

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

macro_rules! prefix {
    ($iter: expr) => {
        iter::once(Default::default())
            .chain($iter.into_iter())
            .scan(Default::default(), |acc, x| {
                *acc += x;
                Some(*acc)
            })
    };
}

struct IterSorted<T: Ord> {
    heap: BinaryHeap<T>,
}

trait IntoIteratorSorted {
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

impl<T: Ord> iter::Iterator for IterSorted<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

fn main() {
    inputln!(sushi_num: usize, eat_num: usize);

    let mut sushis = vec![BinaryHeap::new(); sushi_num];
    for _ in 0..sushi_num {
        inputln!(t: usize1, d: u64);
        sushis[t].push(d);
    }

    let bests = sushis
        .iter_mut()
        .filter_map(|s| s.pop())
        .collect::<BinaryHeap<_>>();

    // let remainings = sushis
    //     .into_iter()
    //     .map(|s| s.into_iter())
    //     .flatten()
    //     .collect::<BinaryHeap<_>>();
    let mut remainings = BinaryHeap::new();
    for s in sushis {
        for d in s {
            remainings.push(d);
        }
    }

    let bests_prefix = prefix!(bests.into_iter_sorted()).collect::<Vec<u64>>();
    let remainings_prefix = prefix!(remainings.into_iter_sorted()).collect::<Vec<u64>>();

    let ans = (1..bests_prefix.len())
        .filter_map(|k| {
            if eat_num - d!(k) < remainings_prefix.len() {
                Some(d!(bests_prefix[k]) + d!(remainings_prefix[eat_num - k]) + (k * k) as u64)
            } else {
                None
            }
        })
        .max()
        .unwrap();

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test() {
        //
    }
}
