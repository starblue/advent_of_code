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

        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                let a = numbers[i];
                let b = numbers[j];
                if i != j && a % b == 0 {
                    sum += a / b;
                }
            }
        }
    }
    println!("{}", sum);
}
