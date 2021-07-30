use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let mut data_left = line.trim().chars().collect::<Vec<_>>();
    let mut data_right = Vec::new();
    while !data_left.is_empty() {
        let c1 = data_left.pop().unwrap();
        if data_right.is_empty() {
            data_right.push(c1);
        } else {
            let c2 = data_right.pop().unwrap();
            if c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
                && c1.is_lowercase() != c2.is_lowercase()
            {
                // reacted and gone
            } else {
                data_right.push(c2);
                data_right.push(c1);
            }
        }
    }

    println!("{}", data_right.len());
}
