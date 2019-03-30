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

const MOD: u64 = 1_000_000_007;
type Q<T> = VecDeque<T>;

fn main() {
    input!(n: usize);

    let last3 = vec!['T'; 3].iter().cloned().collect();
    let ans = dfs(n, 0, &last3, &mut vec![HashMap::new(); n]);

    println!("{}", ans);
}

fn dfs(n: usize, i: usize, last3: &Q<char>, cache: &mut [HashMap<Q<char>, u64>]) -> u64 {
    assert_eq!(last3.len(), 3);

    if i == n {
        return 1;
    }
    if let Some(c) = cache[i].get(last3) {
        return *c;
    }

    let mut ans = 0;
    for c in &['A', 'C', 'G', 'T'] {
        let mut last4 = last3.clone();
        last4.push_back(*c);
        if d!(is_valid(&last4)) {
            last4.pop_front(); // new last3
            ans = (ans + dfs(n, i + 1, &last4, cache)) % MOD;
        }
    }

    cache[i].insert(last3.clone(), ans);
    ans
}

fn is_valid(last4: &Q<char>) -> bool {
    assert_eq!(last4.len(), 4);

    (0..4).all(|i| {
        let mut s = last4.clone();
        if i > 0 {
            s.swap(i, i - 1);
        }
        s.into_iter().collect::<String>().find("AGC").is_none()
    })
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
