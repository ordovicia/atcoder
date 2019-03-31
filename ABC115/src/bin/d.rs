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
    const LEVEL_MAX: usize = 50;
    input!(level: usize, eat_num: i64);

    let heights = (0..(LEVEL_MAX + 1))
        .scan(1, |acc, x| {
            let pre = *acc;
            *acc = 2 * *acc + 3;
            Some(pre)
        })
        .collect::<Vec<_>>();

    let patty_num = (0..(LEVEL_MAX + 1))
        .scan(1, |acc, x| {
            let pre = *acc;
            *acc = 2 * *acc + 1;
            Some(pre)
        })
        .collect::<Vec<_>>();

    let ans = patty_eat_num(level, eat_num, &heights, &patty_num);
    println!("{}", ans);
}

fn patty_eat_num(level: usize, eat_num: i64, heights: &[i64], patty_num: &[usize]) -> usize {
    match (eat_num, level) {
        (e, _) if e <= 0 => 0,
        (_, 0) => 1,
        (e, _) if e <= 1 + heights[level - 1] => {
            patty_eat_num(level - 1, eat_num - 1, heights, patty_num)
        }
        _ => {
            patty_num[level - 1]
                + 1
                + patty_eat_num(
                    level - 1,
                    eat_num - 2 - heights[level - 1],
                    heights,
                    patty_num,
                )
        }
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
