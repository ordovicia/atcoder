#![allow(unused)]

#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, str};
use cmp::Ordering::*;
use collections::{HashMap, HashSet};
use io::{Read, Write};

#[cfg(feature = "debug")]
macro_rules! debug {
    ($fmt: expr) => { eprintln!($fmt) };
    ($fmt: expr, $($arg: tt)*) => { eprintln!($fmt, $($arg)*) };
}

#[cfg(not(feature = "debug"))]
macro_rules! debug {
    ($fmt: expr) => {};
    ($fmt: expr, $($arg: tt)*) => {};
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
    let s = read_line();
    let s = s.as_bytes();

    if solve(s) {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn solve(s: &[u8]) -> bool {
    let len = s.len();
    let mut idx = 0;

    loop {
        if idx == len {
            return true;
        } else if idx + 5 > len {
            return false;
        }

        if &s[idx..idx + 5] == b"dream" {
            if idx + 5 == len {
                return true;
            } else if idx + 7 > len {
                return false;
            }

            if &s[idx + 5..idx + 7] == b"er" {
                if idx + 7 == len {
                    return true;
                }

                if s[idx + 7] == b'a' {
                    // dream era..
                    idx += 5;
                } else {
                    // dreamer
                    idx += 7;
                }
            } else {
                idx += 5;
            }
        } else if &s[idx..idx + 5] == b"erase" {
            if idx + 5 == len {
                return true;
            }

            if s[idx + 5] == b'r' {
                // eraser
                idx += 6;
            } else {
                // erase
                idx += 5;
            }
        } else {
            return false;
        }
    }
}
