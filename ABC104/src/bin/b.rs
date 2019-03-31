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
    input!(s: String);

    let mut ok = true;
    let mut c_found = false;

    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            if c != 'A' {
                ok = false;
                break;
            }
        } else {
            if 2 <= i && i <= s.len() - 2 && c == 'C' {
                if c_found {
                    ok = false;
                    break;
                } else {
                    c_found = true;
                }
            } else if c.is_uppercase() {
                ok = false;
                break;
            }
        }
    }

    if ok && c_found {
        println!("AC");
    } else {
        println!("WA");
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
