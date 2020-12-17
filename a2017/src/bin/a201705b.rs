use std::io;

fn main() {
    let mut line = String::new();

    let mut offsets = Vec::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        match line.trim().parse::<isize>() {
            Ok(offset) => offsets.push(offset),
            Err(_) => break,
        };
    }
    let mut count = 0;

    let mut ip = 0;
    loop {
        if ip < 0 {
            break;
        }
        let uip = ip as usize;
        if uip >= offsets.len() {
            break;
        }

        ip += offsets[uip];
        if offsets[uip] >= 3 {
            offsets[uip] -= 1;
        } else {
            offsets[uip] += 1;
        }
        count += 1;
    }
    println!("{}", count);
}
