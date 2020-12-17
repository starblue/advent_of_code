use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let digits = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let len = digits.len();
    let half_len = len / 2;

    let mut sum = 0;
    for i in 0..len {
        if digits[i] == digits[(i + half_len) % len] {
            sum += digits[i];
        }
    }

    println!("{}", sum);
}
