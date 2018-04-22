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

fn main() {
    let n = parse_word::<usize>();
    let circum = parse_line::<i64>();

    let (pos_cw, cal_cw) = {
        let mut pos = Vec::with_capacity(n);
        let mut cal = Vec::with_capacity(n);

        for _ in 0..n {
            let x = parse_word::<i64>();
            let v = parse_word::<i64>();

            pos.push(x);
            cal.push(v);
        }

        (pos, cal)
    };

    let pos_ccw = pos_cw.iter().rev().map(|p| circum - p).collect::<Vec<_>>();
    let cal_ccw = cal_cw.iter().rev(); // .collect::<Vec<_>>();

    // cal_cw[0] - pos[0]
    // cal_cw[0] + cal_cw[1] - pos[1]
    // ...
    // cal_cw[0] + cal_cw[1] + ... + cal_cw[n - 1] - pos[n - 1]
    let cal_cw_net = cal_cw
        .iter()
        .zip(pos_cw.iter())
        .scan(0, |accum, (&c, pos)| {
            *accum += c;
            Some(*accum - pos)
        })
        .collect::<Vec<_>>();
    debug!("cal_cw_net: {:?}", cal_cw_net);

    let cal_ccw_net = cal_ccw
        // .iter()
        .zip(pos_ccw.iter())
        .scan(0, |accum, (&c, pos)| {
            *accum += c;
            Some(*accum - pos)
        })
        .collect::<Vec<_>>();
    debug!("cal_ccw_net: {:?}", cal_ccw_net);

    // cal_cw[0]
    // max(cal_cw[0], cal_cw[1])
    // ...
    // max(cal_cw[0], cal_cw[1], ..., cal_cw[n - 1])
    let cal_cw_net_scan_max = cal_cw_net
        .iter()
        .scan(-circum, |accum, &c| {
            *accum = cmp::max(*accum, c);
            Some(*accum)
        })
        .collect::<Vec<_>>();
    debug!("cal_cw_net_scan_max: {:?}", cal_cw_net_scan_max);

    let cal_ccw_net_scan_max = cal_ccw_net
        .iter()
        .scan(-circum, |accum, &c| {
            *accum = cmp::max(*accum, c);
            Some(*accum)
        })
        .collect::<Vec<_>>();
    debug!("cal_ccw_net_scan_max: {:?}", cal_ccw_net_scan_max);

    let cal_cw_net_max = {
        let turn_max = (0..n)
            .map(|i| {
                let cal = cal_cw_net[i];
                let turn = if i == n - 1 {
                    0
                } else {
                    let turn = cal_ccw_net_scan_max[n - 2 - i] - pos_cw[i];
                    cmp::max(0, turn)
                };
                cal + turn
            })
            .max()
            .unwrap();
        cmp::max(turn_max, cal_ccw_net_scan_max[n - 1])
    };
    debug!("cal_cw_net_max: {:?}", cal_cw_net_max);

    let cal_ccw_net_max = {
        let turn_max = (0..n)
            .map(|i| {
                let cal = cal_ccw_net[i];
                let turn = if i == n - 1 {
                    0
                } else {
                    let turn = cal_cw_net_scan_max[n - 2 - i] - pos_ccw[i];
                    cmp::max(0, turn)
                };
                cal + turn
            })
            .max()
            .unwrap();
        cmp::max(turn_max, cal_cw_net_scan_max[n - 1])
    };
    debug!("cal_ccw_net_max: {:?}", cal_ccw_net_max);

    let cal_net_max = vmax!(0, cal_cw_net_max, cal_ccw_net_max);
    println!("{}", cal_net_max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
