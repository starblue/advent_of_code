use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum CharLiteral {
    EscapedDoubleQuote,
    EscapedBackslash,
    EscapedHex(u8),
    Other(char),
}
impl CharLiteral {
    fn code_len(&self) -> usize {
        match self {
            CharLiteral::EscapedDoubleQuote => 2,
            CharLiteral::EscapedBackslash => 2,
            CharLiteral::EscapedHex(_) => 4,
            CharLiteral::Other(_) => 1,
        }
    }
    fn code2_len(&self) -> usize {
        match self {
            CharLiteral::EscapedDoubleQuote => 4,
            CharLiteral::EscapedBackslash => 4,
            CharLiteral::EscapedHex(_) => 5,
            CharLiteral::Other(_) => 1,
        }
    }
    fn mem_len(&self) -> usize {
        1
    }
}
impl fmt::Display for CharLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CharLiteral::EscapedDoubleQuote => write!(f, "\\\""),
            CharLiteral::EscapedBackslash => write!(f, "\\\\"),
            CharLiteral::EscapedHex(b) => write!(f, "\\x{:2x}", b),
            CharLiteral::Other(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Clone, Debug)]
struct StringLiteral(Vec<CharLiteral>);
impl StringLiteral {
    fn code_len(&self) -> usize {
        2 + self.0.iter().map(CharLiteral::code_len).sum::<usize>()
    }
    fn mem_len(&self) -> usize {
        self.0.iter().map(CharLiteral::mem_len).sum::<usize>()
    }
    fn overhead(&self) -> usize {
        self.code_len() - self.mem_len()
    }
    fn code2_len(&self) -> usize {
        6 + self.0.iter().map(CharLiteral::code2_len).sum::<usize>()
    }
    fn overhead2(&self) -> usize {
        self.code2_len() - self.code_len()
    }
}
impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "\"")?;
        for cl in &self.0 {
            write!(f, "{}", cl)?;
        }
        write!(f, "\"")
    }
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn from_hex(i: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(i, 16)
}

fn hex_primary(i: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(i)
}

fn char_literal_escaped_double_quote(i: &str) -> IResult<&str, CharLiteral> {
    value(CharLiteral::EscapedDoubleQuote, tag("\\\""))(i)
}

fn char_literal_escaped_backslash(i: &str) -> IResult<&str, CharLiteral> {
    value(CharLiteral::EscapedBackslash, tag("\\\\"))(i)
}

fn char_literal_escaped_hex(i: &str) -> IResult<&str, CharLiteral> {
    let (i, _) = tag("\\x")(i)?;
    let (i, b) = hex_primary(i)?;
    Ok((i, CharLiteral::EscapedHex(b)))
}

fn char_literal_other(i: &str) -> IResult<&str, CharLiteral> {
    let (i, c) = none_of("\"\\\n")(i)?;
    Ok((i, CharLiteral::Other(c)))
}

fn char_literal(i: &str) -> IResult<&str, CharLiteral> {
    alt((
        char_literal_escaped_double_quote,
        char_literal_escaped_backslash,
        char_literal_escaped_hex,
        char_literal_other,
    ))(i)
}

fn line(i: &str) -> IResult<&str, StringLiteral> {
    let (i, _) = char('"')(i)?;
    let (i, char_literals) = many0(char_literal)(i)?;
    let (i, _) = char('"')(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, StringLiteral(char_literals)))
}

fn input(i: &str) -> IResult<&str, Vec<StringLiteral>> {
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
    //println!("{:#?}", input);

    let result_a = input.iter().map(StringLiteral::overhead).sum::<usize>();

    let result_b = input.iter().map(StringLiteral::overhead2).sum::<usize>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
