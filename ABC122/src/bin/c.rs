#![allow(unused)]

use std::{cell, cmp, collections, fmt, ops};

use cell::{Cell, RefCell};
use cmp::Ordering;
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use fmt::{Debug, Display};
use ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
};

use lib::*;

fn main() {
    input!(
        _n: usize,
        query_num: usize,
        s: String,
        queries: [(usize, usize); query_num]
    );

    let ac_num_prefix = s
        .chars()
        .scan((None, 0), |acc, c| {
            if let Some('A') = acc.0 {
                if c == 'C' {
                    acc.1 += 1;
                }
            }

            acc.0 = Some(c);
            Some(*acc)
        })
        .map(|(_, p)| p)
        .collect::<Vec<_>>();

    for (l, r) in queries {
        let ans = ac_num_prefix[r - 1] - ac_num_prefix[l - 1];
        println!("{}", ans);
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
