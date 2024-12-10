use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Square {
    height: u8,
}
impl Square {
    fn to_char(self) -> char {
        char::from_digit(u32::from(self.height), 10).unwrap_or('.')
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    let (i, c) = satisfy(|c| c.is_ascii_digit())(i)?;
    let height = u8::try_from(c.to_digit(10).unwrap()).unwrap();
    Ok((i, Square { height }))
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

    let mut score = 0;
    for p in bbox {
        if input.map[p].height == 0 {
            let mut positions = HashSet::new();
            positions.insert(p);
            for height in 1..=9 {
                let mut new_positions = HashSet::new();
                for p0 in positions {
                    for p1 in p0.neighbors_l1() {
                        if bbox.contains(&p1) && input.map[p1].height == height {
                            new_positions.insert(p1);
                        }
                    }
                }
                positions = new_positions;
            }
            score += positions.len();
        }
    }
    let result1 = score;

    let mut rating = 0;
    for p in bbox {
        if input.map[p].height == 0 {
            let mut position_counts = HashMap::new();
            position_counts.insert(p, 1);
            for height in 1..=9 {
                let mut new_position_counts = HashMap::new();
                for (p0, count) in position_counts {
                    for p1 in p0.neighbors_l1() {
                        if bbox.contains(&p1) && input.map[p1].height == height {
                            let entry = new_position_counts.entry(p1).or_insert(0);
                            *entry += count;
                        }
                    }
                }
                position_counts = new_position_counts;
            }
            rating += position_counts.values().sum::<i64>();
        }
    }
    let result2 = rating;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
