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
        island_num: usize,
        demand_num: usize,
        mut demands: [(usize1, usize1); demand_num]
    );
    demands.sort_by_key(|d| d.1);

    let mut ans = 1;
    let mut removed_pre = demands[0].1 - 1;
    for d in &demands[1..] {
        if d.0 <= removed_pre {
            continue;
        }

        removed_pre = d.1 - 1;
        ans += 1;
    }

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
