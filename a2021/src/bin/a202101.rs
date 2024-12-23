use core::str::FromStr;

use std::io;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn line(i: &str) -> IResult<&str, i64> {
    let (i, line) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    many1(line)(i)
}
fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for n in &input {
    //     println!("{}", n);
    // }

    let result_a = input
        .windows(2)
        .filter(|w| matches!(w, &[a, b] if a < b))
        .count();

    let sums = input
        .windows(3)
        .filter_map(|w| match w {
            &[a, b, c] => Some(a + b + c),
            _ => None,
        })
        .collect::<Vec<_>>();
    let result_b = sums
        .windows(2)
        .filter(|w| matches!(w, &[a, b] if a < b))
        .count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
