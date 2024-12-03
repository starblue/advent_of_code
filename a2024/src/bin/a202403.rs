use core::str::FromStr;

use std::fmt;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
enum Item {
    Mul(i64, i64),
    Do(bool),
    Other(char),
}
impl Item {
    fn value(&self) -> i64 {
        match self {
            Item::Mul(n0, n1) => n0 * n1,
            _ => 0,
        }
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Mul(n0, n1) => write!(f, "mul({},{})", n0, n1),
            Item::Do(true) => write!(f, "do()"),
            Item::Do(false) => write!(f, "don't()"),
            Item::Other(c) => write!(f, "{}", c),
        }
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn item_mul(i: &str) -> IResult<&str, Item> {
    let (i, _) = tag("mul(")(i)?;
    let (i, n0) = uint(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, n1) = uint(i)?;
    let (i, _) = tag(")")(i)?;
    Ok((i, Item::Mul(n0, n1)))
}
fn item_do(i: &str) -> IResult<&str, Item> {
    let (i, _) = tag("do()")(i)?;
    Ok((i, Item::Do(true)))
}
fn item_dont(i: &str) -> IResult<&str, Item> {
    let (i, _) = tag("don't()")(i)?;
    Ok((i, Item::Do(false)))
}
fn item_other(i: &str) -> IResult<&str, Item> {
    let (i, c) = anychar(i)?;
    Ok((i, Item::Other(c)))
}
fn item(i: &str) -> IResult<&str, Item> {
    alt((item_mul, item_do, item_dont, item_other))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Item>> {
    many1(item)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for item in &input {
    //     print!("{}", item);
    // }
    // println!();

    let result1 = input.iter().map(|item| item.value()).sum::<i64>();

    let mut sum = 0;
    let mut mul_enabled = true;
    for item in &input {
        match item {
            Item::Mul(n0, n1) => {
                if mul_enabled {
                    sum += n0 * n1
                }
            }
            Item::Do(enable) => mul_enabled = *enable,
            _ => (),
        }
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
