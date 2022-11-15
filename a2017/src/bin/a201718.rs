use std::collections::HashMap;
use std::collections::VecDeque;
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
    Snd(Arg),
    Set(Reg, Arg),
    Add(Reg, Arg),
    Mul(Reg, Arg),
    Mod(Reg, Arg),
    Rcv(Arg),
    Jgz(Arg, Arg),
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Snd(a) => write!(f, "snd {}", a),
            Instruction::Set(r, a) => write!(f, "set {} {}", r, a),
            Instruction::Add(r, a) => write!(f, "add {} {}", r, a),
            Instruction::Mul(r, a) => write!(f, "mul {} {}", r, a),
            Instruction::Mod(r, a) => write!(f, "mod {} {}", r, a),
            Instruction::Rcv(a) => write!(f, "rcv {}", a),
            Instruction::Jgz(a0, a1) => write!(f, "jgz {} {}", a0, a1),
        }
    }
}

#[derive(Clone, Debug)]
struct State1 {
    ip: i64,
    regs: HashMap<char, i64>,
    last_sound: Option<i64>,
    recovered_sound: Option<i64>,
}
impl State1 {
    fn new() -> State1 {
        State1 {
            ip: 0,
            regs: HashMap::new(),
            last_sound: None,
            recovered_sound: None,
        }
    }

    fn run(&mut self, input: &[Instruction]) -> i64 {
        loop {
            if let Some(s) = self.recovered_sound {
                return s;
            }
            self.step(input);
        }
    }
    fn step(&mut self, input: &[Instruction]) {
        let instruction = &input[usize::try_from(self.ip).unwrap()];
        self.execute(instruction);
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Snd(a) => self.snd(*a),
            Instruction::Set(r, a) => self.set(*r, *a),
            Instruction::Add(r, a) => self.add(*r, *a),
            Instruction::Mul(r, a) => self.mul(*r, *a),
            Instruction::Mod(r, a) => self.mod_(*r, *a),
            Instruction::Rcv(a) => self.rcv(*a),
            Instruction::Jgz(a0, a1) => self.jgz(*a0, *a1),
        }
    }

    fn snd(&mut self, a: Arg) {
        self.last_sound = Some(self.value(a));
        self.ip += 1;
    }
    fn set(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg = val;
        self.ip += 1;
    }
    fn add(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg += val;
        self.ip += 1;
    }
    fn mul(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg *= val;
        self.ip += 1;
    }
    fn mod_(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg %= val;
        self.ip += 1;
    }
    fn rcv(&mut self, a: Arg) {
        if self.value(a) != 0 {
            self.recovered_sound = self.last_sound;
        }
        self.ip += 1;
    }
    fn jgz(&mut self, a0: Arg, a1: Arg) {
        if self.value(a0) > 0 {
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

#[derive(Clone, Debug)]
struct State2 {
    ip: i64,
    regs: HashMap<char, i64>,
    sent_value: Option<i64>,
    recv_queue: VecDeque<i64>,
    waiting: bool,
}
impl State2 {
    fn new(program_id: i64) -> State2 {
        let mut regs = HashMap::new();
        regs.insert('p', program_id);
        State2 {
            ip: 0,
            regs,
            sent_value: None,
            recv_queue: VecDeque::new(),
            waiting: false,
        }
    }

    fn step(&mut self, input: &[Instruction]) {
        let instruction = &input[usize::try_from(self.ip).unwrap()];
        self.execute(instruction);
    }
    fn is_waiting(&self) -> bool {
        self.waiting && self.recv_queue.is_empty()
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Snd(a) => self.snd(*a),
            Instruction::Set(r, a) => self.set(*r, *a),
            Instruction::Add(r, a) => self.add(*r, *a),
            Instruction::Mul(r, a) => self.mul(*r, *a),
            Instruction::Mod(r, a) => self.mod_(*r, *a),
            Instruction::Rcv(a) => self.rcv(*a),
            Instruction::Jgz(a0, a1) => self.jgz(*a0, *a1),
        }
    }

    fn snd(&mut self, a: Arg) {
        self.sent_value = Some(self.value(a));
        self.ip += 1;
    }
    fn set(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg = val;
        self.ip += 1;
    }
    fn add(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg += val;
        self.ip += 1;
    }
    fn mul(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg *= val;
        self.ip += 1;
    }
    fn mod_(&mut self, r: Reg, a: Arg) {
        let val = self.value(a);
        let reg = self.reg(r);
        *reg %= val;
        self.ip += 1;
    }
    fn rcv(&mut self, a: Arg) {
        if let Arg::Reg(r) = a {
            if let Some(val) = self.recv_queue.pop_front() {
                self.waiting = false;
                let reg = self.reg(r);
                *reg = val;
                self.ip += 1;
            } else {
                self.waiting = true;
            }
        } else {
            panic!("rcv with non-register argument");
        }
    }
    fn jgz(&mut self, a0: Arg, a1: Arg) {
        if self.value(a0) > 0 {
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

fn instruction_snd(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("snd ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Snd(a)))
}
fn instruction_set(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("set ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Set(r, a)))
}
fn instruction_add(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("add ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Add(r, a)))
}
fn instruction_mul(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mul ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Mul(r, a)))
}
fn instruction_mod(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mod ")(i)?;
    let (i, r) = reg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Mod(r, a)))
}
fn instruction_rcv(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("rcv ")(i)?;
    let (i, a) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Rcv(a)))
}
fn instruction_jgz(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("jgz ")(i)?;
    let (i, a0) = arg(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a1) = arg(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Jgz(a0, a1)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_snd,
        instruction_set,
        instruction_add,
        instruction_mul,
        instruction_mod,
        instruction_rcv,
        instruction_jgz,
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

    let mut state = State1::new();
    let result_a = state.run(&input);

    let mut count = 0;
    let mut state0 = State2::new(0);
    let mut state1 = State2::new(1);
    while !(state0.is_waiting() && state1.is_waiting()) {
        state0.step(&input);
        if let Some(val) = state0.sent_value {
            state0.sent_value = None;
            state1.recv_queue.push_back(val);
        }

        state1.step(&input);
        if let Some(val) = state1.sent_value {
            count += 1;
            state1.sent_value = None;
            state0.recv_queue.push_back(val);
        }
    }
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
