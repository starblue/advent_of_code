use std::io;

const N: usize = 2017;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let step: usize = line.trim().parse().unwrap();

    let mut v = vec![0];
    let mut pos = 0;
    for i in 1..(N + 1) {
        pos = (pos + step + 1) % v.len();
        v.insert(pos, i);
    }
    pos = (pos + 1) % v.len();
    println!("{}", v[pos]);
}
