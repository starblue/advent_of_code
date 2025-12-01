use core::str::FromStr;

use std::fmt;
use std::fmt::Display;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}
impl Direction {
    fn to_char(self) -> char {
        match self {
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(i)
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

#[derive(Clone, Copy, Debug)]
struct Rotation {
    direction: Direction,
    distance: i64,
}
impl Rotation {
    fn delta(&self) -> i64 {
        match self.direction {
            Direction::Left => -self.distance,
            Direction::Right => self.distance,
        }
    }
}
impl Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

fn rotation(i: &str) -> IResult<&str, Rotation> {
    let (i, direction) = direction(i)?;
    let (i, distance) = uint(i)?;
    Ok((
        i,
        Rotation {
            direction,
            distance,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Rotation>> {
    separated_list1(line_ending, rotation)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for rotation in &input {
    //     println!("{}", rotation);
    // }

    let modulus = 100;

    let mut count = 0;
    let mut dial = 50;
    for rotation in &input {
        dial = (dial + modulus + rotation.delta()) % modulus;
        if dial == 0 {
            count += 1;
        }
    }
    let result1 = count;

    let mut count = 0;
    let mut dial = 50;
    for rotation in &input {
        let delta = match rotation.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        for _ in 0..rotation.distance {
            dial += delta;
            dial = (dial + modulus) % modulus;
            if dial == 0 {
                count += 1;
            }
        }
    }
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
