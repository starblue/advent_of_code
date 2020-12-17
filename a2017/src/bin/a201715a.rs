fn main() {
    let m: u64 = 2147483647;
    let mut a = 516;
    let mut b = 190;

    let mut count = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % m;
        b = (b * 48271) % m;
        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }
    println!("{}", count);
}
