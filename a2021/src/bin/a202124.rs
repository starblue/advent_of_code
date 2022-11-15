use core::fmt;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Op::Add => write!(f, "add"),
            Op::Mul => write!(f, "mul"),
            Op::Div => write!(f, "div"),
            Op::Mod => write!(f, "mod"),
            Op::Eql => write!(f, "eql"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Register {
    W,
    X,
    Y,
    Z,
}
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Register::W => "w",
                Register::X => "x",
                Register::Y => "y",
                Register::Z => "z",
            },
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Source {
    Integer(i64),
    Register(Register),
}
impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Source::Integer(n) => write!(f, "{}", n),
            Source::Register(r) => write!(f, "{}", r),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Inp(Register),
    Op(Op, Register, Source),
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Inp(r) => write!(f, "inp {}", r),
            Instruction::Op(op, r, s) => write!(f, "{} {} {}", op, r, s),
        }
    }
}

fn register(i: &str) -> IResult<&str, Register> {
    alt((
        value(Register::W, tag("w")),
        value(Register::X, tag("x")),
        value(Register::Y, tag("y")),
        value(Register::Z, tag("z")),
    ))(i)
}

fn source_integer(i: &str) -> IResult<&str, Source> {
    let (i, n) = i64(i)?;
    Ok((i, Source::Integer(n)))
}
fn source_register(i: &str) -> IResult<&str, Source> {
    let (i, r) = register(i)?;
    Ok((i, Source::Register(r)))
}
fn source(i: &str) -> IResult<&str, Source> {
    alt((source_integer, source_register))(i)
}

fn op(i: &str) -> IResult<&str, Op> {
    alt((
        value(Op::Add, tag("add")),
        value(Op::Mul, tag("mul")),
        value(Op::Div, tag("div")),
        value(Op::Mod, tag("mod")),
        value(Op::Eql, tag("eql")),
    ))(i)
}

fn instruction_inp(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("inp ")(i)?;
    let (i, r) = register(i)?;
    Ok((i, Instruction::Inp(r)))
}
fn instruction_op(i: &str) -> IResult<&str, Instruction> {
    let (i, op) = op(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, r) = register(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, s) = source(i)?;
    Ok((i, Instruction::Op(op, r, s)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((instruction_inp, instruction_op))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(i)
}

// The code consists of 14 almost identical parts whose only difference
// is in three numbers c0, c1 and c2 (see instructions below).
// Each part handles one input digit, read into w at the beginning.
// z is the only register passed from one part to the next.
// It essentially contains a stack of numbers modulo 26.
// If c0 is 1 then the part pushes w + c2.
// If c0 is 26 then the part pops a value w0 + c2 from a previous input digit
// and adds c1 to get w0 + c2 + c1.
// This must be equal to the current w for a valid serial number,
// otherwise a residual value is added to z which can't be canceled later.

//  0: inp w
//  1: mul x 0
//  2: add x z
//  3: mod x 26
//  4: div z c0
//  5: add x c1
//  6: eql x w
//  7: eql x 0
//  8: mul y 0
//  9: add y 25
// 10: mul y x
// 11: add y 1
// 12: mul z y
// 13: mul y 0
// 14: add y w
// 15: add y c2
// 16: mul y x
// 17: add z y

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for instruction in &input {
    //     println!("{}", instruction);
    // }

    let mut parameters = Vec::new();
    for i in 0..14 {
        let c0 = match input[i * 18 + 4] {
            Instruction::Op(Op::Div, _dest, Source::Integer(n)) => n,
            _ => panic!("unexpected instruction for c0"),
        };
        let c1 = match input[i * 18 + 5] {
            Instruction::Op(Op::Add, _dest, Source::Integer(n)) => n,
            _ => panic!("unexpected instruction for c1"),
        };
        let c2 = match input[i * 18 + 15] {
            Instruction::Op(Op::Add, _dest, Source::Integer(n)) => n,
            _ => panic!("unexpected instruction for c2"),
        };
        parameters.push((c0, c1, c2));
    }

    let mut digits = vec![0; 14];
    let mut stack = Vec::new();
    for (i, &(c0, c1, c2)) in parameters.iter().enumerate() {
        if c0 != 26 {
            stack.push((i, c2));
        } else {
            let (i0, c2) = stack.pop().unwrap();
            // Goal: digits[i0] + c2 + c1 == digits[i] and digits maximal
            if c2 + c1 >= 0 {
                digits[i0] = 9 - (c2 + c1);
                digits[i] = 9;
            } else {
                digits[i0] = 9;
                digits[i] = 9 + (c2 + c1);
            }
        }
    }
    let result_a = digits.into_iter().fold(0, |n, d| n * 10 + d);

    let mut digits = vec![0; 14];
    let mut stack = Vec::new();
    for (i, &(c0, c1, c2)) in parameters.iter().enumerate() {
        if c0 != 26 {
            stack.push((i, c2));
        } else {
            let (i0, c2) = stack.pop().unwrap();
            // Goal: digits[i0] + c2 + c1 == digits[i] and digits minimal
            if c2 + c1 >= 0 {
                digits[i0] = 1;
                digits[i] = 1 + (c2 + c1);
            } else {
                digits[i0] = 1 - (c2 + c1);
                digits[i] = 1;
            }
        }
    }
    let result_b = digits.into_iter().fold(0, |n, d| n * 10 + d);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
