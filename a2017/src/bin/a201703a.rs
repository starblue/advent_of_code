use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let a: isize = line.trim().parse().unwrap();

    let r = ((a - 1) as f64).sqrt() as isize;
    // side length of smallest square containing a
    let s = ((r + 1) / 2) * 2 + 1;
    // side length of largest square not containing a
    let s0 = s - 2;

    // maximal absolute coordinate value
    let d = (s - 1) / 2;
    // start value of border turn
    let b = s0 * s0;
    let da = if s > 1 { (a - b) % (s - 1) - d } else { 0 };

    let result = d + da.abs();
    println!("{}", result);
}
