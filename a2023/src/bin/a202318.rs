use core::fmt;
use core::str::FromStr;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::hex_digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_char(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
    fn to_v2d(self) -> Vec2d {
        match self {
            Direction::Up => v2d(0, -1),
            Direction::Down => v2d(0, 1),
            Direction::Left => v2d(-1, 0),
            Direction::Right => v2d(1, 0),
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

#[derive(Clone, Copy, Debug)]
struct ColorCode(u32);
impl fmt::Display for ColorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:06x}", self.0)
    }
}
impl ColorCode {
    fn decode_instruction(&self) -> util::Result<Instruction> {
        let direction = match self.0 & 0xf {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            c => return Err(util::runtime_error!("unknown direction code {}", c)),
        };
        let distance = i64::from(self.0 >> 4);
        Ok(Instruction { direction, distance })
    }
}

#[derive(Clone, Debug)]
struct Item {
    instruction: Instruction,
    color_code: ColorCode,
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.instruction, self.color_code)
    }
}

#[derive(Clone, Debug)]
struct Input {
    items: Vec<Item>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, tag("U")),
        value(Direction::Down, tag("D")),
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    ))(i)
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn hex_uint(i: &str) -> IResult<&str, u32> {
    map_res(recognize(hex_digit1), |s| u32::from_str_radix(s, 16))(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, direction) = direction(i)?;
    let (i, _) = space1(i)?;
    let (i, distance) = uint(i)?;
    Ok((i, Instruction { direction, distance }))
}

fn color_code(i: &str) -> IResult<&str, ColorCode> {
    let (i, _) = tag("#")(i)?;
    let (i, n) = hex_uint(i)?;
    Ok((i, ColorCode(n)))
}

fn item(i: &str) -> IResult<&str, Item> {
    let (i, instruction) = instruction(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = tag("(")(i)?;
    let (i, color_code) = color_code(i)?;
    let (i, _) = tag(")")(i)?;
    Ok((i, Item { instruction, color_code }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, items) = separated_list1(line_ending, item)(i)?;
    Ok((i, Input { items }))
}

fn volume(instructions: &[Instruction]) -> i64 {
    let mut area = 0;
    let mut length = 0;
    let mut p1 = p2d(0, 0);
    let mut p0 = p2d(0, 0);
    for &Instruction { distance, direction } in instructions {
        p1 += distance * direction.to_v2d();

        length += distance;
        area += p0.x() * p1.y() - p0.y() * p1.x();

        p0 = p1;
    }
    (area + length) / 2 + 1
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let instructions1 = input
        .items
        .iter()
        .map(|item| item.instruction)
        .collect::<Vec<_>>();
    let result1 = volume(&instructions1);

    let instructions2 = input
        .items
        .iter()
        .map(|item| item.color_code.decode_instruction())
        .collect::<util::Result<Vec<_>>>()?;
    let result2 = volume(&instructions2);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
