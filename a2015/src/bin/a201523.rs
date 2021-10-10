use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
}
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Register::A => write!(f, "a"),
            Register::B => write!(f, "b"),
        }
    }
}

type Offset = i64;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}
impl Instruction {
    fn execute(&self, state: &mut State) {
        match self {
            Instruction::Hlf(r) => {
                *state.reg(*r) /= 2;
                state.ip += 1;
            }
            Instruction::Tpl(r) => {
                *state.reg(*r) *= 3;
                state.ip += 1;
            }
            Instruction::Inc(r) => {
                *state.reg(*r) += 1;
                state.ip += 1;
            }
            Instruction::Jmp(offset) => {
                state.ip += offset;
            }
            Instruction::Jie(r, offset) => {
                if *state.reg(*r) % 2 == 0 {
                    state.ip += offset;
                } else {
                    state.ip += 1;
                }
            }
            Instruction::Jio(r, offset) => {
                if *state.reg(*r) == 1 {
                    state.ip += offset;
                } else {
                    state.ip += 1;
                }
            }
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Hlf(r) => write!(f, "hlf {}", *r),
            Instruction::Tpl(r) => write!(f, "tpl {}", *r),
            Instruction::Inc(r) => write!(f, "inc {}", *r),
            Instruction::Jmp(offset) => write!(f, "jmp {:+}", offset),
            Instruction::Jie(r, offset) => write!(f, "jie {}, {:+}", *r, offset),
            Instruction::Jio(r, offset) => write!(f, "jio {}, {:+}", *r, offset),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    ip: i64,
    a: i64,
    b: i64,
}
impl State {
    fn new() -> State {
        State { ip: 0, a: 0, b: 0 }
    }
    fn reg(&mut self, r: Register) -> &mut i64 {
        match r {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
    fn run(&mut self, program: &[Instruction]) {
        loop {
            let ip = usize::try_from(self.ip).unwrap();
            if !(0..program.len()).contains(&ip) {
                break;
            }
            let instruction = program[ip];
            instruction.execute(self);
        }
    }
}

fn offset(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((alt((char('+'), char('-'))), digit1))),
        FromStr::from_str,
    )(i)
}

fn register(i: &str) -> IResult<&str, Register> {
    let p0 = value(Register::A, tag("a"));
    let p1 = value(Register::B, tag("b"));
    alt((p0, p1))(i)
}

fn instruction_hlf(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("hlf ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Hlf(r)))
}
fn instruction_tpl(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("tpl ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Tpl(r)))
}
fn instruction_inc(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("inc ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Inc(r)))
}
fn instruction_jmp(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jmp ")(i)?;
    let (i, offset) = offset(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jmp(offset)))
}
fn instruction_jie(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jie ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = tag(", ")(i)?;
    let (i, offset) = offset(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jie(r, offset)))
}
fn instruction_jio(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jio ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = tag(", ")(i)?;
    let (i, offset) = offset(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jio(r, offset)))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_hlf,
        instruction_tpl,
        instruction_inc,
        instruction_jmp,
        instruction_jie,
        instruction_jio,
    ))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
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
    // for instruction in &input {
    //     println!("{}", instruction);
    // }

    let mut state = State::new();
    state.run(&input);
    let result_a = state.b;

    let mut state = State::new();
    state.a = 1;
    state.run(&input);
    let result_b = state.b;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
