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
    input!(goods_num: usize, mut prices: [i32; goods_num]);
    prices.sort_by(|p0, p1| p1.cmp(&p0));

    let ans = prices[0] / 2 + prices[1..].iter().sum::<i32>();
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
