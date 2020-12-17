use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::alt;
use nom::digit;
use nom::do_parse;
use nom::line_ending;
use nom::many0;
use nom::many1;
use nom::many_m_n;
use nom::map;
use nom::map_res;
use nom::named;
use nom::none_of;
use nom::one_of;
use nom::preceded;
use nom::recognize;
use nom::tag;
use nom::value;

#[derive(Clone, Copy, Debug)]
enum Unit {
    CM,
    INCH,
}
#[derive(Clone, Copy, Debug)]
struct Height {
    value: i64,
    unit: Unit,
}

named!(int64<&str, i64>,
    map_res!(digit, FromStr::from_str)
);

named!(year<&str, i64>,
    map_res!(
        recognize!(
            many_m_n!(4, 4, one_of!("0123456789"))),
        FromStr::from_str
    )
);

named!(unit<&str, Unit>,
    alt!(
        value!(Unit::CM, tag!("cm")) |
        value!(Unit::INCH, tag!("in"))
    )
);
named!(height<&str, Height>,
    do_parse!(
        value: int64 >>
        unit: unit >> (Height {value, unit})
    )
);

named!(hcl<&str, String>,
    map_res!(
        recognize!(
            preceded!(
                tag!("#"),
                many_m_n!(6, 6, one_of!("0123456789abcdef"))
            )
        ),
        FromStr::from_str
    )
);

named!(ecl<&str, String>,
    map_res!(
        recognize!(
            alt!(
                tag!("amb") |
                tag!("blu") |
                tag!("brn") |
                tag!("gry") |
                tag!("grn") |
                tag!("hzl") |
                tag!("oth")
            )
        ),
        FromStr::from_str
    )
);
named!(pid<&str, String>,
    map_res!(
        recognize!(
            many_m_n!(9, 9, one_of!("0123456789"))
        ),
        FromStr::from_str
    )
);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Field {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}
impl Field {
    fn is_valid_data(&self, data: &str) -> bool {
        match self {
            Field::BYR => {
                let result = year(data);
                result.map_or(false, |(r, y)| r.is_empty() && 1920 <= y && y <= 2002)
            }
            Field::IYR => {
                let result = year(data);
                result.map_or(false, |(r, y)| r.is_empty() && 2010 <= y && y <= 2020)
            }
            Field::EYR => {
                let result = year(data);
                result.map_or(false, |(r, y)| r.is_empty() && 2020 <= y && y <= 2030)
            }
            Field::HGT => {
                let result = height(data);
                result.map_or(false, |(r, h)| {
                    r.is_empty()
                        && match h.unit {
                            Unit::CM => 150 <= h.value && h.value <= 193,
                            Unit::INCH => 59 <= h.value && h.value <= 76,
                        }
                })
            }
            Field::HCL => {
                let result = hcl(data);
                result.map_or(false, |(r, _)| r.is_empty())
            }
            Field::ECL => {
                let result = ecl(data);
                result.map_or(false, |(r, _)| r.is_empty())
            }
            Field::PID => {
                let result = pid(data);
                result.map_or(false, |(r, _)| r.is_empty())
            }
            Field::CID => true,
        }
    }
}

#[derive(Clone, Debug)]
struct Passport(HashMap<Field, String>);
impl Passport {
    fn is_valid_a(&self) -> bool {
        vec![
            Field::BYR,
            Field::IYR,
            Field::EYR,
            Field::HGT,
            Field::HCL,
            Field::ECL,
            Field::PID,
        ]
        .iter()
        .all(|f| self.0.contains_key(f))
    }
    fn is_valid_b(&self) -> bool {
        vec![
            Field::BYR,
            Field::IYR,
            Field::EYR,
            Field::HGT,
            Field::HCL,
            Field::ECL,
            Field::PID,
        ]
        .iter()
        .all(|f| self.0.contains_key(f) && f.is_valid_data(&self.0[f]))
    }
}

named!(field<&str, Field>,
    alt!(
        value!(Field::BYR, tag!("byr")) |
        value!(Field::IYR, tag!("iyr")) |
        value!(Field::EYR, tag!("eyr")) |
        value!(Field::HGT, tag!("hgt")) |
        value!(Field::HCL, tag!("hcl")) |
        value!(Field::ECL, tag!("ecl")) |
        value!(Field::PID, tag!("pid")) |
        value!(Field::CID, tag!("cid"))
    )
);

named!(value<&str, String>,
    map!(recognize!(many0!(none_of!(" \n"))), String::from)
);

named!(pair<&str, (Field, String)>,
    do_parse!(
        field: field >>
        tag!(":") >>
        value: value >>
        alt!(
            tag!(" ") |
            line_ending
        ) >> ((field, value)))
);
named!(passport<&str, Passport>,
    do_parse!(
        pairs: many1!(pair) >>
        line_ending >> (Passport(pairs.into_iter().collect()))
    )
);

named!(
    input<&str, Vec<Passport>>,
    many1!(passport)
);

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
    use crate::value;
    use crate::Field;

    #[test]
    fn test_field_byr() {
        assert_eq!(Field::BYR, field("byr").unwrap().1);
    }

    #[test]
    fn test_field_hcl() {
        assert_eq!(Field::HCL, field("hcl").unwrap().1);
    }

    #[test]
    fn test_value() {
        assert_eq!("5d90f0".to_string(), value("5d90f0 ").unwrap().1);
    }

    #[test]
    fn test_pair_0() {
        assert_eq!(
            (Field::HCL, "5d90f0".to_string()),
            pair("hcl:5d90f0 ").unwrap().1
        );
    }
}
