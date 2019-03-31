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
    input!(
        difficulty_max: usize,
        target_score: u64,
        problems: [(usize, u64); difficulty_max],
    );

    let ans = (0..(1 << difficulty_max))
        .filter_map(|solve_all| {
            let mut solves = 0;
            let mut score = 0;

            for b in 0..difficulty_max {
                if bits::is_high(solve_all, b) {
                    solves += problems[b].0;
                    score += 100 * (b + 1) as u64 * problems[b].0 as u64 + problems[b].1;
                }
            }

            'for_difficulty: for b in (0..difficulty_max)
                .rev()
                .filter(|b| !bits::is_high(solve_all, *b))
            {
                for _ in 0..(problems[b].0 - 1) {
                    if score >= target_score {
                        break 'for_difficulty;
                    }

                    solves += 1;
                    score += 100 * (b + 1) as u64;
                }
            }

            if score >= target_score {
                Some(solves)
            } else {
                None
            }
        })
        .min()
        .unwrap();

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
