use core::fmt;

use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Towel(String);
impl fmt::Display for Towel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn towel(i: &str) -> IResult<&str, Towel> {
    let (i, cs) = recognize(many1(satisfy(|c| c.is_ascii_alphabetic())))(i)?;
    Ok((i, Towel(cs.to_string())))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Design(String);
impl fmt::Display for Design {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn design(i: &str) -> IResult<&str, Design> {
    let (i, cs) = recognize(many1(satisfy(|c| c.is_ascii_alphabetic())))(i)?;
    Ok((i, Design(cs.to_string())))
}

#[derive(Clone, Debug)]
struct Input {
    towels: Vec<Towel>,
    designs: Vec<Design>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for t in &self.towels {
            write!(f, "{}{}", sep, t)?;
            sep = ", ";
        }
        writeln!(f)?;
        for d in &self.designs {
            writeln!(f, "{}", d)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, towels) = separated_list1(tag(", "), towel)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, designs) = separated_list1(line_ending, design)(i)?;
    Ok((i, Input { towels, designs }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut count = 0;
    for design in &input.designs {
        let len = design.0.len();
        let mut stack = vec![0];
        let mut seen = HashSet::new();
        while let Some(index) = stack.pop() {
            if !seen.contains(&index) {
                seen.insert(index);
                for towel in &input.towels {
                    if design.0[index..].starts_with(&towel.0) {
                        stack.push(index + towel.0.len());
                    }
                }
            }
        }
        if seen.contains(&len) {
            count += 1;
        }
    }
    let result1 = count;

    let mut sum = 0;
    for design in &input.designs {
        let len = design.0.len();
        let mut counts = (0..=len)
            .map(|i| if i == 0 { 1_i64 } else { 0 })
            .collect::<Vec<_>>();
        for i in 0..=len {
            for towel in &input.towels {
                if i >= towel.0.len() {
                    let j = i - towel.0.len();
                    if design.0[j..i] == towel.0 {
                        counts[i] += counts[j];
                    }
                }
            }
        }
        sum += counts[len];
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
