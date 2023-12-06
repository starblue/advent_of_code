use core::fmt;
use core::str::FromStr;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Race {
    time: usize,
    dist: usize,
}
impl Race {
    fn win_count(&self) -> usize {
        (1..self.time)
            .map(|t| t * (self.time - t))
            .filter(|&d| d > self.dist)
            .count()
    }
}

#[derive(Clone, Debug)]
struct Input {
    races: Vec<Race>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Time:    ")?;
        let mut sep = "";
        for r in &self.races {
            write!(f, "{sep}{:6}", r.time)?;
            sep = " ";
        }
        writeln!(f)?;
        write!(f, "Distance:")?;
        let mut sep = "";
        for r in &self.races {
            write!(f, "{sep}{:6}", r.dist)?;
            sep = " ";
        }
        writeln!(f)?;
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn numbers(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, uint)(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("Time:")(i)?;
    let (i, _) = space0(i)?;
    let (i, times) = numbers(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Distance:")(i)?;
    let (i, _) = space0(i)?;
    let (i, dists) = numbers(i)?;
    let (i, _) = line_ending(i)?;
    let races = times
        .into_iter()
        .zip(dists)
        .map(|(time, dist)| Race { time, dist })
        .collect::<Vec<_>>();
    Ok((i, Input { races }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input1 = result.1;
    // println!("{}", input1);

    let result1 = input1
        .races
        .iter()
        .map(|r| r.win_count())
        .product::<usize>();

    // parse input again, without spaces
    let input2_data = input_data.chars().filter(|&c| c != ' ').collect::<String>();
    let result = input(&input2_data).map_err(|e| e.to_owned())?;

    let input2 = result.1;
    println!("{}", input2);

    let result2 = input2
        .races
        .iter()
        .map(|r| r.win_count())
        .product::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
