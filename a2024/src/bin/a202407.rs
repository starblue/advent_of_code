use core::fmt;
use core::str::FromStr;

use std::io;

use nom::bytes::complete::tag;
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

#[derive(Clone, Debug)]
struct State<'a> {
    value: i64,
    rest: &'a [i64],
}
impl<'a> State<'a> {
    fn init(ns: &'a [i64]) -> State<'a> {
        let value = ns[0];
        let rest = &ns[1..];
        State { value, rest }
    }
}

fn concat(a: i64, b: i64) -> i64 {
    let len_b = b.ilog10() + 1;
    let p10 = 10_i64.pow(len_b);
    a * p10 + b
}

#[derive(Clone, Debug)]
struct Equation {
    left: i64,
    right: Vec<i64>,
}
impl Equation {
    fn is_solvable1(&self) -> bool {
        let init_state = State::init(&self.right);
        let mut stack = vec![init_state];
        while let Some(state) = stack.pop() {
            if state.rest.is_empty() {
                if state.value == self.left {
                    // We found a solution.
                    return true;
                }
            } else {
                let first = state.rest[0];
                let rest = &state.rest[1..];
                stack.push(State {
                    value: state.value + first,
                    rest,
                });
                stack.push(State {
                    value: state.value * first,
                    rest,
                });
            }
        }
        false
    }
    fn is_solvable2(&self) -> bool {
        let init_state = State::init(&self.right);
        let mut stack = vec![init_state];
        while let Some(state) = stack.pop() {
            if state.rest.is_empty() {
                if state.value == self.left {
                    // We found a solution.
                    return true;
                }
            } else {
                let first = state.rest[0];
                let rest = &state.rest[1..];
                stack.push(State {
                    value: state.value + first,
                    rest,
                });
                stack.push(State {
                    value: state.value * first,
                    rest,
                });
                stack.push(State {
                    value: concat(state.value, first),
                    rest,
                });
            }
        }
        false
    }
}
impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.left)?;
        let mut sep = "";
        for n in &self.right {
            write!(f, "{}{}", sep, n)?;
            sep = " ";
        }
        Ok(())
    }
}
fn equation(i: &str) -> IResult<&str, Equation> {
    let (i, left) = uint(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, right) = separated_list1(tag(" "), uint)(i)?;
    Ok((i, Equation { left, right }))
}

fn input(i: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(line_ending, equation)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for equation in &input {
    //     println!("{}", equation);
    // }

    let result1 = input
        .iter()
        .filter(|e| e.is_solvable1())
        .map(|e| e.left)
        .sum::<i64>();

    let result2 = input
        .iter()
        .filter(|e| e.is_solvable2())
        .map(|e| e.left)
        .sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(12345, concat(12, 345));
    }
}
