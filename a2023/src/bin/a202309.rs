use core::fmt;
use core::str::FromStr;

use std::io;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug)]
struct History(Vec<i64>);
impl History {
    fn extrapolate_forward(&self) -> i64 {
        let mut last_ns = Vec::new();
        let mut h = self.clone();
        while !h.all_zero() {
            last_ns.push(h.last());
            h = h.differences();
        }
        let mut result = 0;
        while let Some(n) = last_ns.pop() {
            result += n;
        }
        result
    }
    fn extrapolate_backward(&self) -> i64 {
        let mut first_ns = Vec::new();
        let mut h = self.clone();
        while !h.all_zero() {
            first_ns.push(h.first());
            h = h.differences();
        }
        let mut result = 0;
        while let Some(n) = first_ns.pop() {
            result = n - result;
        }
        result
    }
    fn differences(&self) -> History {
        History(self.0.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>())
    }
    fn all_zero(&self) -> bool {
        self.0.iter().all(|&n| n == 0)
    }
    fn first(&self) -> i64 {
        *self.0.first().unwrap()
    }
    fn last(&self) -> i64 {
        *self.0.last().unwrap()
    }
}
impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut sep = "";
        for n in &self.0 {
            write!(f, "{sep}{n}")?;
            sep = " ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    histories: Vec<History>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for h in &self.histories {
            writeln!(f, "{}", h)?;
        }
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn history(i: &str) -> IResult<&str, History> {
    let (i, ns) = separated_list1(space1, int)(i)?;
    Ok((i, History(ns)))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, histories) = separated_list1(line_ending, history)(i)?;
    Ok((i, Input { histories }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input
        .histories
        .iter()
        .map(|h| h.extrapolate_forward())
        .sum::<i64>();

    let result2 = input
        .histories
        .iter()
        .map(|h| h.extrapolate_backward())
        .sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::History;

    #[test]
    fn test_extrapolate_forward_1() {
        let history = History(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(18, history.extrapolate_forward());
    }
    #[test]
    fn test_extrapolate_forward_2() {
        let history = History(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(28, history.extrapolate_forward());
    }
    #[test]
    fn test_extrapolate_forward_3() {
        let history = History(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(68, history.extrapolate_forward());
    }

    #[test]
    fn test_extrapolate_backward() {
        let history = History(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(5, history.extrapolate_backward());
    }
}
