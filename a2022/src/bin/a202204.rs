use core::fmt;
use core::str::FromStr;

use std::io;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Range {
    min: i64,
    max: i64,
}
impl Range {
    fn is_superset(&self, other: &Range) -> bool {
        self.min <= other.min && other.max <= self.max
    }
    fn intersects(&self, other: &Range) -> bool {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        min <= max
    }
}
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    range0: Range,
    range1: Range,
}
impl Pair {
    fn has_superset(&self) -> bool {
        self.range0.is_superset(&self.range1) || self.range1.is_superset(&self.range0)
    }
    fn has_overlap(&self) -> bool {
        self.range0.intersects(&self.range1)
    }
}
impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.range0, self.range1)
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn range(i: &str) -> IResult<&str, Range> {
    let (i, min) = int(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = int(i)?;
    Ok((i, Range { min, max }))
}

fn pair(i: &str) -> IResult<&str, Pair> {
    let (i, range0) = range(i)?;
    let (i, _) = char(',')(i)?;
    let (i, range1) = range(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Pair { range0, range1 }))
}

fn input(i: &str) -> IResult<&str, Vec<Pair>> {
    many1(pair)(i)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for pair in &input {
    //     println!("{}", pair);
    // }

    let result1 = input.iter().filter(|&&p| p.has_superset()).count();

    let result2 = input.iter().filter(|&&p| p.has_overlap()).count();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
