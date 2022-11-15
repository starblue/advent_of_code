use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Reg(char);
impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
enum Arg {
    Int(i64),
    Reg(Reg),
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Arg::Int(a) => write!(f, "{}", a),
            Arg::Reg(r) => write!(f, "{}", r),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(Reg, Arg),
    Sub(Reg, Arg),
    Mul(Reg, Arg),
    Jnz(Arg, Arg),
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Set(r, a) => write!(f, "set {} {}", r, a),
            Instruction::Sub(r, a) => write!(f, "sub {} {}", r, a),
            Instruction::Mul(r, a) => write!(f, "mul {} {}", r, a),
            Instruction::Jnz(a0, a1) => write!(f, "jnz {} {}", a0, a1),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    ip: i64,
    regs: BTreeMap<char, i64>,
    mul_count: usize,
}
impl State {
    fn new() -> State {
        let regs = BTreeMap::new();
        State {
            ip: 0,
            regs,
            mul_count: 0,
        }
    }

    fn run(&mut self, input: &[Instruction]) {
        let len = i64::try_from(input.len()).unwrap();
        while 0 <= self.ip && self.ip < len {
            self.step(input);
        }
    }

    fn step(&mut self, input: &[Instruction]) {
        let instruction = &input[usize::try_from(self.ip).unwrap()];
        if self.ip == 23 && self.regs[&'a'] == 1 {
            if self.value(Arg::Reg(Reg('h'))) < 2 {
                // manipulate state to speed things up
                let g = self.regs[&'g'];
                let d = self.regs.entry('d').or_insert(0);
                *d -= g;
                let g = self.regs.entry('g').or_insert(0);
                *g = 0;
            }
            println!("{}: {} {:?}", self.ip, instruction, self.regs);
        }
        self.execute(instruction);
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Set(r, a) => self.set(*r, *a),
            Instruction::Sub(r, a) => self.sub(*r, *a),
            Instruction::Mul(r, a) => self.mul(*r, *a),
            Instruction::Jnz(a0, a1) => self.jnz(*a0, *a1),
        }
    }

    fn set(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg = val;
        self.ip += 1;
    }
    fn sub(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg -= val;
        self.ip += 1;
    }
    fn mul(&mut self, r: Reg, a: Arg) {
        self.mul_count += 1;
        let val = self.value(a);
        let reg = self.reg(r);
        *reg *= val;
        self.ip += 1;
    }
    fn jnz(&mut self, a0: Arg, a1: Arg) {
        if self.value(a0) != 0 {
            self.ip += self.value(a1);
        } else {
            self.ip += 1;
        }
    }

    fn reg(&mut self, r: Reg) -> &mut i64 {
        self.regs.entry(r.0).or_insert(0)
    }
    fn value(&mut self, a: Arg) -> i64 {
        match a {
            Arg::Int(n) => n,
            Arg::Reg(r) => *self.reg(r),
        }
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn letter(i: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(i)
}

fn reg(i: &str) -> IResult<&str, Reg> {
    let (i, c) = letter(i)?;
    Ok((i, Reg(c)))
}

fn arg_int(i: &str) -> IResult<&str, Arg> {
    let (i, n) = int(i)?;
    Ok((i, Arg::Int(n)))
}
fn arg_reg(i: &str) -> IResult<&str, Arg> {
    let (i, r) = reg(i)?;
    Ok((i, Arg::Reg(r)))
}
fn arg(i: &str) -> IResult<&str, Arg> {
    alt((arg_int, arg_reg))(i)
}

fn instruction_set(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("set ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Set(r, a)))
}
fn instruction_sub(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("sub ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Sub(r, a)))
}
fn instruction_mul(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mul ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Mul(r, a)))
}
fn instruction_jnz(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jnz ")(i)?;
    let (i, a0) = arg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a1) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jnz(a0, a1)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_set,
        instruction_sub,
        instruction_mul,
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
    let result_a = state.mul_count;

    let b = 57 * 100 + 100_000;
    let c = b + 17_000;
    let result_b = (b..=c)
        .step_by(17)
        .filter(|&n| !primal::is_prime(n))
        .count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
