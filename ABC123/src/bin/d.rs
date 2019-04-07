#![allow(unused)]

use std::{cell, cmp, collections, fmt, iter, ops};

use cell::{Cell, RefCell};
use cmp::Ordering;
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};
use ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
};
use std::str::FromStr;

use lib::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct CakeCombination {
    delicious: u64,
    idx: [usize; 3],
}

impl PartialOrd for CakeCombination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CakeCombination {
    fn cmp(&self, other: &Self) -> Ordering {
        self.delicious.cmp(&other.delicious)
    }
}

fn main() {
    input!(
        one_num: usize,
        two_num: usize,
        three_num: usize,
        k: usize,
        mut ones: [u64; one_num],
        mut twos: [u64; two_num],
        mut threes: [u64; three_num]
    );

    ones.sort_by(|x0, x1| x1.cmp(&x0));
    twos.sort_by(|x0, x1| x1.cmp(&x0));
    threes.sort_by(|x0, x1| x1.cmp(&x0));
    let mut cakes = [ones, twos, threes];

    let mut q = BinaryHeap::new();
    let mut pushed = HashSet::new();

    q.push(CakeCombination {
        delicious: (0..3).map(|i| cakes[i][0]).sum(),
        idx: [0, 0, 0],
    });

    for _ in 0..k {
        let c = q.pop().unwrap();
        println!("{}", c.delicious);

        for i in 0..3 {
            let mut idx = c.idx;
            idx[i] += 1;

            if d!(idx)[i] < cakes[i].len() && !pushed.contains(&idx) {
                q.push(CakeCombination {
                    delicious: (0..3).map(|i| cakes[i][idx[i]]).sum(),
                    idx: idx,
                });
                pushed.insert(idx);
            }
        }
    }
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
