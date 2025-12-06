use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Digit(u8);
impl Digit {
    fn to_char(self) -> char {
        char::from(b'0' + self.0)
    }

    fn value(&self) -> i64 {
        i64::from(self.0)
    }
}
fn digit(i: &str) -> IResult<&str, Digit> {
    let (i, n) = map_res(recognize(one_of("0123456789")), FromStr::from_str)(i)?;
    Ok((i, Digit(n)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DigitItem {
    Space,
    Digit(Digit),
}
impl DigitItem {
    fn to_char(self) -> char {
        match self {
            DigitItem::Space => ' ',
            DigitItem::Digit(d) => d.to_char(),
        }
    }
}
impl fmt::Display for DigitItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn digit_item(i: &str) -> IResult<&str, DigitItem> {
    alt((
        map(digit, |d| DigitItem::Digit(d)),
        value(DigitItem::Space, char(' ')),
    ))(i)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}
impl Op {
    fn unit(op: &Op) -> i64 {
        match op {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
    fn apply(&self, a: &mut i64, n: i64) {
        match self {
            Op::Add => *a += n,
            Op::Mul => *a *= n,
        }
    }
    fn to_char(self) -> char {
        match self {
            Op::Add => '+',
            Op::Mul => '*',
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn op(i: &str) -> IResult<&str, Op> {
    alt((value(Op::Add, char('+')), value(Op::Mul, char('*'))))(i)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpItem {
    Space,
    Op(Op),
}
impl OpItem {
    fn to_char(self) -> char {
        match self {
            OpItem::Space => ' ',
            OpItem::Op(d) => d.to_char(),
        }
    }
}
impl fmt::Display for OpItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn op_item(i: &str) -> IResult<&str, OpItem> {
    alt((
        map(op, |op| OpItem::Op(op)),
        value(OpItem::Space, char(' ')),
    ))(i)
}

#[derive(Clone, Debug)]
struct Input {
    digits: Vec<Vec<DigitItem>>,
    ops: Vec<OpItem>,
}
impl Input {
    fn row_numbers(&self) -> Vec<Vec<i64>> {
        let mut result = Vec::new();
        for row in &self.digits {
            let mut row_numbers = Vec::new();
            let mut state = None;
            for item in row {
                state = match (item, state) {
                    (DigitItem::Space, None) => None,
                    (DigitItem::Space, Some(n)) => {
                        row_numbers.push(n);
                        None
                    }
                    (DigitItem::Digit(d), None) => Some(d.value()),
                    (DigitItem::Digit(d), Some(n)) => Some(10 * n + d.value()),
                };
            }
            if let Some(n) = state {
                row_numbers.push(n);
            }
            result.push(row_numbers);
        }
        result
    }
    fn columns(&self) -> Vec<(Vec<i64>, Option<Op>)> {
        let mut result = Vec::new();
        let mut digit_iterators = self.digits.iter().map(|v| v.iter()).collect::<Vec<_>>();
        let mut op_iterator = self.ops.iter();

        loop {
            let mut empty_column = true;

            let mut numbers = Vec::new();
            let mut state = None;
            for it in &mut digit_iterators {
                if let Some(digit_item) = it.next() {
                    empty_column = false;
                    state = match (digit_item, state) {
                        (DigitItem::Space, None) => None,
                        (DigitItem::Space, Some(n)) => {
                            numbers.push(n);
                            None
                        }
                        (DigitItem::Digit(d), None) => Some(d.value()),
                        (DigitItem::Digit(d), Some(n)) => Some(10 * n + d.value()),
                    };
                }
            }
            if let Some(n) = state {
                numbers.push(n);
            }

            let op = {
                if let Some(op_item) = op_iterator.next() {
                    empty_column = false;
                    match op_item {
                        OpItem::Space => None,
                        OpItem::Op(op) => Some(*op),
                    }
                } else {
                    None
                }
            };

            if empty_column {
                break;
            }

            result.push((numbers, op));
        }
        result
    }
    fn ops(&self) -> Vec<Op> {
        let mut result = Vec::new();
        for item in &self.ops {
            if let OpItem::Op(op) = item {
                result.push(*op);
            }
        }
        result
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.digits {
            for item in line {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        for item in &self.ops {
            write!(f, "{}", item)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, numbers) = separated_list1(line_ending, many1(digit_item))(i)?;
    let (i, _) = line_ending(i)?;
    let (i, ops) = many1(op_item)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Input {
            digits: numbers,
            ops,
        },
    ))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut acc = input.ops().iter().map(Op::unit).collect::<Vec<_>>();
    for row in &input.row_numbers() {
        for (op, (a, &n)) in input.ops().iter().zip(acc.iter_mut().zip(row.iter())) {
            op.apply(a, n);
        }
    }
    let result1 = acc.iter().sum::<i64>();

    let mut sum = 0;
    let mut acc = Vec::new();
    for (numbers, op) in input.columns().iter().rev() {
        if numbers.len() == 1 {
            acc.push(numbers[0]);
        }
        if let Some(op) = op {
            sum += {
                match op {
                    Op::Add => acc.iter().sum::<i64>(),
                    Op::Mul => acc.iter().product::<i64>(),
                }
            };
            acc.clear();
        }
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
