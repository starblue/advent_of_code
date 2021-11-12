use std::io;
use std::io::Read;

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    let mut depth = 0;
    let mut in_garbage = false;
    let mut escaped = false;

    let mut score = 0;
    let mut garbage_count = 0;

    for c in input_data.chars() {
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
            score += depth;
            depth -= 1;
        } else if c == '<' {
            in_garbage = true;
        } else if c == ',' {
            // commas separate groups, but we don't check that
        } else if c == '\n' {
            // ignore newline at end
        } else {
            panic!("ill-formed input");
        }
    }
    let result_a = score;
    let result_b = garbage_count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
