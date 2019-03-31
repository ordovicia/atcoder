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
    const MOD: u64 = 1_000_000_007;
    input!(s: chars);

    let mut dp = [1, 0, 0, 0];

    for c in s {
        match c {
            'A' => dp[1] = dp[1] + dp[0],
            'B' => dp[2] = dp[2] + dp[1],
            'C' => dp[3] = dp[3] + dp[2],
            '?' => {
                dp[3] = 3 * dp[3] + dp[2];
                dp[2] = 3 * dp[2] + dp[1];
                dp[1] = 3 * dp[1] + dp[0];
                dp[0] = 3 * dp[0];
            }
            _ => unreachable!(),
        }

        for i in 0..4 {
            dp[i] %= MOD;
        }
    }

    println!("{}", dp[3]);
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
