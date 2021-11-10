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
enum Arg {
    Int(i64),
    Reg(Register),
}
impl Arg {
    fn value(&self, state: &State) -> i64 {
        match self {
            Arg::Int(n) => *n,
            Arg::Reg(r) => state.reg_value(*r),
        }
    }
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Arg::Int(n) => write!(f, "{}", n),
            Arg::Reg(r) => write!(f, "{}", r),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Cpy(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
    Tgl(Arg),
}
impl Instruction {
    fn toggle(&self) -> Instruction {
        match self {
            Instruction::Cpy(arg0, arg1) => Instruction::Jnz(*arg0, *arg1),
            Instruction::Inc(arg) => Instruction::Dec(*arg),
            Instruction::Dec(arg) => Instruction::Inc(*arg),
            Instruction::Jnz(arg0, arg1) => Instruction::Cpy(*arg0, *arg1),
            Instruction::Tgl(arg) => Instruction::Inc(*arg),
        }
    }
    fn execute(&self, state: &mut State) {
        match self {
            Instruction::Cpy(arg0, arg1) => {
                if let Arg::Reg(r) = arg1 {
                    let source = arg0.value(state);
                    let dest = state.reg(*r);
                    *dest = source;
                }
                state.ip += 1;
            }
            Instruction::Inc(arg) => {
                if let Arg::Reg(r) = arg {
                    let reg = state.reg(*r);
                    *reg += 1;
                }
                state.ip += 1;
            }
            Instruction::Dec(arg) => {
                if let Arg::Reg(r) = arg {
                    let reg = state.reg(*r);
                    *reg -= 1;
                }
                state.ip += 1;
            }
            Instruction::Jnz(arg0, arg1) => {
                let source = arg0.value(state);
                let d = arg1.value(state);
                if source != 0 {
                    state.ip += d;
                } else {
                    state.ip += 1;
                }
            }
            Instruction::Tgl(arg) => {
                let d = arg.value(state);
                let adr = usize::try_from(state.ip + d).unwrap();
                if let Some(ins) = state.program.get_mut(adr) {
                    *ins = ins.toggle();
                }
                state.ip += 1;
            }
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Cpy(arg0, arg1) => write!(f, "cpy {} {}", arg0, arg1),
            Instruction::Inc(arg) => write!(f, "inc {}", arg),
            Instruction::Dec(arg) => write!(f, "dec {}", arg),
            Instruction::Jnz(arg0, arg1) => write!(f, "jnz {} {}", arg0, arg1),
            Instruction::Tgl(arg) => write!(f, "tgl {}", arg),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    program: Vec<Instruction>,
    ip: i64,
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}
impl State {
    fn new(input: &[Instruction]) -> State {
        State {
            program: input.to_vec(),
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
    fn run(&mut self) {
        let len = self.program.len() as i64;
        while (0..len).contains(&self.ip) {
            let instruction = self.program[self.ip as usize];
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

fn arg_int(i: &str) -> IResult<&str, Arg> {
    let (i, n) = int(i)?;
    Ok((i, Arg::Int(n)))
}
fn arg_reg(i: &str) -> IResult<&str, Arg> {
    let (i, r) = register(i)?;
    Ok((i, Arg::Reg(r)))
}
fn arg(i: &str) -> IResult<&str, Arg> {
    alt((arg_int, arg_reg))(i)
}

fn instruction_cpy(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("cpy ")(i)?;
    let (i, arg0) = arg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, arg1) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Cpy(arg0, arg1)))
}
fn instruction_inc(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("inc ")(i)?;
    let (i, arg) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Inc(arg)))
}
fn instruction_dec(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("dec ")(i)?;
    let (i, arg) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Dec(arg)))
}
fn instruction_jnz(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jnz ")(i)?;
    let (i, arg0) = arg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, arg1) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jnz(arg0, arg1)))
}
fn instruction_tgl(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("tgl ")(i)?;
    let (i, arg) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Tgl(arg)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_cpy,
        instruction_inc,
        instruction_dec,
        instruction_jnz,
        instruction_tgl,
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

    let mut state = State::new(&input);
    state.a = 7;
    state.run();
    let result_a = state.a;

    let mut state = State::new(&input);
    state.a = 12;
    state.run();
    let result_b = state.a;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
