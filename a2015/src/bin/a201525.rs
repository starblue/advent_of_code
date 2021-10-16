use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Input {
    row: u32,
    column: u32,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "To continue, please consult the code grid in the manual.  Enter the code at row {}, column {}.", self.row, self.column)
    }
}

fn uint(i: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, _) =
        tag("To continue, please consult the code grid in the manual.  Enter the code at row ")(i)?;
    let (i, row) = uint(i)?;
    let (i, _) = tag(", column ")(i)?;
    let (i, column) = uint(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Input { row, column }))
}

fn diagonal_map(row: u32, column: u32) -> u32 {
    let n = row + column - 2;
    let sn = n * (n + 1) / 2;
    sn + column
}

pub fn mod_mul(m: u32, a: u32, b: u32) -> u32 {
    let a = u64::from(a);
    let b = u64::from(b);
    let m = u64::from(m);
    ((a * b) % m) as u32
}

pub fn mod_square(m: u32, n: u32) -> u32 {
    mod_mul(m, n, n)
}

pub fn mod_pow(m: u32, b: u32, e: u32) -> u32 {
    // invariant: result * b^e
    let mut b = b;
    let mut e = e;
    let mut result = 1;
    while e > 0 {
        if e % 2 == 0 {
            b = mod_square(m, b);
            e /= 2;
        } else {
            result = mod_mul(m, result, b);
            e -= 1;
        }
    }
    result
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
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let start = 20151125;
    let m = 33554393;
    let base = 252533;
    let exponent = diagonal_map(input.row, input.column) - 1;

    let result_a = mod_mul(m, start, mod_pow(m, base, exponent));

    println!("a: {}", result_a);
}
