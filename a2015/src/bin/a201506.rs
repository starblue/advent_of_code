use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
impl Action {
    fn apply_1(&self, b: &mut bool) {
        match self {
            Action::TurnOn => {
                *b = true;
            }
            Action::TurnOff => {
                *b = false;
            }
            Action::Toggle => {
                *b = !*b;
            }
        }
    }
    fn apply_2(&self, b: &mut i64) {
        match self {
            Action::TurnOn => {
                *b += 1;
            }
            Action::TurnOff => {
                *b = (*b - 1).max(0);
            }
            Action::Toggle => {
                *b += 2;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    action: Action,
    bbox: BBox2d,
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn action(i: &str) -> IResult<&str, Action> {
    let p0 = value(Action::TurnOn, tag("turn on"));
    let p1 = value(Action::TurnOff, tag("turn off"));
    let p2 = value(Action::Toggle, tag("toggle"));
    alt((p0, p1, p2))(i)
}

fn point(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = uint(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = uint(i)?;
    Ok((i, p2d(x, y)))
}

fn bbox(i: &str) -> IResult<&str, BBox2d> {
    let (i, p0) = point(i)?;
    let (i, _) = tag(" through ")(i)?;
    let (i, p1) = point(i)?;
    Ok((i, BBox2d::from_points(p0, p1)))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, action) = action(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, bbox) = bbox(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction { action, bbox }))
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

    let bbox = BBox2d::from_points(p2d(0, 0), p2d(999, 999));
    let mut lights = Array2d::new(bbox, false);
    for &instruction in &input {
        for p in instruction.bbox.iter() {
            instruction.action.apply_1(&mut lights[p]);
        }
    }
    let result_a = lights.iter().filter(|&&b| b).count();

    let bbox = BBox2d::from_points(p2d(0, 0), p2d(999, 999));
    let mut lights = Array2d::new(bbox, 0);
    for &instruction in &input {
        for p in instruction.bbox.iter() {
            instruction.action.apply_2(&mut lights[p]);
        }
    }
    let result_b = lights.iter().sum::<i64>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
