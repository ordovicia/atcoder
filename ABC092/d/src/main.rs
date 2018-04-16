#![allow(unused)]

#[allow(unused_imports)]
use std::{cmp, collections, fmt, io, iter, ops, str};
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
    let white_num = parse_word::<usize>();
    let black_num = parse_word::<usize>();

    const HEIGHT: usize = 100;
    const WIDTH: usize = 100;

    println!("{} {}", HEIGHT, WIDTH);

    let mut white_cnt = 1;
    let mut black_cnt = 1;

    for h in 0..(HEIGHT / 2) {
        for w in 0..WIDTH {
            if white_cnt < white_num && w % 2 == 0 && h % 2 == 0 {
                print!(".");
                white_cnt += 1;
            } else {
                print!("#");
            }
        }
        println!();
    }

    for h in 0..(HEIGHT / 2) {
        for w in 0..WIDTH {
            if black_cnt < black_num && w % 2 == 0 && h % 2 == 1 {
                print!("#");
                black_cnt += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
