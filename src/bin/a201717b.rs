use std::io;

use std::collections::LinkedList;

const N: usize = 50_000_000;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let step: usize = line.trim().parse().unwrap();

    let mut vs = LinkedList::new();
    vs.push_back(0);
    for i in 1..(N + 1) {
        let s = step % vs.len();
        let mut vs_end = vs.split_off(s);
        vs_end.append(&mut vs);
        vs = vs_end;
        vs.push_back(i);

        if i % 1_000_000 == 0 {
            println!("{}", i);
        }
    }
    let mut iter = vs.iter();
    iter.find(|n| **n == 0);
    let x = iter.next().unwrap_or(vs.front().unwrap());
    println!("{}", x);
}
