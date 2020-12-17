use core::iter::once;

use std::io;

fn is_sum(n: usize, ns: &[usize]) -> bool {
    for i in 0..ns.len() {
        for j in (i + 1)..ns.len() {
            if ns[i] + ns[j] == n {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut line = String::new();

    let mut ns = Vec::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let v: usize = match line.trim().parse() {
            Result::Ok(mass) => mass,
            Result::Err(_) => break,
        };
        ns.push(v);
    }

    let mut result_a = 0;
    for i in 25..ns.len() {
        if !is_sum(ns[i], &ns[(i - 25)..i]) {
            result_a = ns[i];
        }
    }

    let mut result_b = 0;
    let partial_sums = once(0)
        .chain(ns.iter().scan(0, |s, n| {
            *s += n;
            Some(*s)
        }))
        .collect::<Vec<_>>();
    for i in 0..ns.len() {
        for j in (i + 1)..ns.len() {
            let sum = partial_sums[j + 1] - partial_sums[i];
            if sum == result_a {
                let min = ns[i..=j].iter().min().unwrap();
                let max = ns[i..=j].iter().max().unwrap();
                result_b = min + max;
            }
        }
    }

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
