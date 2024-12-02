use core::str::FromStr;

use std::io;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn line(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, uint)(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, line)(i)
}

fn safe_increasing(s: &[i64]) -> bool {
    for i in 0..(s.len() - 1) {
        let delta = s[i + 1] - s[i];
        if !(1 <= delta && delta <= 3) {
            return false;
        }
    }
    true
}

fn safe_decreasing(s: &[i64]) -> bool {
    for i in 0..(s.len() - 1) {
        let delta = s[i] - s[i + 1];
        if !(1 <= delta && delta <= 3) {
            return false;
        }
    }
    true
}

fn safe(s: &[i64]) -> bool {
    safe_increasing(s) || safe_decreasing(s)
}

fn dampened_safe(s: &[i64]) -> bool {
    if safe(s) {
        true
    } else {
        for i in 0..s.len() {
            let mut s1 = s.to_vec();
            s1.remove(i);
            if safe(&s1) {
                return true;
            }
        }
        false
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for line in &input {
    //     let mut sep = "";
    //     for n in line {
    //         print!("{}{}", sep, n);
    //         sep = " ";
    //     }
    //     println!();
    // }

    let result1 = input.iter().filter(|&s| safe(s)).count();

    let result2 = input.iter().filter(|&s| dampened_safe(s)).count();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
