use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Register::A => "a",
                Register::B => "b",
                Register::C => "c",
                Register::D => "d",
            },
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum Source {
    Integer(i64),
    Register(Register),
}
impl Source {
    fn value(&self, state: &State) -> i64 {
        match self {
            Source::Integer(n) => *n,
            Source::Register(r) => state.reg_value(*r),
        }
    }
}
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Source::Integer(n) => write!(f, "{}", n),
            Source::Register(r) => write!(f, "{}", r),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Cpy(Source, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Source, i64),
}
impl Instruction {
    fn execute(&self, state: &mut State) {
        match self {
            Instruction::Cpy(s, r) => {
                let source = s.value(state);
                let dest = state.reg(*r);
                *dest = source;
                state.ip += 1;
            }
            Instruction::Inc(r) => {
                let reg = state.reg(*r);
                *reg += 1;
                state.ip += 1;
            }
            Instruction::Dec(r) => {
                let reg = state.reg(*r);
                *reg -= 1;
                state.ip += 1;
            }
            Instruction::Jnz(s, d) => {
                let source = s.value(state);
                if source != 0 {
                    state.ip += *d;
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
            Instruction::Cpy(s, r) => write!(f, "cpy {} {}", s, r),
            Instruction::Inc(r) => write!(f, "inc {}", r),
            Instruction::Dec(r) => write!(f, "dec {}", r),
            Instruction::Jnz(s, d) => write!(f, "jnz {} {}", s, d),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    ip: i64,
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}
impl State {
    fn new() -> State {
        State {
            ip: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }
    fn reg(&mut self, r: Register) -> &mut i64 {
        match r {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }
    fn reg_value(&self, r: Register) -> i64 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
        }
    }
    fn run(&mut self, program: &[Instruction]) {
        let len = program.len() as i64;
        while (0..len).contains(&self.ip) {
            let instruction = program[self.ip as usize];
            // println!("{} ins: {}", self, instruction);
            instruction.execute(self);
        }
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "ip: {}, a: {}, b: {}, c: {}, d: {}",
            self.ip, self.a, self.b, self.c, self.d
        )
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn register(i: &str) -> IResult<&str, Register> {
    alt((
        value(Register::A, tag("a")),
        value(Register::B, tag("b")),
        value(Register::C, tag("c")),
        value(Register::D, tag("d")),
    ))(i)
}

fn source_integer(i: &str) -> IResult<&str, Source> {
    let (i, n) = int(i)?;
    Ok((i, Source::Integer(n)))
}
fn source_register(i: &str) -> IResult<&str, Source> {
    let (i, r) = register(i)?;
    Ok((i, Source::Register(r)))
}
fn source(i: &str) -> IResult<&str, Source> {
    alt((source_integer, source_register))(i)
}

fn instruction_cpy(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("cpy ")(i)?;
    let (i, s) = source(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Cpy(s, r)))
}
fn instruction_inc(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("inc ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Inc(r)))
}
fn instruction_dec(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("dec ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Dec(r)))
}
fn instruction_jnz(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jnz ")(i)?;
    let (i, s) = source(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, d) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jnz(s, d)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_cpy,
        instruction_inc,
        instruction_dec,
        instruction_jnz,
    ))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for instruction in &input {
    //     println!("{}", instruction);
    // }

    let mut state = State::new();
    state.run(&input);
    let result_a = state.a;

    let mut state = State::new();
    state.c = 1;
    state.run(&input);
    let result_b = state.a;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
