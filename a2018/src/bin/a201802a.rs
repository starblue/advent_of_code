use std::collections::HashMap;
use std::io;

fn main() {
    let mut line = String::new();

    let mut count2 = 0;
    let mut count3 = 0;
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.is_empty() {
            break;
        }

        let mut counts = HashMap::new();
        for ch in line.chars() {
            let counter = counts.entry(ch).or_insert(0);
            *counter += 1;
        }

        for v in counts.values() {
            if *v == 2 {
                count2 += 1;
                break;
            }
        }
        for v in counts.values() {
            if *v == 3 {
                count3 += 1;
                break;
            }
        }
    }
    println!("{}", count2 * count3);
}
