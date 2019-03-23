#![allow(unused)]

use std::{cell, cmp, collections, fmt, ops};

use cell::{Cell, RefCell};
use cmp::Ordering::{self, Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};
use ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
};

use lib::*;

fn main() {
    input!(
        n: usize,
        pair_num: usize,
        p: [i32; n],
        pairs: [(usize, usize); pair_num]
    );

    let mut uf = UnionFind::new(n + 1);
    for p in pairs {
        uf.union(p.0, p.1);
    }

    let ans = p
        .iter()
        .enumerate()
        .filter(|&(i, &p)| uf.same(i + 1, p as usize))
        .count();
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
