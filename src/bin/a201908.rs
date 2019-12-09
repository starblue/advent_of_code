use core::iter::repeat;

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

    let xs = 25;
    let ys = 6;
    let zs = digits.len() / (xs * ys);

    let mut min_count0 = std::i64::MAX;
    let mut min_count0_12 = None;
    for k in 0..zs {
        let mut counts = repeat(0).take(10).collect::<Vec<_>>();
        for j in 0..ys {
            for i in 0..xs {
                let d = digits[xs * ys * k + xs * j + i];
                counts[d as usize] += 1;
            }
        }
        if counts[0] < min_count0 {
            min_count0 = counts[0];
            min_count0_12 = Some(counts[1] * counts[2]);
        }
    }

    let mut image = repeat(repeat(None).take(xs).collect::<Vec<_>>())
        .take(ys)
        .collect::<Vec<_>>();
    for k in 0..zs {
        for j in 0..ys {
            for i in 0..xs {
                let d = digits[xs * ys * k + xs * j + i];
                if d != 2 && image[j][i] == None {
                    image[j][i] = Some(d);
                }
            }
        }
    }
    for j in 0..ys {
        for i in 0..xs {
            if let Some(d) = image[j][i] {
                if d == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }

    let result_a = min_count0_12.unwrap();
    let result_b = "KYHFE";
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
