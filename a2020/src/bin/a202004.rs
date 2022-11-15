use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many_m_n;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Unit {
    Cm,
    Inch,
}
#[derive(Clone, Copy, Debug)]
struct Height {
    value: i64,
    unit: Unit,
}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn year(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(many_m_n(4, 4, one_of("0123456789"))),
        FromStr::from_str,
    )(i)
}

fn unit(i: &str) -> IResult<&str, Unit> {
    alt((value(Unit::Cm, tag("cm")), value(Unit::Inch, tag("in"))))(i)
}

fn height(i: &str) -> IResult<&str, Height> {
    let (i, value) = int64(i)?;
    let (i, unit) = unit(i)?;
    Ok((i, Height { value, unit }))
}

fn hcl(i: &str) -> IResult<&str, String> {
    map_res(
        recognize(preceded(
            tag("#"),
            many_m_n(6, 6, one_of("0123456789abcdef")),
        )),
        FromStr::from_str,
    )(i)
}

fn ecl(i: &str) -> IResult<&str, String> {
    map_res(
        recognize(alt((
            tag("amb"),
            tag("blu"),
            tag("brn"),
            tag("gry"),
            tag("grn"),
            tag("hzl"),
            tag("oth"),
        ))),
        FromStr::from_str,
    )(i)
}

fn pid(i: &str) -> IResult<&str, String> {
    map_res(
        recognize(many_m_n(9, 9, one_of("0123456789"))),
        FromStr::from_str,
    )(i)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}
impl Field {
    fn is_valid_data(&self, data: &str) -> bool {
        match self {
            Field::Byr => {
                let result = year(data);
                result.map_or(false, |(r, y)| r.is_empty() && 1920 <= y && y <= 2002)
            }
            Field::Iyr => {
                let result = year(data);
                result.map_or(false, |(r, y)| r.is_empty() && 2010 <= y && y <= 2020)
            }
            Field::Eyr => {
                let result = year(data);
                result.map_or(false, |(r, y)| r.is_empty() && 2020 <= y && y <= 2030)
            }
            Field::Hgt => {
                let result = height(data);
                result.map_or(false, |(r, h)| {
                    r.is_empty()
                        && match h.unit {
                            Unit::Cm => 150 <= h.value && h.value <= 193,
                            Unit::Inch => 59 <= h.value && h.value <= 76,
                        }
                })
            }
            Field::Hcl => {
                let result = hcl(data);
                result.map_or(false, |(r, _)| r.is_empty())
            }
            Field::Ecl => {
                let result = ecl(data);
                result.map_or(false, |(r, _)| r.is_empty())
            }
            Field::Pid => {
                let result = pid(data);
                result.map_or(false, |(r, _)| r.is_empty())
            }
            Field::Cid => true,
        }
    }
}

#[derive(Clone, Debug)]
struct Passport(HashMap<Field, String>);
impl Passport {
    fn is_valid_a(&self) -> bool {
        vec![
            Field::Byr,
            Field::Iyr,
            Field::Eyr,
            Field::Hgt,
            Field::Hcl,
            Field::Ecl,
            Field::Pid,
        ]
        .iter()
        .all(|f| self.0.contains_key(f))
    }
    fn is_valid_b(&self) -> bool {
        vec![
            Field::Byr,
            Field::Iyr,
            Field::Eyr,
            Field::Hgt,
            Field::Hcl,
            Field::Ecl,
            Field::Pid,
        ]
        .iter()
        .all(|f| self.0.contains_key(f) && f.is_valid_data(&self.0[f]))
    }
}

fn field(i: &str) -> IResult<&str, Field> {
    alt((
        value(Field::Byr, tag("byr")),
        value(Field::Iyr, tag("iyr")),
        value(Field::Eyr, tag("eyr")),
        value(Field::Hgt, tag("hgt")),
        value(Field::Hcl, tag("hcl")),
        value(Field::Ecl, tag("ecl")),
        value(Field::Pid, tag("pid")),
        value(Field::Cid, tag("cid")),
    ))(i)
}

fn val(i: &str) -> IResult<&str, String> {
    map(recognize(many0(none_of(" \n"))), String::from)(i)
}

fn pair(i: &str) -> IResult<&str, (Field, String)> {
    let (i, field) = field(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, value) = val(i)?;
    Ok((i, (field, value)))
}

fn pair_sep(i: &str) -> IResult<&str, ()> {
    let (i, _) = alt((tag(" "), line_ending))(i)?;
    Ok((i, ()))
}

fn passport(i: &str) -> IResult<&str, Passport> {
    let (i, pairs) = separated_list1(pair_sep, pair)(i)?;
    Ok((i, Passport(pairs.into_iter().collect())))
}

fn passport_sep(i: &str) -> IResult<&str, ()> {
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, ()))
}

fn input(i: &str) -> IResult<&str, Vec<Passport>> {
    separated_list1(passport_sep, passport)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let passports = result.unwrap().1;

    let result_a = passports.iter().filter(|p| p.is_valid_a()).count();
    let result_b = passports.iter().filter(|p| p.is_valid_b()).count();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::field;
    use crate::pair;
    use crate::Field;

    #[test]
    fn test_field_byr() {
        assert_eq!(Field::Byr, field("byr").unwrap().1);
    }

    #[test]
    fn test_field_hcl() {
        assert_eq!(Field::Hcl, field("hcl").unwrap().1);
    }

    #[test]
    fn test_pair_0() {
        assert_eq!(
            (Field::Hcl, "5d90f0".to_string()),
            pair("hcl:5d90f0 ").unwrap().1
        );
    }
}
