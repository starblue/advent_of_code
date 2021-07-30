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

    let mut children = Vec::new();
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

fn data_size(cert: &Cert) -> i32 {
    cert.children.iter().map(|c| data_size(c)).sum::<i32>() + cert.data.iter().sum::<i32>()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");
    let data = line
        .split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let (_rest, result) = parse(&data);
    let cert = result.unwrap();
    println!("{:?}", data_size(&cert));
}
