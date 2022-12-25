use core::fmt;

use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use util::runtime_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Digit {
    M2,
    M1,
    Z0,
    P1,
    P2,
}
impl Digit {
    fn value(self) -> i64 {
        match self {
            Digit::M2 => -2,
            Digit::M1 => -1,
            Digit::Z0 => 0,
            Digit::P1 => 1,
            Digit::P2 => 2,
        }
    }
    fn to_char(self) -> char {
        match self {
            Digit::M2 => '=',
            Digit::M1 => '-',
            Digit::Z0 => '0',
            Digit::P1 => '1',
            Digit::P2 => '2',
        }
    }
}
impl TryFrom<i64> for Digit {
    type Error = util::Error;
    fn try_from(value: i64) -> Result<Self, util::Error> {
        match value {
            -2 => Ok(Digit::M2),
            -1 => Ok(Digit::M1),
            0 => Ok(Digit::Z0),
            1 => Ok(Digit::P1),
            2 => Ok(Digit::P2),
            _ => Err(runtime_error!("not a digit value")),
        }
    }
}
impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Snafu {
    value: i64,
}
impl Snafu {
    fn from_digits(digits: &[Digit]) -> Snafu {
        let value = digits.iter().fold(0, |a, d| 5 * a + d.value());
        Snafu { value }
    }
    fn digits(&self) -> util::Result<Vec<Digit>> {
        let mut rest = self.value;
        let mut digits = Vec::new();
        while rest != 0 {
            let rem = rest % 5;
            let digit_value = if rem >= 3 { rem - 5 } else { rem };
            digits.push(Digit::try_from(digit_value)?);
            rest = (rest - digit_value) / 5;
        }
        digits.reverse();
        Ok(digits)
    }
}
impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.digits().map_err(|_| fmt::Error)? {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}
impl From<i64> for Snafu {
    fn from(value: i64) -> Snafu {
        Snafu { value }
    }
}
impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> Self {
        snafu.value
    }
}
impl From<&Snafu> for i64 {
    fn from(snafu: &Snafu) -> Self {
        snafu.value
    }
}

#[derive(Clone, Debug)]
struct Input {
    values: Vec<Snafu>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for value in &self.values {
            writeln!(f, "{}", value)?;
        }
        Ok(())
    }
}

fn digit(i: &str) -> IResult<&str, Digit> {
    alt((
        value(Digit::M2, char('=')),
        value(Digit::M1, char('-')),
        value(Digit::Z0, char('0')),
        value(Digit::P1, char('1')),
        value(Digit::P2, char('2')),
    ))(i)
}

fn snafu(i: &str) -> IResult<&str, Snafu> {
    let (i, digits) = many1(digit)(i)?;
    Ok((i, Snafu::from_digits(&digits)))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, values) = separated_list1(line_ending, snafu)(i)?;
    Ok((i, Input { values }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let sum = input
        .values
        .iter()
        .map(|snafu| i64::from(snafu))
        .sum::<i64>();
    let result1 = Snafu::from(sum);

    println!("Part 1: {}", result1);

    Ok(())
}
