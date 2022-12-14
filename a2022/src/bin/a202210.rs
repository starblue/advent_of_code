use core::fmt;
use core::str::FromStr;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Addx(i64),
    Noop,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Addx(v) => write!(f, "addx {}", v),
            Instruction::Noop => write!(f, "noop"),
        }
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn instruction_addx(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("addx ")(i)?;
    let (i, v) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Addx(v)))
}
fn instruction_noop(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("noop")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Noop))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((instruction_addx, instruction_noop))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many0(instruction)(i)
}

#[derive(Clone, Debug)]
struct Cpu {
    cycle: i64,
    x: i64,
    next_cycle: i64,
    next_x: i64,
}
impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycle: 0,
            x: 1,
            next_cycle: 0,
            next_x: 1,
        }
    }
    fn signal_strength(&self) -> i64 {
        self.cycle * self.x
    }
    fn step(&mut self, instructions: &mut dyn Iterator<Item = &Instruction>) {
        self.cycle += 1;
        if self.cycle >= self.next_cycle {
            self.x = self.next_x;
            if let Some(instruction) = instructions.next() {
                match instruction {
                    Instruction::Addx(v) => {
                        self.next_cycle = self.cycle + 2;
                        self.next_x = self.x + v;
                    }
                    Instruction::Noop => {
                        self.next_cycle = self.cycle + 1;
                        self.next_x = self.x;
                    }
                }
            }
        }
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for instruction in &input {
    //     println!("{}", instruction);
    // }

    let mut sum = 0;

    let mut cpu = Cpu::new();
    let mut instructions = input.iter();

    let sample_times = vec![20, 60, 100, 140, 180, 220];
    let mut sample_index = 0;
    while sample_index < sample_times.len() {
        if cpu.cycle == sample_times[sample_index] {
            sample_index += 1;
            sum += cpu.signal_strength();
        }
        cpu.step(&mut instructions);
    }
    let result1 = sum;

    let mut cpu = Cpu::new();
    let mut instructions = input.iter();
    for _ in 0..6 {
        for x in 0..40 {
            cpu.step(&mut instructions);
            if (cpu.x - x).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    let result2 = "ZRARLFZU";

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
