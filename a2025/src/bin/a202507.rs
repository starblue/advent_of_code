use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;

use lowdim::v2d;
use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Splitter,
    Start,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Splitter => '^',
            Square::Start => 'S',
        }
    }
}
impl Square {
    fn is_splitter(&self) -> bool {
        *self == Square::Splitter
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char('.')),
        value(Square::Splitter, char('^')),
        value(Square::Start, char('S')),
    ))(i)
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn start_pos(&self) -> Option<Point2d> {
        self.bbox().iter().find(|&p| self.map[p] == Square::Start)
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn is_splitter(&self, p: Point2d) -> bool {
        if let Some(square) = self.map.get(p) {
            square.is_splitter()
        } else {
            false
        }
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range() {
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
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let down = v2d(0, 1);
    let down_left = v2d(-1, 1);
    let down_right = v2d(1, 1);

    let start_pos = input.start_pos().ok_or("beam start not found")?;

    let mut beams = HashSet::new();
    beams.insert(start_pos);
    let mut count = 0;
    let y_range = (start_pos.y() + 1)..input.bbox().y_end();
    for _y in y_range {
        let mut new_beams = HashSet::new();
        for p in beams {
            if input.is_splitter(p + down) {
                new_beams.insert(p + down_left);
                new_beams.insert(p + down_right);
                count += 1;
            } else {
                // The beam continues straight down.
                new_beams.insert(p + down);
            }
        }
        beams = new_beams;
    }
    let result1 = count;

    let mut beams = HashMap::new();
    beams.insert(start_pos, 1);
    let y_range = (start_pos.y() + 1)..input.bbox().y_end();
    for _y in y_range {
        let mut new_beams = HashMap::new();
        for (p, count) in beams {
            if input.is_splitter(p + down) {
                let e = new_beams.entry(p + down_left).or_insert(0);
                *e += count;
                let e = new_beams.entry(p + down_right).or_insert(0);
                *e += count;
            } else {
                // The beam continues straight down.
                let e = new_beams.entry(p + down).or_insert(0);
                *e += count;
            }
        }
        beams = new_beams;
    }
    let result2 = beams.values().sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
