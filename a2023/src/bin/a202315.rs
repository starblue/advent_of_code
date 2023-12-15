use core::fmt;
use core::str::FromStr;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

fn hash(s: &str) -> usize {
    let h = s.chars().fold(0, |a, c| (a + u32::from(c)) * 17 % 256);
    usize::try_from(h).unwrap()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Label(String);
impl Label {
    fn hash(&self) -> usize {
        hash(&self.0)
    }
}
impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Delete,
    Insert(u8),
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Delete => write!(f, "-"),
            Operation::Insert(n) => write!(f, "={}", n),
        }
    }
}

#[derive(Clone, Debug)]
struct Step {
    label: Label,
    operation: Operation,
}
impl Step {
    fn hash(&self) -> usize {
        hash(&self.to_string())
    }
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.label, self.operation)
    }
}

#[derive(Clone, Debug)]
struct Input {
    steps: Vec<Step>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for step in &self.steps {
            write!(f, "{}{}", sep, step)?;
            sep = ",";
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, u8> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn label(i: &str) -> IResult<&str, Label> {
    let (i, s) = alpha1(i)?;
    Ok((i, Label(s.to_string())))
}

fn operation_delete(i: &str) -> IResult<&str, Operation> {
    value(Operation::Delete, tag("-"))(i)
}
fn operation_insert(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("=")(i)?;
    let (i, n) = uint(i)?;
    Ok((i, Operation::Insert(n)))
}
fn operation(i: &str) -> IResult<&str, Operation> {
    alt((operation_insert, operation_delete))(i)
}

fn step(i: &str) -> IResult<&str, Step> {
    let (i, label) = label(i)?;
    let (i, operation) = operation(i)?;
    Ok((i, Step { label, operation }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, steps) = separated_list1(tag(","), step)(i)?;
    Ok((i, Input { steps }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input.steps.iter().map(|step| step.hash()).sum::<usize>();

    let mut boxes = (0..256)
        .map(|_| Vec::<(Label, u8)>::new())
        .collect::<Vec<_>>();
    for Step { label, operation } in &input.steps {
        let h = label.hash();
        let b = &mut boxes[h];
        match operation {
            Operation::Delete => {
                if let Some(i) = b.iter().position(|(l1, _)| l1 == label) {
                    b.remove(i);
                }
            }
            Operation::Insert(n) => {
                if let Some(i) = b.iter().position(|(l1, _)| l1 == label) {
                    b[i] = (label.clone(), *n);
                } else {
                    b.push((label.clone(), *n));
                }
            }
        }
    }
    let result2 = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(j, &(_, n))| (i + 1) * (j + 1) * usize::from(n))
        })
        .sum::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
