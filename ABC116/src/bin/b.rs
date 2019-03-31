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
    input!(s: i32);

    let mut results = HashSet::new();
    let mut a = s;
    results.insert(a);

    for i in 2.. {
        a = f(a);
        if results.contains(&a) {
            println!("{}", i);
            return;
        }
        results.insert(a);
    }
}

fn f(n: i32) -> i32 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
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
