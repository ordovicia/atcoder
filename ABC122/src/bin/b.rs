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
    input!(s: String);

    let ans = (0..s.len())
        .map(|b| {
            (b..s.len())
                .map(|e| &s[b..(e + 1)])
                .filter(|s| {
                    s.chars()
                        .all(|c| c == 'A' || c == 'C' || c == 'G' || c == 'T')
                })
                .map(|s| s.len())
                .max()
                .unwrap_or(0)
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
