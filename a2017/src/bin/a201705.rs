use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn line(i: &str) -> IResult<&str, i64> {
    let (i, n) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, n))
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
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
    // for n in &input {
    //     println!("{}", n);
    // }

    let mut offsets = input.clone();
    let mut ip = 0;
    let mut count = 0;
    loop {
        let uip = ip as usize;
        if uip >= offsets.len() {
            break;
        }

        ip += offsets[uip];
        offsets[uip] += 1;

        count += 1;
    }
    let result_a = count;

    let mut offsets = input;
    let mut ip = 0;
    let mut count = 0;
    loop {
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
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
