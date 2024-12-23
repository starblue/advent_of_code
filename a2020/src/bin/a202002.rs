use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Record {
    c: char,
    n1: i64,
    n2: i64,
    password: String,
}
impl Record {
    fn is_valid1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();
        let count = i64::try_from(count).unwrap();
        let min = self.n1;
        let max = self.n2;
        min <= count && count <= max
    }
    fn is_valid2(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        let index1 = usize::try_from(self.n1).unwrap();
        let index2 = usize::try_from(self.n2).unwrap();
        let found_at_1 = chars[index1] == self.c;
        let found_at_2 = chars[index2] == self.c;
        found_at_1 != found_at_2
    }
}
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} {}: {}", self.n1, self.n2, self.c, self.password)
    }
}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn password(i: &str) -> IResult<&str, String> {
    map(recognize(not_line_ending), String::from)(i)
}

fn record(i: &str) -> IResult<&str, Record> {
    let (i, n1) = int64(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, n2) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, c) = anychar(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, password) = password(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Record {
            n1,
            n2,
            c,
            password,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Record>> {
    many1(record)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let records = result.unwrap().1;
    //println!("{:?}", records);

    let result_a = records.iter().filter(|r| r.is_valid1()).count();
    let result_b = records.iter().filter(|r| r.is_valid2()).count();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
