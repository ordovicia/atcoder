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
    input!(n: i64);

    let mut cache = HashMap::new();
    println!("{}", dfs(n, 0, &mut cache));
}

fn dfs(n: i64, mut x: i64, cache: &mut HashMap<i64, usize>) -> usize {
    if x > n {
        0
    } else {
        if let Some(c) = cache.get(&x) {
            return *c;
        }

        let ans = dfs(n, x * 10 + 7, cache)
            + dfs(n, x * 10 + 5, cache)
            + dfs(n, x * 10 + 3, cache)
            + is_valid(x) as usize;
        cache.insert(x, ans);
        ans
    }
}

fn is_valid(mut x: i64) -> bool {
    let (mut seven, mut five, mut three) = (false, false, false);
    while x > 0 {
        match x % 10 {
            7 => seven = true,
            5 => five = true,
            3 => three = true,
            _ => return false,
        }
        x /= 10;
    }

    seven && five && three
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
