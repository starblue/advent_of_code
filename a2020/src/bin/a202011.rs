use std::fmt;
use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::do_parse;
use nom::character::complete::line_ending;
use nom::many1;
use nom::named;
use nom::value;

use gamedim::v2d;
use gamedim::Array2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Floor,
    Seat(bool),
}
impl Square {
    fn to_char(&self) -> char {
        match self {
            Square::Floor => '.',
            Square::Seat(false) => 'L',
            Square::Seat(true) => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

named!(square<&str, Square>,
    alt!(
        value!(Square::Floor, char!('.')) |
        value!(Square::Seat(false), char!('L')) |
        value!(Square::Seat(true), char!('#'))
    )
);

named!(
    line<&str, Vec<Square>>,
    many1!(square)
);

named!(
    lines<&str, Vec<Vec<Square>>>,
    many1!(
        do_parse!(
            line: line >>
            line_ending >> (line)
        )
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
    let result = lines(&input_data);
    //println!("{:?}", result);

    let m = Array2d::<i32, Square>::from_vec(result.unwrap().1);

    let mut old_map = m.clone();
    let mut dirty = true;
    while dirty {
        dirty = false;

        let mut new_map = old_map.clone();
        for p in m.bounds().iter() {
            if let Square::Seat(occupied) = old_map[p] {
                let c = vec![
                    v2d(1, 0),
                    v2d(1, 1),
                    v2d(0, 1),
                    v2d(-1, 1),
                    v2d(-1, 0),
                    v2d(-1, -1),
                    v2d(0, -1),
                    v2d(1, -1),
                ]
                .iter()
                .map(|v| p + v)
                .filter(|&np| m.bounds().contains(np) && old_map[np] == Square::Seat(true))
                .count();
                if !occupied && c == 0 {
                    new_map[p] = Square::Seat(true);
                    dirty = true;
                } else if occupied && c >= 4 {
                    new_map[p] = Square::Seat(false);
                    dirty = true;
                } else {
                    // don't change seat state
                }
            }
        }
        old_map = new_map;
    }
    let result_a = old_map
        .bounds()
        .iter()
        .filter(|&np| old_map[np] == Square::Seat(true))
        .count();

    let mut old_map = m.clone();
    let mut dirty = true;
    while dirty {
        dirty = false;

        let mut new_map = old_map.clone();
        for p in m.bounds().iter() {
            if let Square::Seat(occupied) = old_map[p] {
                let mut count = 0;
                for v in &[
                    v2d(1, 0),
                    v2d(1, 1),
                    v2d(0, 1),
                    v2d(-1, 1),
                    v2d(-1, 0),
                    v2d(-1, -1),
                    v2d(0, -1),
                    v2d(1, -1),
                ] {
                    let mut np = p + v;
                    while m.bounds().contains(np) && old_map[np] == Square::Floor {
                        np += v;
                    }
                    if m.bounds().contains(np) && old_map[np] == Square::Seat(true) {
                        count += 1;
                    }
                }
                if !occupied && count == 0 {
                    new_map[p] = Square::Seat(true);
                    dirty = true;
                } else if occupied && count >= 5 {
                    new_map[p] = Square::Seat(false);
                    dirty = true;
                } else {
                    // don't change seat state
                }
            }
        }
        old_map = new_map;
    }
    let result_b = old_map
        .bounds()
        .iter()
        .filter(|&np| old_map[np] == Square::Seat(true))
        .count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
