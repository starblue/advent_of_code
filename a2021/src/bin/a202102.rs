use core::fmt;
use core::str::FromStr;

use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Forward(n) => write!(f, "forward {}", n),
            Instruction::Down(n) => write!(f, "down {}", n),
            Instruction::Up(n) => write!(f, "up {}", n),
        }
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn instruction_forward(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("forward ")(i)?;
    let (i, n) = int(i)?;
    Ok((i, Instruction::Forward(n)))
}
fn instruction_down(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("down ")(i)?;
    let (i, n) = int(i)?;
    Ok((i, Instruction::Down(n)))
}
fn instruction_up(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("up ")(i)?;
    let (i, n) = int(i)?;
    Ok((i, Instruction::Up(n)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((instruction_forward, instruction_down, instruction_up))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(i)
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
    // for instruction in &input {
    //     println!("{}", instruction);
    // }

    let pos = input
        .iter()
        .map(|i| match i {
            Instruction::Forward(n) => v2d(*n, 0),
            Instruction::Down(n) => v2d(0, *n),
            Instruction::Up(n) => v2d(0, -*n),
        })
        .fold(p2d(0, 0), |p, v| p + v);
    let result_a = pos.x() * pos.y();

    let mut pos = p2d(0, 0);
    let mut aim = 0;
    for i in &input {
        match i {
            Instruction::Forward(n) => {
                pos += v2d(*n, *n * aim);
            }
            Instruction::Down(n) => {
                aim += n;
            }
            Instruction::Up(n) => {
                aim -= n;
            }
        }
    }
    let result_b = pos.x() * pos.y();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
