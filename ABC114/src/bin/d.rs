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

fn next_prime(mut x: u64) -> u64 {
    while {
        x += 1;
        !is_prime(x)
    } {}
    x
}

fn is_prime(x: u64) -> bool {
    match x {
        0 | 1 => false,
        2 => true,
        _ => {
            let ceil = (x as f64).sqrt().ceil() as u64;
            (2..(ceil + 1)).all(|i| x % i != 0)
        }
    }
}

fn factorize(n_fact: u64) -> Vec<usize> {
    let prime_max = {
        let (mut pm, mut tmp) = (1, 1);
        while {
            pm = tmp;
            tmp = next_prime(pm);
            tmp <= n_fact
        } {}
        pm
    };
    let mut pows = vec![0; prime_max as usize + 1];

    for i in 2..(n_fact + 1) {
        let mut cur = i;
        let mut prime = 1;

        while {
            prime = next_prime(prime);
            prime <= i
        } {
            while cur % prime == 0 {
                pows[prime as usize] += 1;
                cur /= prime;
            }
        }
    }

    pows
}

fn main() {
    input!(n: u64);

    // 75 = 3 * 5 * 5
    // - p^2 q^4 r^4
    // - p^2 q^24
    // - p^4 q^14
    // - p^74

    let pows = factorize(n);
    let pow_count_ge = |rhs: usize| -> usize { pows.iter().filter(|c| **c >= rhs).count() };

    let ans = pow_count_ge(74)
        + pow_count_ge(14) * (pow_count_ge(4) - 1)
        + pow_count_ge(24) * (pow_count_ge(2) - 1)
        + (pow_count_ge(4) * (pow_count_ge(4) - 1) * (pow_count_ge(2) - 2)) / 2;

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
