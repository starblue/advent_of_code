use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn line(i: &str) -> IResult<&str, (i64, i64, i64)> {
    let (i, _) = space0(i)?;
    let (i, a) = uint(i)?;
    let (i, _) = space1(i)?;
    let (i, b) = uint(i)?;
    let (i, _) = space1(i)?;
    let (i, c) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, (a, b, c)))
}

fn input(i: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    many1(line)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for (a, b, c) in &input {
    //     println!("{:5}{:5}{:5}", a, b, c);
    // }
    // println!();

    let mut count = 0;
    for &(a, b, c) in &input {
        let mut v = vec![a, b, c];
        v.sort();
        if v[0] + v[1] > v[2] {
            count += 1;
        }
    }
    let result_a = count;

    let mut count = 0;
    for chunk in input.chunks(3) {
        if let &[(a0, b0, c0), (a1, b1, c1), (a2, b2, c2)] = chunk {
            for mut v in vec![vec![a0, a1, a2], vec![b0, b1, b2], vec![c0, c1, c2]] {
                v.sort();
                if v[0] + v[1] > v[2] {
                    count += 1;
                }
            }
        }
    }
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
