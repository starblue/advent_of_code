use core::fmt;
use core::str::FromStr;

use std::collections::VecDeque;
use std::error;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::IResult;

use num::Integer;

#[derive(Clone, Debug)]
enum Operation {
    Add(i64),
    Mul(i64),
    Square,
}
impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Add(n) => old + n,
            Operation::Mul(n) => old * n,
            Operation::Square => old * old,
        }
    }
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operation::Add(n) => write!(f, "old + {}", n),
            Operation::Mul(n) => write!(f, "old * {}", n),
            Operation::Square => write!(f, "old * old"),
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<i64>,
    operation: Operation,
    test_divisor: i64,
    throw_if_true: usize,
    throw_if_false: usize,
}
impl Monkey {
    fn push_item(&mut self, item: i64) {
        self.items.push_back(item)
    }
    fn pop_item(&mut self) -> Option<i64> {
        self.items.pop_front()
    }
}
impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Monkey {}:", self.id)?;
        write!(f, "  Starting items: ")?;
        let mut sep = "";
        for item in &self.items {
            write!(f, "{}{}", sep, item)?;
            sep = ", ";
        }
        writeln!(f)?;
        writeln!(f, "  Operation: new = {}", self.operation)?;
        writeln!(f, "  Test: divisible by {}", self.test_divisor)?;
        writeln!(f, "    If true: throw to monkey {}", self.throw_if_true)?;
        writeln!(f, "    If false: throw to monkey {}", self.throw_if_false)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    monkeys: Vec<Monkey>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for monkey in &self.monkeys {
            writeln!(f, "{}", monkey)?;
        }
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn monkey_id(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}

fn operation_add(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("old + ")(i)?;
    let (i, n) = int(i)?;
    Ok((i, Operation::Add(n)))
}
fn operation_mul(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("old * ")(i)?;
    let (i, n) = int(i)?;
    Ok((i, Operation::Mul(n)))
}
fn operation(i: &str) -> IResult<&str, Operation> {
    alt((
        operation_add,
        operation_mul,
        value(Operation::Square, tag("old * old")),
    ))(i)
}

fn monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tag("Monkey ")(i)?;
    let (i, id) = monkey_id(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("  Starting items: ")(i)?;
    let (i, starting_items) = separated_list0(tag(", "), int)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("  Operation: new = ")(i)?;
    let (i, operation) = operation(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("  Test: divisible by ")(i)?;
    let (i, test_divisor) = int(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("    If true: throw to monkey ")(i)?;
    let (i, throw_if_true) = monkey_id(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("    If false: throw to monkey ")(i)?;
    let (i, throw_if_false) = monkey_id(i)?;
    let (i, _) = line_ending(i)?;

    let items = starting_items.into();
    Ok((
        i,
        Monkey {
            id,
            items,
            operation,
            test_divisor,
            throw_if_true,
            throw_if_false,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, monkeys) = separated_list0(line_ending, monkey)(i)?;
    Ok((i, Input { monkeys }))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut monkeys = input.monkeys.clone();

    // Make sure we can use indices and ids interchangably.
    for i in 0..monkeys.len() {
        assert_eq!(i, monkeys[i].id);
    }

    let mut inspection_counts = monkeys.iter().map(|_| 0_usize).collect::<Vec<_>>();
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].pop_item() {
                inspection_counts[i] += 1;
                let worry_level = monkeys[i].operation.apply(item) / 3;
                let throw = if worry_level % monkeys[i].test_divisor == 0 {
                    monkeys[i].throw_if_true
                } else {
                    monkeys[i].throw_if_false
                };
                monkeys[throw].push_item(worry_level);
            }
        }
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    let result1 = inspection_counts[0] * inspection_counts[1];

    let mut monkeys = input.monkeys;
    let modulus = monkeys.iter().fold(1_i64, |a, m| a.lcm(&m.test_divisor));
    println!("modulus {}", modulus);
    let mut inspection_counts = monkeys.iter().map(|_| 0_usize).collect::<Vec<_>>();
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].pop_item() {
                inspection_counts[i] += 1;
                let worry_level = monkeys[i].operation.apply(item) % modulus;
                let throw = if worry_level % monkeys[i].test_divisor == 0 {
                    monkeys[i].throw_if_true
                } else {
                    monkeys[i].throw_if_false
                };
                monkeys[throw].push_item(worry_level);
            }
        }
    }
    inspection_counts.sort();
    inspection_counts.reverse();
    let result2 = inspection_counts[0] * inspection_counts[1];

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
