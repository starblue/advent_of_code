use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Vec2d;

fn action(i: &str) -> IResult<&str, Vec2d> {
    let p0 = value(v2d(0, 1), char('^'));
    let p1 = value(v2d(0, -1), char('v'));
    let p2 = value(v2d(1, 0), char('>'));
    let p3 = value(v2d(-1, 0), char('<'));

    alt((p0, p1, p2, p3))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vec2d>> {
    let (i, actions) = many1(action)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, actions))
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

    let mut p = p2d(0, 0);
    let mut visited = HashSet::new();
    visited.insert(p);
    for v in &input {
        p += v;
        visited.insert(p);
    }
    let result_a = visited.len();

    let mut sp = p2d(0, 0);
    let mut rp = p2d(0, 0);
    let mut visited = HashSet::new();
    visited.insert(sp);
    let mut santa_turn = true;
    for v in &input {
        if santa_turn {
            sp += v;
            visited.insert(sp);
        } else {
            rp += v;
            visited.insert(rp);
        }
        santa_turn = !santa_turn;
    }
    let result_b = visited.len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
