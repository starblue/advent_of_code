use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use util::runtime_error;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
    fn apply_inv_a(&self, result: i64, b: i64) -> i64 {
        match self {
            Op::Add => result - b,
            Op::Sub => result + b,
            Op::Mul => result / b,
            Op::Div => result * b,
        }
    }
    fn apply_inv_b(&self, result: i64, a: i64) -> i64 {
        match self {
            Op::Add => result - a,
            Op::Sub => a - result,
            Op::Mul => result / a,
            Op::Div => a / result,
        }
    }
    fn to_char(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Operation {
    op: Op,
    a: String,
    b: String,
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.a, self.op, self.b)
    }
}

#[derive(Clone, Debug)]
enum Job {
    Const(i64),
    Op(Operation),
}
impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Job::Const(v) => write!(f, "{}", v),
            Job::Op(operation) => write!(f, "{}", operation),
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    name: String,
    job: Job,
}
impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.job)
    }
}

fn monkey_name(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn op(i: &str) -> IResult<&str, Op> {
    alt((
        value(Op::Add, char('+')),
        value(Op::Sub, char('-')),
        value(Op::Mul, char('*')),
        value(Op::Div, char('/')),
    ))(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    let (i, a) = monkey_name(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, op) = op(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, b) = monkey_name(i)?;
    Ok((i, Operation { op, a, b }))
}

fn job_const(i: &str) -> IResult<&str, Job> {
    let (i, n) = int(i)?;
    Ok((i, Job::Const(n)))
}
fn job_op(i: &str) -> IResult<&str, Job> {
    let (i, op) = operation(i)?;
    Ok((i, Job::Op(op)))
}
fn job(i: &str) -> IResult<&str, Job> {
    alt((job_const, job_op))(i)
}

fn monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, name) = monkey_name(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, job) = job(i)?;
    Ok((i, Monkey { name, job }))
}

fn input(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(line_ending, monkey)(i)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for monkey in &input {
    //     println!("{}", monkey);
    // }

    let monkeys = input
        .iter()
        .map(|m| (m.name.clone(), m))
        .collect::<HashMap<_, _>>();
    let mut values: HashMap<String, i64> = HashMap::new();
    let mut unresolved = input
        .iter()
        .map(|m| m.name.clone())
        .collect::<VecDeque<_>>();
    while let Some(n) = unresolved.pop_front() {
        let monkey = &monkeys[&n];
        match &monkey.job {
            Job::Const(vn) => {
                values.insert(n.clone(), *vn);
            }
            Job::Op(Operation { op, a, b }) => match (values.get(a), values.get(b)) {
                (Some(&va), Some(&vb)) => {
                    let vn = op.apply(va, vb);
                    values.insert(n.clone(), vn);
                }
                _ => unresolved.push_back(n),
            },
        }
    }
    let result1 = values
        .get("root")
        .ok_or(runtime_error!("value for root not known"))?;

    let mut monkeys: HashMap<String, Monkey> = input
        .iter()
        .map(|m| (m.name.clone(), m.clone()))
        .collect::<HashMap<_, _>>();
    if let Some(root) = monkeys.get_mut("root") {
        if let Job::Op(operation) = &mut root.job {
            operation.op = Op::Sub;
        } else {
            return Err(runtime_error!("root has unexpected job"));
        }
    } else {
        return Err(runtime_error!("root not found"));
    }
    let mut values: HashMap<String, i64> = HashMap::new();
    values.insert("root".to_string(), 0);

    let mut unresolved = input
        .iter()
        .map(|m| m.name.clone())
        .collect::<VecDeque<_>>();
    while let Some(n) = unresolved.pop_front() {
        if &n != "humn" {
            let monkey = &monkeys[&n];
            match &monkey.job {
                Job::Const(value) => {
                    values.insert(monkey.name.clone(), *value);
                }
                Job::Op(Operation { op, a, b }) => {
                    match (values.get(&n), values.get(a), values.get(b)) {
                        (None, Some(&va), Some(&vb)) => {
                            let vn = op.apply(va, vb);
                            values.insert(n.clone(), vn);
                        }
                        (Some(&vn), Some(&va), None) => {
                            let vb = op.apply_inv_b(vn, va);
                            values.insert(b.clone(), vb);
                        }
                        (Some(&vn), None, Some(&vb)) => {
                            let va = op.apply_inv_a(vn, vb);
                            values.insert(a.clone(), va);
                        }
                        _ => unresolved.push_back(n),
                    }
                }
            }
        }
    }
    let result2 = values
        .get("humn")
        .ok_or(runtime_error!("value for human not known"))?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
