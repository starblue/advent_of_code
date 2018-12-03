use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut digits = input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let first = digits[0];
    digits.push(first);
    let mut sum = 0;
    for w in digits.windows(2) {
        if w[0] == w[1] {
            sum += w[0];
        }
    }

    println!("{}", sum);
}
