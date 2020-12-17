use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("I/O error");

    let mut depth = 0;
    let mut in_garbage = false;
    let mut escaped = false;
    let mut garbage_count = 0;

    for c in line.chars().take_while(|c| *c != '\n') {
        if escaped {
            // ignore escaped character
            escaped = false;
        } else if in_garbage {
            if c == '>' {
                in_garbage = false;
            } else if c == '!' {
                escaped = true;
            } else {
                // count garbage character
                garbage_count += 1;
            }
        } else if c == '{' {
            depth += 1;
        } else if c == '}' && depth > 0 {
            depth -= 1;
        } else if c == '<' {
            in_garbage = true;
        } else if c == '!' {
            escaped = true;
        } else if c == ',' {
            // commas separate groups, but we don't check that
        } else {
            panic!("ill-formed input");
        }
    }

    println!("{}", garbage_count);
}
