use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::named;
use nom::value;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Vec2d;

named!(action<&str, Vec2d>,
    alt!(
        value!(v2d(0, 1), char!('^')) |
        value!(v2d(0, -1), char!('v')) |
        value!(v2d(1, 0), char!('>')) |
        value!(v2d(-1, 0), char!('<'))
    )
);

named!(input<&str, Vec<Vec2d>>,
    do_parse!(
        actions: many1!(action) >> line_ending >> (actions)
    )
);

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
