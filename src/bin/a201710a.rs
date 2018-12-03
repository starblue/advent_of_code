use std::io;

const SIZE: usize = 256;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("I/O error");
    let lens = line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let mut ns = (0..SIZE).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip = 0;
    for len in lens {
        let mut p1 = pos;
        let mut p2 = (pos + len - 1) % SIZE;
        let mut rest = len / 2;
        while rest > 0 {
            ns.swap(p1, p2);

            p1 = (p1 + 1) % SIZE;
            p2 = (p2 + SIZE - 1) % SIZE;
            rest -= 1;
        }
        pos = (pos + len + skip) % SIZE;
        skip = (skip + 1) % SIZE;
    }
    println!("{}", ns[0] * ns[1]);
}
