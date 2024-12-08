use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    Antenna(char),
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Antenna(c) => c,
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn square_empty(i: &str) -> IResult<&str, Square> {
    value(Square::Empty, char('.'))(i)
}
fn square_antenna(i: &str) -> IResult<&str, Square> {
    let (i, c) = satisfy(|c| c.is_ascii_alphanumeric())(i)?;
    Ok((i, Square::Antenna(c)))
}
fn square(i: &str) -> IResult<&str, Square> {
    alt((square_empty, square_antenna))(i)
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range().rev() {
            for x in self.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, mut lines) = separated_list1(line_ending, many1(square))(i)?;

    // The y coordinate increases from the bottom, i.e. here from the end.
    lines.reverse();

    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let bbox = input.bbox();

    let mut frequencies = HashMap::new();
    for p in bbox {
        if let Square::Antenna(c) = input.map[p] {
            let e = frequencies.entry(c).or_insert_with(Vec::new);
            e.push(p);
        }
    }

    let mut antinodes1 = HashSet::new();
    for antennas in frequencies.values() {
        let len = antennas.len();
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                let pi = antennas[i];
                let pj = antennas[j];
                let d = pj - pi;
                let pn0 = pi - d;
                let pn1 = pj + d;
                if bbox.contains(&pn0) {
                    antinodes1.insert(pn0);
                }
                if bbox.contains(&pn1) {
                    antinodes1.insert(pn1);
                }
            }
        }
    }
    let result1 = antinodes1.len();

    let mut antinodes2 = HashSet::new();
    for antennas in frequencies.values() {
        let len = antennas.len();
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                let pi = antennas[i];
                let pj = antennas[j];
                let d = pj - pi;

                let mut pn = pi;
                while bbox.contains(&pn) {
                    antinodes2.insert(pn);
                    pn -= d;
                }

                let mut pn = pj;
                while bbox.contains(&pn) {
                    antinodes2.insert(pn);
                    pn += d;
                }
            }
        }
    }
    let result2 = antinodes2.len();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
