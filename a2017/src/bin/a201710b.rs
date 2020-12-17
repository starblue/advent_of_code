use std::io;

const SIZE: usize = 256;
const ITERATIONS: usize = 64;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("I/O error");
    let mut lens = line
        .chars()
        .take_while(|c| *c != '\n')
        .map(|c| c as usize)
        .collect::<Vec<usize>>();
    lens.append(&mut vec![17, 31, 73, 47, 23]);
    println!("{:?}", lens);

    let mut ns = (0..SIZE).collect::<Vec<_>>();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..ITERATIONS {
        for len in &lens {
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
    }
    let xs = ns
        .chunks(16)
        .map(|c| c.iter().fold(0, |a, b| a ^ b))
        .collect::<Vec<_>>();
    let mut result = String::new();
    for x in xs {
        result.push_str(&format!("{:02x}", x));
    }
    println!("{}", result);
}
