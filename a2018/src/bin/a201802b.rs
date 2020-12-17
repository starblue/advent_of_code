use std::io;

fn main() {
    let mut ids = Vec::new();

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.is_empty() {
            break;
        }

        ids.push(line.trim_end().to_owned());
    }

    let mut result = String::new();
    for id1 in &ids {
        for id2 in &ids {
            if id1.len() == id2.len() {
                let common_part = id1
                    .chars()
                    .zip(id2.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect::<String>();
                if common_part.len() == id1.len() - 1 {
                    result = common_part;
                }
            }
        }
    }
    println!("{}", result);
}
