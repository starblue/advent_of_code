use std::collections::HashSet;
use std::io;

fn main() {
    let mut line = String::new();

    let mut changes = Vec::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let change: isize = match line.trim().parse() {
            Result::Ok(change) => change,
            Result::Err(_) => break,
        };
        changes.push(change);
    }
    let mut sum = 0;
    let mut sums = HashSet::new();
    'a: loop {
        for c in &changes {
            sum += c;
            if sums.contains(&sum) {
                break 'a;
            }
            sums.insert(sum);
        }
    }

    println!("{}", sum);
}
