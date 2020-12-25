use std::io;

use modulo::mod_mul;
use modulo::mod_pow;

fn loop_size(m: u32, sn: u32, pk: u32) -> u64 {
    let mut key = 1;
    let mut result = 0;
    while key != pk {
        key = mod_mul(m, sn, key);
        result += 1;
    }
    result
}

fn main() {
    let mut line = String::new();

    line.clear();
    io::stdin().read_line(&mut line).expect("I/O error");
    let cpk = line.trim().parse().expect("parse error");

    line.clear();
    io::stdin().read_line(&mut line).expect("I/O error");
    let dpk = line.trim().parse().expect("parse error");

    let m = 20201227;
    let sn = 7;

    let cls = loop_size(m, sn, cpk);
    let csk = mod_pow(m, dpk, cls);

    let result_a = csk;

    println!("a: {}", result_a);
}
