#![allow(unused)]

use std::{cmp, collections, fmt, io, iter, ops, str};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use io::{Read, Write};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt: expr) => { eprintln!($fmt) };
    ($fmt: expr, $($args: tt)*) => { eprintln!($fmt, $($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt: expr) => {};
    ($fmt: expr, $($args: tt)*) => {};
}

macro_rules! vmax {
    ($x: expr, $y: expr) => { cmp::max($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::max($x, vmax!($($args),*)) };
}

macro_rules! vmin {
    ($x: expr, $y: expr) => { cmp::min($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::min($x, vmin!($($args),*)) };
}

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

fn read_word() -> String {
    const CAPACITY: usize = 16;

    let mut stdin = io::stdin();
    let mut byte: [u8; 1] = [0];

    loop {
        let mut buf = Vec::with_capacity(CAPACITY);

        loop {
            let res = stdin.read(&mut byte);

            if res.unwrap_or(0) == 0 || byte[0] <= b' ' {
                break;
            } else {
                buf.push(byte[0]);
            }
        }

        if buf.len() > 0 {
            return String::from_utf8(buf).unwrap();
        }
    }
}

fn parse_line<T: str::FromStr>() -> T {
    read_line().parse().ok().unwrap()
}

fn parse_word<T: str::FromStr>() -> T {
    read_word().parse().ok().unwrap()
}

fn parse_vec<T: str::FromStr>() -> Vec<T> {
    read_line()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn flush() {
    io::stdout().flush();
}

const MAX_SIZE: usize = 50;

struct Cells {
    cells: [[bool; MAX_SIZE]; MAX_SIZE], // true: black, false: white
    height: usize,
    width: usize,
}

fn main() {
    let height = parse_word::<usize>();
    let width = parse_word::<usize>();

    let mut cells = Cells {
        cells: [[false; MAX_SIZE]; MAX_SIZE],
        height: height,
        width: width,
    };

    for i in 0..cells.height {
        let s = read_line();
        assert!(s.len() == cells.width);

        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                cells.cells[i][j] = true;
            }
        }
    }

    for i in 0..cells.height {
        for j in 0..cells.width {
            if cells.cells[i][j] // black
                && !is_contiguous_black(&cells, (i, j))
            {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn is_contiguous_black(cells: &Cells, (i, j): (usize, usize)) -> bool {
    let c = cells.cells;

    let mut is_contig = if i == 0 { false } else { c[i - 1][j] };
    is_contig |= if i == cells.height { false } else { c[i + 1][j] };
    is_contig |= if j == 0 { false } else { c[i][j - 1] };
    is_contig |= if j == cells.width { false } else { c[i][j + 1] };

    is_contig
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
