use std::io;
use std::iter::once;
use std::iter::Iterator;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let digits = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let len = digits.len();

    let first = digits[0];
    let last = digits[len - 1];
    let result_a = digits
        .windows(2)
        .chain(once(&[last, first][..]))
        .filter_map(|w| {
            if let &[d0, d1] = w {
                if d0 == d1 {
                    Some(d0)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<u32>();

    let half_len = len / 2;
    let result_b = (0..len)
        .filter_map(|i| {
            let j = (i + half_len) % len;
            if digits[i] == digits[j] {
                Some(digits[i])
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
