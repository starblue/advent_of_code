use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::many_m_n;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Mask {
    mask: i64,
    value: i64,
}
impl Mask {
    fn apply(&self, v: i64) -> i64 {
        (self.mask & self.value) | (v & !self.mask)
    }
    fn addresses(&self, address: i64) -> Vec<i64> {
        let mut result = Vec::new();

        let base_address = (address & self.mask & !self.value) | (self.mask & self.value);
        let x_bits = self.x_bits();
        for i in 0..(1 << x_bits.len()) {
            let mut a = base_address;
            for j in 0..x_bits.len() {
                if (i & (1 << j)) != 0 {
                    a |= 1 << x_bits[j];
                }
            }
            result.push(a);
        }
        result
    }
    fn x_bits(&self) -> Vec<i64> {
        let mut bits = Vec::new();
        for i in 0..36 {
            let b = 1 << i;
            if (self.mask & b) == 0 {
                bits.push(i);
            }
        }
        bits
    }
}
impl Default for Mask {
    fn default() -> Mask {
        Mask { mask: 0, value: 0 }
    }
}
impl fmt::Display for Mask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..36).rev() {
            let b = 1 << i;
            if (self.mask & b) != 0 {
                if (self.value & b) == 0 {
                    write!(f, "0")?;
                } else {
                    write!(f, "1")?;
                }
            } else {
                write!(f, "X")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Mask(Mask),
    Write { address: i64, value: i64 },
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Mask(mask) => {
                write!(f, "mask = {}", mask)
            }
            Instruction::Write { address, value } => {
                write!(f, "mem[{}] = {}", address, value)
            }
        }
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn mask_bit(i: &str) -> IResult<&str, (i64, i64)> {
    alt((
        value((0, 0), tag("X")),
        value((1, 0), tag("0")),
        value((1, 1), tag("1")),
    ))(i)
}

fn mask(i: &str) -> IResult<&str, Mask> {
    let (i, bits) = many_m_n(36, 36, mask_bit)(i)?;
    Ok((i, {
        let (mask, value) = bits
            .iter()
            .fold((0, 0), |(m, v), (mb, vb)| ((m << 1) | mb, (v << 1) | vb));
        Mask { mask, value }
    }))
}

fn mask_instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mask = ")(i)?;
    let (i, mask) = mask(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Mask(mask)))
}

fn write_instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("mem[")(i)?;
    let (i, address) = int(i)?;
    let (i, _) = tag("] = ")(i)?;
    let (i, value) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Write { address, value }))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((mask_instruction, write_instruction))(i)
}
fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let program = result.unwrap().1;
    // for instruction in &program {
    //     println!("{}", instruction);
    // }

    let mut mem = HashMap::new();
    let mut mask = Mask::default();
    for &instruction in &program {
        match instruction {
            Instruction::Mask(m) => {
                mask = m;
            }
            Instruction::Write { address, value } => {
                mem.insert(address, mask.apply(value));
            }
        }
    }

    let result_a = mem.values().sum::<i64>();

    let mut mem = HashMap::new();
    let mut mask = Mask::default();
    for &instruction in &program {
        match instruction {
            Instruction::Mask(m) => {
                mask = m;
            }
            Instruction::Write { address, value } => {
                for a in mask.addresses(address) {
                    mem.insert(a, value);
                }
            }
        }
    }
    let result_b = mem.values().sum::<i64>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
