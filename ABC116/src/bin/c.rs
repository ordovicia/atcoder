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
    input!(flower_num: usize, mut target_heights: [i32; flower_num]);
    println!("{}", solve(&target_heights));
}

fn solve(heights: &[i32]) -> i32 {
    if heights.is_empty() {
        0
    } else if let Some(z) = heights.iter().position(|&h| h == 0) {
        solve(&heights[0..z]) + solve(&heights[(z + 1)..])
    } else {
        let heights = heights.iter().map(|h| h - 1).collect::<Vec<_>>();
        solve(&heights) + 1
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
