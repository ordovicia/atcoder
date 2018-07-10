#![allow(unused)]

use std::{cell, cmp, collections, convert, fmt, io, iter, ops, str};

use cell::{Cell, RefCell};
use cmp::Ordering::{Equal, Greater, Less};
use collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use io::{Read, Write};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($($args: tt)*) => { eprint!($($args)*) };
}

#[cfg(feature = "debug")]
macro_rules! debugln {
    ($($args: tt)*) => { eprintln!($($args)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($($args:tt)*) => {};
}

#[cfg(not(feature = "debug"))]
macro_rules! debugln {
    ($($args:tt)*) => {};
}

macro_rules! vmax {
    ($x: expr, $y: expr) => { cmp::max($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::max($x, vmax!($($args),*)) };
}

macro_rules! vmin {
    ($x: expr, $y: expr) => { cmp::min($x, $y) };
    ($x: expr, $($args: expr),*) => { cmp::min($x, vmin!($($args),*)) };
}

fn main() {
    let grid_size = parse_word::<usize>();
    let color_num = parse_word::<usize>();

    const PARTS_MOD: usize = 3;

    let wrongness = {
        let mut wrongness = vec![vec![0; color_num]; color_num];
        for i in wrongness.iter_mut() {
            for j in i.iter_mut() {
                *j = parse_word();
            }
        }
        wrongness
    };

    let colors = {
        let mut colors = vec![vec![0; grid_size]; grid_size];
        for i in colors.iter_mut() {
            for j in i.iter_mut() {
                *j = parse_word();
            }
        }
        colors
    };

    // wrongness_parts[(i + j) % 3][color]:
    //  Sum of wrongness for redrawing the (i + j) % 3 part to the color.
    let mut wrongness_parts: [Vec<i32>; PARTS_MOD] =
        [vec![0; color_num], vec![0; color_num], vec![0; color_num]];
    for i in 0..grid_size {
        for j in 0..grid_size {
            for c in 0..color_num {
                wrongness_parts[(i + j) % PARTS_MOD][c] +=
                    wrongness[(colors[i][j] - 1) as usize][c];
            }
        }
    }

    for (i, w) in wrongness_parts.iter().enumerate() {
        debugln!("Redraw parts {}: {:?}", i, w);
    }

    // Minimize sum of wrongness with all color combinations.
    let mut min_wrongness = std::i32::MAX;

    for c0 in 0..color_num {
        for c1 in 0..color_num {
            if c1 == c0 {
                continue;
            }

            for c2 in 0..color_num {
                if c2 == c0 || c2 == c1 {
                    continue;
                }

                let w = wrongness_parts[0][c0] + wrongness_parts[1][c1] + wrongness_parts[2][c2];
                min_wrongness = cmp::min(min_wrongness, w);
                debugln!("Redraw to color [{}][{}][{}]: {}", c0, c1, c2, w);
            }
        }
    }

    println!("{}", min_wrongness);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
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

        if !buf.is_empty() {
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

fn next_prime(x: i32) -> i32 {
    for i in (x + 1).. {
        if is_prime(i) {
            return i;
        }
    }

    panic!("attempt to add with overflow");
}

fn is_prime(x: i32) -> bool {
    let ceil = f64::from(x).sqrt().ceil() as i32;
    x == 2 || (2..(ceil + 1)).all(|i| x % i != 0)
}

fn bit_high(bits: u64, pos: u64) -> bool {
    bits & (1 << pos) != 0
}
