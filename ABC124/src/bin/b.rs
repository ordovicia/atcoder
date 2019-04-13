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

fn main() {
    input!(mountain_num: usize, heights: [i32; mountain_num]);

    let ans = heights
        .iter()
        .scan(0, |acc, &h| {
            *acc = cmp::max(*acc, h);
            Some((h, *acc))
        })
        .filter(|&(h, max)| h >= max)
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
