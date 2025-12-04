use std::collections::HashSet;
use std::fmt;
use std::io;

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
    Paper,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Paper => '@',
        }
    }
}
impl Square {
    fn is_paper(&self) -> bool {
        *self == Square::Paper
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
        value(Square::Paper, char('@')),
    ))(i)
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn is_paper(&self, p: Point2d) -> bool {
        if let Some(square) = self.map.get(p) {
            square.is_paper()
        } else {
            false
        }
    }
    fn is_accessible(&self, p: Point2d) -> bool {
        let mut count = 0;
        for np in p.neighbors_l_infty() {
            if let Some(square) = self.map.get(np) {
                if square.is_paper() {
                    count += 1;
                }
            }
        }
        count < 4
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

    let result1 = input
        .bbox()
        .iter()
        .filter(|&p| input.is_paper(p) && input.is_accessible(p))
        .count();

    let mut input = input;
    let mut count = 0;
    let mut candidates = input
        .bbox()
        .iter()
        .filter(|&p| input.is_paper(p))
        .collect::<HashSet<_>>();
    loop {
        let mut new_candidates = HashSet::new();
        let mut changed = false;
        for &p in &candidates {
            if input.is_paper(p) {
                if input.is_accessible(p) {
                    // Remove candidate.
                    input.map[p] = Square::Empty;
                    count += 1;
                    changed = true;

                    // Add neighboring paper rolls as candidates.
                    for np in p.neighbors_l_infty() {
                        if input.is_paper(np) {
                            new_candidates.insert(np);
                        }
                    }
                } else {
                    // Retry later, maybe it becomes accessible.
                    new_candidates.insert(p);
                }
            }
        }
        if !changed {
            break;
        }
        candidates = new_candidates;
    }
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
