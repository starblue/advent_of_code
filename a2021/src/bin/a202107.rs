use core::str::FromStr;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), int)(i)
}

fn s(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // let mut sep = "";
    // for n in &input {
    //     print!("{}{}", sep, n);
    //     sep = ",";
    // }
    // println!();

    let mut positions = input;
    positions.sort();

    let len = positions.len();
    let mid = len / 2;
    let median = positions[mid];

    let result_a = positions.iter().map(|p| (p - median).abs()).sum::<i64>();

    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();
    let result_b = (min..=max)
        .map(|p0| positions.iter().map(|&p| s((p - p0).abs())).sum::<i64>())
        .min()
        .unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
