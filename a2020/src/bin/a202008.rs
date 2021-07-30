use core::str::FromStr;

use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map_res;
use nom::named;
use nom::recognize;
use nom::tag;
use nom::tuple;
use nom::value;

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Acc => write!(f, "acc"),
            Opcode::Jmp => write!(f, "jmp"),
            Opcode::Nop => write!(f, "nop"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Opcode,
    arg: i64,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:+}", self.opcode, self.arg)
    }
}

named!(int64<&str, i64>,
    map_res!(
        recognize!(
            tuple!(
                alt!(
                    char!('+') |
                    char!('-')
                ),
                digit1
            )
        ),
        FromStr::from_str
    )
);
named!(opcode<&str, Opcode>,
    alt!(
        value!(Opcode::Acc, tag!("acc")) |
        value!(Opcode::Jmp, tag!("jmp")) |
        value!(Opcode::Nop, tag!("nop"))
    )
);
named!(instruction<&str, Instruction>,
    do_parse!(
        opcode: opcode >>
        tag!(" ") >>
        arg: int64 >>
        line_ending >> (Instruction { opcode, arg })
    )
);
named!(
    input<&str, Vec<Instruction>>,
    many1!(instruction)
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Looped,
    Terminated,
    Error,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    ip: i64,
    acc: i64,
}
impl State {
    fn new() -> State {
        State { ip: 0, acc: 0 }
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode {
            Opcode::Acc => {
                self.acc += instruction.arg;
                self.ip += 1;
            }
            Opcode::Jmp => {
                self.ip += instruction.arg;
            }
            Opcode::Nop => {
                self.ip += 1;
            }
        }
    }
    fn run(&mut self, instructions: &[Instruction]) -> Outcome {
        let len = instructions.len() as i64;

        let mut seen = HashSet::new();
        loop {
            if self.ip == len {
                return Outcome::Terminated;
            }
            if seen.contains(&self.ip) {
                return Outcome::Looped;
            }
            if !(0 <= self.ip && self.ip < len) {
                return Outcome::Error;
            }

            seen.insert(self.ip);
            self.execute(instructions[self.ip as usize]);
        }
    }
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

    let instructions = result.unwrap().1;
    // for r in &instructions {
    //     println!("{}", r);
    // }

    let mut state = State::new();
    state.run(&instructions);
    let result_a = state.acc;

    let mut saved_acc = None;
    for i in 0..instructions.len() {
        fn modify(instruction: Instruction) -> Instruction {
            let opcode = match instruction.opcode {
                Opcode::Acc => Opcode::Acc,
                Opcode::Jmp => Opcode::Nop,
                Opcode::Nop => Opcode::Jmp,
            };
            let arg = instruction.arg;
            Instruction { opcode, arg }
        }
        let mut modified_instructions = instructions.clone();
        modified_instructions[i] = modify(instructions[i]);

        let mut state = State::new();
        let outcome = state.run(&modified_instructions);
        if outcome == Outcome::Terminated {
            saved_acc = Some(state.acc);
            break;
        }
    }
    let result_b = saved_acc.unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
