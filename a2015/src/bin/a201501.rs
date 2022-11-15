use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

fn action(i: &str) -> IResult<&str, i64> {
    let p0 = value(1, char('('));
    let p1 = value(-1, char(')'));
    alt((p0, p1))(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    many1(action)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;

    let result_a = input.iter().sum::<i64>();

    let mut floor = 0;
    let mut position = None;
    for (i, delta) in input.iter().enumerate() {
        floor += delta;
        if floor == -1 {
            position = Some(i + 1);
            break;
        }
    }
    let result_b = position.unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
