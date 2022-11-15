use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}
impl Instruction {
    fn execute(&self, state: &mut State) {
        match self {
            Instruction::Rect(a, b) => state.rect(*a, *b),
            Instruction::RotateRow(a, b) => state.rotate_row(*a, *b),
            Instruction::RotateColumn(a, b) => state.rotate_column(*a, *b),
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Rect(a, b) => write!(f, "rect {}x{}", a, b),
            Instruction::RotateRow(a, b) => write!(f, "rotate row y={} by {}", a, b),
            Instruction::RotateColumn(a, b) => write!(f, "rotate column x={} by {}", a, b),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    screen: Vec<Vec<bool>>,
}
impl State {
    fn new() -> State {
        let screen = vec![vec![false; 50]; 6];
        State { screen }
    }
    fn rect(&mut self, a: usize, b: usize) {
        for x in 0..a {
            for y in 0..b {
                self.screen[y][x] = true;
            }
        }
    }
    fn rotate_row(&mut self, y: usize, n: usize) {
        let row = self.screen[y].clone();
        for x in 0..50 {
            self.screen[y][x] = row[(x + 50 - n) % 50];
        }
    }
    fn rotate_column(&mut self, x: usize, n: usize) {
        let column = (0..6).map(|y| self.screen[y][x]).collect::<Vec<_>>();
        for y in 0..6 {
            self.screen[y][x] = column[(y + 6 - n) % 6];
        }
    }
    fn lit_pixel_count(&self) -> usize {
        self.screen
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for row in &self.screen {
            for &b in row {
                write!(f, "{}", if b { "#" } else { "." })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn instruction_rect(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("rect ")(i)?;
    let (i, a) = uint(i)?;
    let (i, _) = tag("x")(i)?;
    let (i, b) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Rect(a, b)))
}

fn instruction_rotate_row(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("rotate row y=")(i)?;
    let (i, a) = uint(i)?;
    let (i, _) = tag(" by ")(i)?;
    let (i, b) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::RotateRow(a, b)))
}

fn instruction_rotate_column(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("rotate column x=")(i)?;
    let (i, a) = uint(i)?;
    let (i, _) = tag(" by ")(i)?;
    let (i, b) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::RotateColumn(a, b)))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_rect,
        instruction_rotate_row,
        instruction_rotate_column,
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
    for instruction in &input {
        instruction.execute(&mut state);
        // println!("{}", instruction);
        // println!("{}", state);
    }
    let result_a = state.lit_pixel_count();

    println!("{}", state);
    let result_b = "EOARGPHYAO";

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
