use std::io;

fn main() {
    let mut line = String::new();

    let mut sum = 0;
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let change: isize = match line.trim().parse() {
            Result::Ok(change) => change,
            Result::Err(_) => break,
        };
        sum += change;
    }
    println!("{}", sum);
}
