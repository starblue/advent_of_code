use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn row(i: &str) -> IResult<&str, Vec<i64>> {
    let (i, row) = separated_list1(tag("\t"), uint)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, row))
}

fn input(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
    many1(row)(i)
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
    // for row in &input {
    //     let mut sep = "";
    //     for n in row {
    //         print!("{}{}", sep, n);
    //         sep = "\t";
    //     }
    //     println!();
    // }

    let mut sum = 0;
    for row in &input {
        let min = row.iter().min().unwrap_or(&0);
        let max = row.iter().max().unwrap_or(&0);

        sum += max - min;
    }
    let result_a = sum;

    let mut sum = 0;
    for row in &input {
        for i in 0..row.len() {
            for j in 0..row.len() {
                let a = row[i];
                let b = row[j];
                if i != j && a % b == 0 {
                    sum += a / b;
                }
            }
        }
    }
    let result_b = sum;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
