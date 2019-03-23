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

const BIT_LEN: usize = 20;

fn main() {
    input!(n: usize, a: [u64; n]);

    let mut high_bits = [false; BIT_LEN];

    let mut r = 0;
    let mut ans = 0;

    for l in 0..n {
        while r < n {
            if high_bits_saturated(&high_bits, a[r]) {
                clear_high_bits(&mut high_bits, a[l]);
                break;
            }

            accumulate_high_bits(&mut high_bits, a[r]);
            ans += r - l + 1;
            r += 1;
        }
    }

    println!("{}", ans);
}

fn high_bits_saturated(high_bits: &[bool], a: u64) -> bool {
    (0..BIT_LEN).any(|b| high_bits[b] && bits::is_high(a, b))
}

fn accumulate_high_bits(high_bits: &mut [bool], a: u64) {
    for b in 0..BIT_LEN {
        high_bits[b] |= bits::is_high(a, b);
    }
}

fn clear_high_bits(high_bits: &mut [bool], a: u64) {
    for b in 0..BIT_LEN {
        high_bits[b] &= !bits::is_high(a, b);
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
