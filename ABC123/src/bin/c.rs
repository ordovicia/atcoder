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
    input!(people_num: usize, capacities: [usize; 5]);

    let bandwidth = capacities.iter().min().unwrap();
    let ans = (people_num + bandwidth - 1) / bandwidth + 4;

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
