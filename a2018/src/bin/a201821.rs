use std::collections::HashSet;

fn compute_first_r5() -> i64 {
    compute_r5(true)
}
fn compute_last_r5() -> i64 {
    compute_r5(false)
}
fn compute_r5(first: bool) -> i64 {
    // reverse engineering of the input file
    let mut stop_values = HashSet::new();
    let mut last_stop_value = std::i64::MAX;
    let mut r2;
    let mut r4;
    let mut r5;
    loop {
        r5 = 123;
        r5 &= 456;
        if r5 == 72 {
            break;
        }
    }
    r5 = 0;
    loop {
        r4 = r5 | 65536;
        r5 = 8858047;
        loop {
            r2 = r4 & 255;
            r5 += r2;
            r5 &= 16777215; // 2^24-1 = 0xffffff
            r5 *= 65899;
            r5 &= 16777215; // 2^24-1 = 0xffffff

            if 256 > r4 {
                break;
            }
            r2 = 0;
            while (r2 + 1) * 256 <= r4 {
                r2 += 1;
            }
            r4 = r2
        }
        // Here the program stops if r5 == r0.
        // We want the first value of r5 for part 1,
        // and the last before values start repeating for part 2.
        if first {
            return r5;
        } else {
            if stop_values.contains(&r5) {
                // value in r5 was seen before, return previous
                return last_stop_value;
            }
            last_stop_value = r5;
            stop_values.insert(r5);
        }
    }
}

fn main() {
    println!("1: {}", compute_first_r5());
    println!("2: {}", compute_last_r5());
}
