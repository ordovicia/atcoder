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

fn main() {
    input!(durs: [i32; 5]);
    println!("{}", dfs(&durs, 0, 0));
}

fn dfs(durs: &[i32], clock: i32, ordered: u64) -> i32 {
    (0..5)
        .filter(|m| !bits::is_high(ordered, *m as usize))
        .map(|m| dfs(durs, ceil_10(clock) + durs[m], ordered | (1 << m)))
        .min()
        .unwrap_or(clock)
}

fn ceil_10(clock: i32) -> i32 {
    (clock + 9) / 10 * 10
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
