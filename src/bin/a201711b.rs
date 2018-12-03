use std::cmp::{max, min};
use std::io;
use std::iter;
use std::ops;

/// The Eisenstein number a+b\omega
#[derive(Clone, Copy, Debug)]
struct EisNum {
    a: isize,
    b: isize,
}

impl ops::Add for EisNum {
    type Output = EisNum;

    fn add(self, other: EisNum) -> EisNum {
        EisNum {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl ops::AddAssign for EisNum {
    fn add_assign(&mut self, rhs: EisNum) {
        self.a += rhs.a;
        self.b += rhs.b;
    }
}

impl iter::Sum for EisNum {
    fn sum<I>(iter: I) -> EisNum
    where
        I: Iterator<Item = EisNum>,
    {
        iter.fold(EisNum { a: 0, b: 0 }, |x, y: EisNum| x + y)
    }
}

/// The shortest distance to zero on a triagonal lattice
impl EisNum {
    fn hex_norm(&self) -> isize {
        let a = self.a;
        let b = self.b;
        if a >= 0 && b >= 0 {
            a + b - min(a, b)
        } else if a < 0 && b < 0 {
            -a - b + max(a, b)
        } else {
            a.abs() + b.abs()
        }
    }
}

/// Maps directions to Eisenstein numbers
/// The x-axis (a-axis) points north here.
fn to_eis_num(dir: &str) -> EisNum {
    if dir == "n" {
        EisNum { a: 1, b: 0 }
    } else if dir == "nw" {
        EisNum { a: 1, b: 1 }
    } else if dir == "sw" {
        EisNum { a: 0, b: 1 }
    } else if dir == "s" {
        EisNum { a: -1, b: 0 }
    } else if dir == "se" {
        EisNum { a: -1, b: -1 }
    } else if dir == "ne" {
        EisNum { a: 0, b: -1 }
    } else {
        panic!("unknown direction");
    }
}

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("I/O error");
    let dirs = line.trim().split(',').map(|d| to_eis_num(d));
    let mut sum = EisNum { a: 0, b: 0 };
    let mut max_steps = 0;
    for d in dirs {
        sum += d;
        let steps = sum.hex_norm();
        if steps >= max_steps {
            max_steps = steps;
        }
    }
    println!("{}", max_steps);
}
