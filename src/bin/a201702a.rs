use std::io;

fn main() {
    let mut line = String::new();
    let mut sum = 0;

    loop {
        line.clear();
        let num_bytes = io::stdin().read_line(&mut line).expect("I/O error");

        if num_bytes == 0 {
            break;
        }

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let min = numbers.iter().min().unwrap_or(&0);
        let max = numbers.iter().max().unwrap_or(&0);

        sum += max - min;
    }
    println!("{}", sum);
}
