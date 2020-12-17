use std::io;

fn main() {
    let mut line = String::new();

    let mut vs = Vec::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let v: isize = match line.trim().parse() {
            Result::Ok(mass) => mass,
            Result::Err(_) => break,
        };
        vs.push(v);
    }

    let mut result_a = 0;
    for i in 0..vs.len() {
        for j in 0..i {
            let vi = vs[i];
            let vj = vs[j];
            if vi + vj == 2020 {
                result_a = vi * vj;
            }
        }
    }

    let mut result_b = 0;
    for i in 0..vs.len() {
        for j in 0..i {
            for k in 0..j {
                let vi = vs[i];
                let vj = vs[j];
                let vk = vs[k];
                if vi + vj + vk == 2020 {
                    result_b = vi * vj * vk;
                }
            }
        }
    }

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
