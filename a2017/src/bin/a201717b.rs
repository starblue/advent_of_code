use std::io;

use std::collections::VecDeque;

const N: usize = 50_000_000;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let step: usize = line.trim().parse().unwrap();
    //println!("step = {}", step);

    let mut vs = VecDeque::new();
    vs.push_back(0);
    for i in 1..(N + 1) {
        vs.rotate_left(step % vs.len());
        vs.push_back(i);

        if i % 10_000_000 == 0 {
            println!("{}", i);
        }
    }
    let mut iter = vs.iter();
    iter.find(|n| **n == 0);
    let x = iter.next().unwrap_or(vs.front().unwrap());
    println!("{}", x);
}
