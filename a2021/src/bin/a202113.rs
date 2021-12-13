use core::fmt;
use core::str::FromStr;

use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::BBox2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    FoldAlongX(i64),
    FoldAlongY(i64),
}
impl Instruction {
    fn apply(&self, p: Point2d) -> Point2d {
        match self {
            Instruction::FoldAlongX(x) => {
                let dx = p.x() - x;
                let new_x = x - dx.abs();
                p2d(new_x, p.y())
            }
            Instruction::FoldAlongY(y) => {
                let dy = p.y() - y;
                let new_y = y - dy.abs();
                p2d(p.x(), new_y)
            }
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::FoldAlongX(x) => write!(f, "fold along x={}", x),
            Instruction::FoldAlongY(y) => write!(f, "fold along y={}", y),
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    dots: Vec<Point2d>,
    instructions: Vec<Instruction>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for p in &self.dots {
            writeln!(f, "{},{}", p.x(), p.y())?;
        }
        writeln!(f)?;
        for i in &self.instructions {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn dot(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = uint(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = uint(i)?;
    Ok((i, p2d(x, y)))
}

fn instruction_fold_along_x(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("fold along x=")(i)?;
    let (i, x) = uint(i)?;
    Ok((i, Instruction::FoldAlongX(x)))
}
fn instruction_fold_along_y(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("fold along y=")(i)?;
    let (i, y) = uint(i)?;
    Ok((i, Instruction::FoldAlongY(y)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((instruction_fold_along_x, instruction_fold_along_y))(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, dots) = separated_list1(line_ending, dot)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, instructions) = separated_list1(line_ending, instruction)(i)?;
    Ok((i, Input { dots, instructions }))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let instruction = &input.instructions[0];
    let dots = input
        .dots
        .iter()
        .map(|&p| instruction.apply(p))
        .collect::<HashSet<_>>();
    let result_a = dots.len();

    let mut dots = input.dots.into_iter().collect::<HashSet<_, _>>();
    for &instruction in &input.instructions {
        dots = dots
            .into_iter()
            .map(|p| instruction.apply(p))
            .collect::<HashSet<_>>();
    }
    let bbox = BBox2d::enclosing(&dots).unwrap();
    for y in bbox.y_range() {
        for x in bbox.x_range() {
            if dots.contains(&p2d(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    let result_b = "LKREBPRK";

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
