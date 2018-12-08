use std::io;
use std::result::Result;

#[derive(Debug)]
struct Cert {
    children: Vec<Cert>,
    data: Vec<i32>,
}

fn parse(data: &[i32]) -> (&[i32], Result<Cert, ()>) {
    let n_children = data[0];
    let n_data = data[1] as usize;
    let mut rest = &data[2..];

    let mut children = Vec::with_capacity(n_children as usize);
    for _ in 0..n_children {
        let (new_rest, result) = parse(rest);
        match result {
            Ok(cert) => children.push(cert),
            Err(_) => {
                return (new_rest, result);
            }
        }
        rest = new_rest;
    }

    if data.len() >= n_data {
        let data = rest[0..n_data].to_owned();
        rest = &rest[n_data..];
        (rest, Ok(Cert { children, data }))
    } else {
        (rest, Err(()))
    }
}

fn value(cert: &Cert) -> i32 {
    if cert.children.is_empty() {
        cert.data.iter().sum::<i32>()
    } else {
        cert.data
            .iter()
            .map(|&i| {
                let i = (i - 1) as usize;
                if i < cert.children.len() {
                    let c = &cert.children[i];
                    value(c)
                } else {
                    0
                }
            })
            .sum::<i32>()
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");
    let data = line
        .split(" ")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let (_rest, result) = parse(&data);
    let cert = result.unwrap();
    println!("{:?}", value(&cert));
}
