use core::str::FromStr;

use std::fmt;
use std::io;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Matrix2d;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}
impl Action {
    fn to_char(&self) -> char {
        match self {
            Action::North => 'N',
            Action::South => 'S',
            Action::East => 'E',
            Action::West => 'W',
            Action::Left => 'L',
            Action::Right => 'R',
            Action::Forward => 'F',
        }
    }
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    action: Action,
    arg: i64,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.action, self.arg)
    }
}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn action(i: &str) -> IResult<&str, Action> {
    alt((
        value(Action::North, tag("N")),
        value(Action::South, tag("S")),
        value(Action::East, tag("E")),
        value(Action::West, tag("W")),
        value(Action::Left, tag("L")),
        value(Action::Right, tag("R")),
        value(Action::Forward, tag("F")),
    ))(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, action) = action(i)?;
    let (i, arg) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction { action, arg }))
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let instructions = result.unwrap().1;
    // for r in &instructions {
    //     println!("{}", r);
    // }

    let left = Matrix2d::rotate_left_90();
    let right = Matrix2d::rotate_right_90();

    let mut p = p2d(0, 0);
    // the direction the ship is facing
    let mut d = v2d(1, 0);
    for &i in &instructions {
        match i.action {
            Action::North => {
                p += i.arg * v2d(0, 1);
            }
            Action::South => {
                p += i.arg * v2d(0, -1);
            }
            Action::East => {
                p += i.arg * v2d(1, 0);
            }
            Action::West => {
                p += i.arg * v2d(-1, 0);
            }
            Action::Left => {
                let mut arg = i.arg;
                while arg > 0 {
                    d = left * d;
                    arg -= 90;
                }
            }
            Action::Right => {
                let mut arg = i.arg;
                while arg > 0 {
                    d = right * d;
                    arg -= 90;
                }
            }
            Action::Forward => {
                p += i.arg * d;
            }
        }
    }

    let result_a = p.x().abs() + p.y().abs();

    let mut p = p2d(0, 0);
    // the waypoint
    let mut d = v2d(10, 1);
    for &i in &instructions {
        match i.action {
            Action::North => {
                d += i.arg * v2d(0, 1);
            }
            Action::South => {
                d += i.arg * v2d(0, -1);
            }
            Action::East => {
                d += i.arg * v2d(1, 0);
            }
            Action::West => {
                d += i.arg * v2d(-1, 0);
            }
            Action::Left => {
                let mut arg = i.arg;
                while arg > 0 {
                    d = left * d;
                    arg -= 90;
                }
            }
            Action::Right => {
                let mut arg = i.arg;
                while arg > 0 {
                    d = right * d;
                    arg -= 90;
                }
            }
            Action::Forward => {
                p += i.arg * d;
            }
        }
    }
    let result_b = p.x().abs() + p.y().abs();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
