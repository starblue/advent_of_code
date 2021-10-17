use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum Move {
    U,
    D,
    L,
    R,
}
impl Move {
    fn to_vec2d(&self) -> Vec2d {
        match self {
            Move::U => v2d(0, 1),
            Move::D => v2d(0, -1),
            Move::L => v2d(-1, 0),
            Move::R => v2d(1, 0),
        }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Move::U => write!(f, "U"),
            Move::D => write!(f, "D"),
            Move::L => write!(f, "L"),
            Move::R => write!(f, "R"),
        }
    }
}

fn move_(i: &str) -> IResult<&str, Move> {
    alt((
        value(Move::U, tag("U")),
        value(Move::D, tag("D")),
        value(Move::L, tag("L")),
        value(Move::R, tag("R")),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Move>> {
    let (i, moves) = many1(move_)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, moves))
}

fn input(i: &str) -> IResult<&str, Vec<Vec<Move>>> {
    many1(line)(i)
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

    let input = result.unwrap().1;
    // for line in &input {
    //     for m in line {
    //         print!("{}", m);
    //     }
    //     println!();
    // }
    // println!();

    let mut keypad1 = HashMap::new();
    keypad1.insert(p2d(0, 2), '1');
    keypad1.insert(p2d(1, 2), '2');
    keypad1.insert(p2d(2, 2), '3');
    keypad1.insert(p2d(0, 1), '4');
    keypad1.insert(p2d(1, 1), '5');
    keypad1.insert(p2d(2, 1), '6');
    keypad1.insert(p2d(0, 0), '7');
    keypad1.insert(p2d(1, 0), '8');
    keypad1.insert(p2d(2, 0), '9');
    let mut code = String::new();
    for line in &input {
        let mut pos = p2d(1, 1);
        for m in line {
            let new_pos = pos + m.to_vec2d();
            if keypad1.contains_key(&new_pos) {
                pos = new_pos;
            }
        }
        let digit = keypad1[&pos];
        code.push(digit);
    }
    let result_a = code;

    let mut keypad2 = HashMap::new();
    keypad2.insert(p2d(2, 4), '1');
    keypad2.insert(p2d(1, 3), '2');
    keypad2.insert(p2d(2, 3), '3');
    keypad2.insert(p2d(3, 3), '4');
    keypad2.insert(p2d(0, 2), '5');
    keypad2.insert(p2d(1, 2), '6');
    keypad2.insert(p2d(2, 2), '7');
    keypad2.insert(p2d(3, 2), '8');
    keypad2.insert(p2d(4, 2), '9');
    keypad2.insert(p2d(1, 1), 'A');
    keypad2.insert(p2d(2, 1), 'B');
    keypad2.insert(p2d(3, 1), 'C');
    keypad2.insert(p2d(2, 0), 'D');
    let mut code = String::new();
    for line in &input {
        let mut pos = p2d(0, 2);
        for m in line {
            let new_pos = pos + m.to_vec2d();
            if keypad2.contains_key(&new_pos) {
                pos = new_pos;
            }
        }
        let digit = keypad2[&pos];
        code.push(digit);
    }
    let result_b = code;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
