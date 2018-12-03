use std::io;

fn main() {
    let mut line = String::new();

    let mut offsets = Vec::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let offset: isize = match line.trim().parse() {
            Result::Ok(offset) => offset,
            Result::Err(_) => break,
        };
        offsets.push(offset);
    }
    let mut count = 0;

    let mut ip = 0;
    loop {
        let uip = ip as usize;
        if uip >= offsets.len() {
            break;
        }

        ip += offsets[uip];
        offsets[uip] += 1;

        count += 1;
    }
    println!("{}", count);
}
