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
    input!(_n: usize, k: usize, s: String);

    let ranges = build_ranges(&s);
    let ranges_prefix = iter::once(0usize)
        .chain(ranges.iter().cloned())
        .scan(0, |acc, r| {
            *acc += r;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let ans = (0..ranges.len())
        // step_by(2)
        .filter_map(|start| {
            if start % 2 == 0 {
                let end = cmp::min(start + 2 * k + 1, ranges.len());
                Some(ranges_prefix[end] - ranges_prefix[start])
            } else {
                None
            }
        })
        .max()
        .unwrap();
    println!("{}", ans);
}

fn build_ranges(s: &str) -> Vec<usize> {
    let mut ranges = Vec::with_capacity(s.len());
    let (mut cur, mut cnt) = (1, 0);

    for c in s.chars() {
        let d = c.to_digit(10).expect("failed to parse digit");
        if d == cur {
            cnt += 1;
        } else {
            ranges.push(cnt);
            cur = d;
            cnt = 1;
        }
    }

    if cnt != 0 {
        ranges.push(cnt);
    }
    if ranges.len() % 2 == 0 {
        ranges.push(0);
    }

    ranges
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
