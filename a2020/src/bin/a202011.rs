use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::Array2d;
use lowdim::Vec2d;
use lowdim::Vector;

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

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Floor, char('.')),
        value(Square::Seat(false), char('L')),
        value(Square::Seat(true), char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    let (i, line) = many1(square)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn lines(i: &str) -> IResult<&str, Vec<Vec<Square>>> {
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
    let result = lines(&input_data);
    //println!("{:?}", result);

    let m = Array2d::<i32, Square>::from_vec(result.unwrap().1);

    let mut old_map = m.clone();
    let mut dirty = true;
    while dirty {
        dirty = false;

        let mut new_map = old_map.clone();
        for p in m.bbox().iter() {
            if let Square::Seat(occupied) = old_map[p] {
                let c = p
                    .neighbors_l_infty()
                    .into_iter()
                    .filter(|&np| old_map.get(np) == Some(&Square::Seat(true)))
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
        .iter()
        .filter(|&&square| square == Square::Seat(true))
        .count();

    let mut old_map = m.clone();
    let mut dirty = true;
    while dirty {
        dirty = false;

        let mut new_map = old_map.clone();
        for p in m.bbox().iter() {
            if let Square::Seat(occupied) = old_map[p] {
                let mut count = 0;
                for v in &Vec2d::unit_vecs_l_infty() {
                    let mut np = p + v;
                    while old_map.get(np) == Some(&Square::Floor) {
                        np += v;
                    }
                    if old_map.get(np) == Some(&Square::Seat(true)) {
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
        .iter()
        .filter(|&&square| square == Square::Seat(true))
        .count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
