use core::str::FromStr;

use std::fmt;
use std::fmt::Display;
use std::io;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

#[derive(Clone, Copy, Debug)]
struct IdRange {
    min: i64,
    max: i64,
}
impl IdRange {
    fn contains(&self, id: i64) -> bool {
        self.min <= id && id <= self.max
    }
}
impl Display for IdRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

fn id_range(i: &str) -> IResult<&str, IdRange> {
    let (i, min) = uint(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = uint(i)?;
    Ok((i, IdRange { min, max }))
}

#[derive(Clone, Debug)]
struct Input {
    fresh_ranges: Vec<IdRange>,
    ingredients: Vec<i64>,
}
impl Input {
    fn is_fresh(&self, id: i64) -> bool {
        for r in &self.fresh_ranges {
            if r.contains(id) {
                return true;
            }
        }
        false
    }
}
impl Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.fresh_ranges {
            writeln!(f, "{}", r)?;
        }
        writeln!(f)?;
        for i in &self.ingredients {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, fresh_ranges) = separated_list1(line_ending, id_range)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, ingredients) = separated_list1(line_ending, uint)(i)?;
    Ok((
        i,
        Input {
            fresh_ranges,
            ingredients,
        },
    ))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input
        .ingredients
        .iter()
        .filter(|&&id| input.is_fresh(id))
        .count();

    let mut fresh_ranges = input.fresh_ranges;
    fresh_ranges.sort_by(|r0, r1| r0.min.cmp(&r1.min).then(r0.max.cmp(&r1.max)));
    let mut count = 0;
    let mut min = 0;
    let mut max = -1;
    for r in fresh_ranges {
        if r.min > max + 1 {
            // Count the previous merged range.
            count += max - min + 1;

            // Start a new merged range.
            min = r.min;
            max = r.max;
        } else {
            // Extend the merged range, if necessary.
            max = max.max(r.max);
        }
    }
    // Count the last merged range.
    count += max - min + 1;
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
