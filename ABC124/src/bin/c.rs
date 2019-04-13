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
    input!(s: chars);
    let tiles = s
        .into_iter()
        .map(|c| c.to_digit(10).expect("failed to parse digit"))
        .collect::<Vec<_>>();

    let ans = cmp::min(
        tiles
            .iter()
            .zip([0, 1].iter().cycle())
            .map(|(t, z)| t == z)
            .filter(|b| *b)
            .count(),
        tiles
            .iter()
            .zip([1, 0].iter().cycle())
            .map(|(t, z)| t == z)
            .filter(|b| *b)
            .count(),
    );

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
