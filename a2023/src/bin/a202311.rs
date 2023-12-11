use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Galaxy,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Galaxy => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn galaxy_positions(&self) -> Vec<Point2d> {
        self.map
            .bbox()
            .iter()
            .filter(|&p| self.map[p] == Square::Galaxy)
            .collect::<Vec<Point2d>>()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.map.bbox().y_range() {
            for x in self.map.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char('.')),
        value(Square::Galaxy, char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    let (i, line) = many1(square)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, lines) = many1(line)(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn expand_universe(positions: &[Point2d], factor: i64) -> Vec<Point2d> {
    let x_values = positions.iter().map(|p| p.x()).collect::<HashSet<_>>();
    let y_values = positions.iter().map(|p| p.y()).collect::<HashSet<_>>();

    let bbox = BBox::enclosing(positions).unwrap();

    let mut new_x = 0;
    let mut x_map = HashMap::new();
    for x in bbox.x_range() {
        x_map.insert(x, new_x);
        new_x += if x_values.contains(&x) { 1 } else { factor };
    }
    let mut new_y = 0;
    let mut y_map = HashMap::new();
    for y in bbox.y_range() {
        y_map.insert(y, new_y);
        new_y += if y_values.contains(&y) { 1 } else { factor };
    }
    positions
        .iter()
        .map(|p| p2d(x_map[&p.x()], y_map[&p.y()]))
        .collect::<Vec<_>>()
}

fn distance_sum(positions: &[Point2d]) -> i64 {
    let mut sum = 0;
    for i in 0..(positions.len() - 1) {
        let pi = positions[i];
        for j in (i + 1)..positions.len() {
            let pj = positions[j];
            sum += pi.distance_l1(pj);
        }
    }
    sum
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let positions = input.galaxy_positions();

    let new_positions = expand_universe(&positions, 2);
    let result1 = distance_sum(&new_positions);

    let new_positions = expand_universe(&positions, 1_000_000);
    let result2 = distance_sum(&new_positions);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
