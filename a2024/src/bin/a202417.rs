use core::fmt;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
struct Registers([i64; 3]);
impl Registers {
    fn get(&self, index: usize) -> i64 {
        self.0[index]
    }
    fn get_mut(&mut self, index: usize) -> &mut i64 {
        &mut self.0[index]
    }
}
impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Register A: {}", self.get(0))?;
        writeln!(f, "Register B: {}", self.get(1))?;
        writeln!(f, "Register C: {}", self.get(2))?;
        Ok(())
    }
}

fn registers(i: &str) -> IResult<&str, Registers> {
    let (i, _) = tag("Register A: ")(i)?;
    let (i, a) = i64(i)?;
    let (i, _) = line_ending(i)?;

    let (i, _) = tag("Register B: ")(i)?;
    let (i, b) = i64(i)?;
    let (i, _) = line_ending(i)?;

    let (i, _) = tag("Register C: ")(i)?;
    let (i, c) = i64(i)?;
    let (i, _) = line_ending(i)?;

    Ok((i, Registers([a, b, c])))
}

#[derive(Clone, Debug)]
struct Program(Vec<i64>);
impl Program {
    fn get(&self, index: usize) -> Option<i64> {
        self.0.get(index).cloned()
    }
    fn as_slice(&self) -> &[i64] {
        self.0.as_slice()
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program: ")?;
        let mut sep = "";
        for instruction in &self.0 {
            write!(f, "{}{}", sep, instruction)?;
            sep = ",";
        }
        writeln!(f)?;
        Ok(())
    }
}

fn program(i: &str) -> IResult<&str, Program> {
    let (i, _) = tag("Program: ")(i)?;
    let (i, instructions) = separated_list1(tag(","), i64)(i)?;
    let (i, _) = line_ending(i)?;

    Ok((i, Program(instructions)))
}

#[derive(Clone, Debug)]
struct Input {
    registers: Registers,
    program: Program,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.registers)?;
        writeln!(f, "{}", self.program)?;
        Ok(())
    }
}
fn input(i: &str) -> IResult<&str, Input> {
    let (i, registers) = registers(i)?;
    let (i, _) = line_ending(i)?;
    let (i, program) = program(i)?;

    Ok((i, Input { registers, program }))
}

#[derive(Clone, Debug)]
struct Output(Vec<i64>);
impl Output {
    fn new() -> Output {
        Output(Vec::new())
    }
    fn add(&mut self, value: i64) {
        self.0.push(value);
    }
    fn as_slice(&self) -> &[i64] {
        self.0.as_slice()
    }
}
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";
        for value in &self.0 {
            write!(f, "{}{}", sep, value)?;
            sep = ",";
        }
        Ok(())
    }
}

const OPCODE_ADV: i64 = 0;
const OPCODE_BXL: i64 = 1;
const OPCODE_BST: i64 = 2;
const OPCODE_JNZ: i64 = 3;
const OPCODE_BXC: i64 = 4;
const OPCODE_OUT: i64 = 5;
const OPCODE_BDV: i64 = 6;
const OPCODE_CDV: i64 = 7;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[derive(Clone, Debug)]
struct State {
    ip: usize,
    registers: Registers,
    output: Output,
}
impl State {
    fn init(registers: Registers) -> State {
        let ip = 0;
        let output = Output::new();
        State {
            ip,
            registers,
            output,
        }
    }
    fn combo_operand(&self, operand: i64) -> Result<i64> {
        Ok(match operand {
            0..=3 => operand,
            4 => self.registers.get(REG_A),
            5 => self.registers.get(REG_B),
            6 => self.registers.get(REG_C),
            _ => return Err("unexpected combo operand".into()),
        })
    }
    fn execute(&mut self, opcode: i64, operand: i64) -> Result<()> {
        match opcode {
            OPCODE_ADV => {
                let exponent = self.combo_operand(operand)?;
                let divisor = 2_i64.pow(u32::try_from(exponent)?);
                let a = self.registers.get_mut(REG_A);
                *a /= divisor;
                self.ip += 2;
            }
            OPCODE_BXL => {
                let b = self.registers.get_mut(REG_B);
                *b ^= operand;
                self.ip += 2;
            }
            OPCODE_BST => {
                let value = self.combo_operand(operand)?;
                let b = self.registers.get_mut(REG_B);
                *b = value % 8;
                self.ip += 2;
            }
            OPCODE_JNZ => {
                let a = self.registers.get(REG_A);
                if a != 0 {
                    self.ip = usize::try_from(operand)?;
                } else {
                    self.ip += 2;
                }
            }
            OPCODE_BXC => {
                let c = self.registers.get(REG_C);
                let b = self.registers.get_mut(REG_B);
                *b ^= c;
                self.ip += 2;
            }
            OPCODE_OUT => {
                let value = self.combo_operand(operand)?;
                self.output.add(value % 8);
                self.ip += 2;
            }
            OPCODE_BDV => {
                let exponent = self.combo_operand(operand)?;
                let divisor = 2_i64.pow(u32::try_from(exponent)?);
                let a = self.registers.get(REG_A);
                let b = self.registers.get_mut(REG_B);
                *b = a / divisor;
                self.ip += 2;
            }
            OPCODE_CDV => {
                let exponent = self.combo_operand(operand)?;
                let divisor = 2_i64.pow(u32::try_from(exponent)?);
                let a = self.registers.get(REG_A);
                let c = self.registers.get_mut(REG_C);
                *c = a / divisor;
                self.ip += 2;
            }
            _ => return Err("unexpected opcode".into()),
        }
        Ok(())
    }
    fn run(&mut self, program: &Program) -> Result<()> {
        while let Some(opcode) = program.get(self.ip) {
            if let Some(operand) = program.get(self.ip + 1) {
                self.execute(opcode, operand)?;
            } else {
                return Err("missing operand".into());
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut state = State::init(input.registers);
    state.run(&input.program)?;
    let result1 = state.output;

    // Program:
    // 0: 2,4  BST A  B = A MOD 8
    //    1,7  BXL 7  B = B XOR 7
    //    7,5  CDV B  C = A DIV 2**B
    //    1,7  BXL 7  B = B XOR 7
    //    4,6  BXC 6  B = B XOR C
    //    0,3  ADV 3  A = A DIV 2**3
    //    5,5  OUT B  OUT B MOD 8
    //    3,0  JNZ 0
    //
    // Each output is the corresponding octal digit xored with some higher bits,
    // with the least significant digit producing the first output.
    // Start guessing at the most significant digit and work backwards
    // as long as that the end of the output and the end of the program match.

    let mut octal_digits = [0; 16];
    let mut i = 15;
    let mut a;
    loop {
        // Try the next digit at the highest index
        // where output and program differ.
        octal_digits[i] += 1;
        for j in 0..i {
            octal_digits[j] = 0;
        }

        // Compute register A from the octal digits.
        a = 0;
        let mut pow = 1;
        for d in octal_digits {
            a += pow * d;
            pow *= 8;
        }

        // Run the machine.
        let mut state = State::init(input.registers);
        *state.registers.get_mut(REG_A) = a;
        state.run(&input.program)?;

        // Compare the output to the program.
        let outputs = state.output.as_slice();
        let instructions = input.program.as_slice();
        if outputs == instructions {
            break;
        }

        // Find the highest index where they differ.
        // Must be at least zero, since they are not equal.
        i = 15;
        while outputs[i] == instructions[i] {
            i -= 1;
        }
    }
    let result2 = a;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
