use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

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
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operation {
    Inc,
    Dec,
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Inc => write!(f, "inc"),
            Operation::Dec => write!(f, "dec"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CondOp {
    Eq,
    Neq,
    Leq,
    Geq,
    Lt,
    Gt,
}
impl CondOp {
    fn apply(&self, a: i64, b: i64) -> bool {
        match self {
            CondOp::Eq => a == b,
            CondOp::Neq => a != b,
            CondOp::Leq => a <= b,
            CondOp::Geq => a >= b,
            CondOp::Lt => a < b,
            CondOp::Gt => a > b,
        }
    }
}
impl fmt::Display for CondOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CondOp::Eq => write!(f, "=="),
            CondOp::Neq => write!(f, "!="),
            CondOp::Leq => write!(f, "<="),
            CondOp::Geq => write!(f, ">="),
            CondOp::Lt => write!(f, "<"),
            CondOp::Gt => write!(f, ">"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Condition {
    reg: String,
    op: CondOp,
    arg: i64,
}
impl Condition {
    fn eval(&self, state: &State) -> bool {
        let &reg_val = state.regs.get(&self.reg).unwrap_or(&0);
        self.op.apply(reg_val, self.arg)
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.reg, self.op, self.arg)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instruction {
    reg: String,
    op: Operation,
    arg: i64,
    cond: Condition,
}
impl Instruction {
    fn execute(&self, state: &mut State) {
        if self.cond.eval(state) {
            let entry = state.regs.entry(self.reg.clone()).or_insert(0);
            match self.op {
                Operation::Inc => {
                    *entry += self.arg;
                }
                Operation::Dec => {
                    *entry -= self.arg;
                }
            }
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} if {}", self.reg, self.op, self.arg, self.cond)
    }
}

fn name(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Inc, tag("inc")),
        value(Operation::Dec, tag("dec")),
    ))(i)
}

fn cond_op(i: &str) -> IResult<&str, CondOp> {
    alt((
        value(CondOp::Eq, tag("==")),
        value(CondOp::Neq, tag("!=")),
        value(CondOp::Leq, tag("<=")),
        value(CondOp::Geq, tag(">=")),
        // Prefixes of other alternatives must be put after them.
        value(CondOp::Lt, tag("<")),
        value(CondOp::Gt, tag(">")),
    ))(i)
}

fn condition(i: &str) -> IResult<&str, Condition> {
    let (i, reg) = name(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, op) = cond_op(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, arg) = int64(i)?;
    Ok((i, Condition { reg, op, arg }))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, reg) = name(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, op) = operation(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, arg) = int64(i)?;
    let (i, _) = tag(" if ")(i)?;
    let (i, cond) = condition(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction { reg, op, arg, cond }))
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
}

#[derive(Clone, Debug)]
struct State {
    regs: HashMap<String, i64>,
}
impl State {
    fn new() -> State {
        State {
            regs: HashMap::new(),
        }
    }
    fn max_value(&self) -> i64 {
        *self.regs.values().max().unwrap_or(&0)
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for n in &input {
    //     println!("{}", n);
    // }

    let mut state = State::new();
    let mut max_value = 0;
    for instruction in &input {
        instruction.execute(&mut state);
        max_value = max_value.max(state.max_value());
    }
    let result_a = state.max_value();

    let result_b = max_value;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::cond_op;
    use crate::instruction;
    use crate::name;
    use crate::CondOp;
    use crate::Condition;
    use crate::Instruction;
    use crate::Operation;

    #[test]
    fn test_name_a() {
        let (i, s) = name("a").unwrap();
        assert_eq!(i, "");
        assert_eq!(s, "a".to_string());
    }
    #[test]
    fn test_name_a_() {
        let (i, s) = name("a ").unwrap();
        assert_eq!(i, " ");
        assert_eq!(s, "a".to_string());
    }

    #[test]
    fn test_cond_op_leq() {
        let (i, cond_op) = cond_op("<=").unwrap();
        assert_eq!(i, "");
        assert_eq!(cond_op, CondOp::Leq);
    }
    #[test]
    fn test_cond_op_geq() {
        let (i, cond_op) = cond_op(">=").unwrap();
        assert_eq!(i, "");
        assert_eq!(cond_op, CondOp::Geq);
    }

    #[test]
    fn test_instruction() {
        let (i, s) = instruction("a inc -904 if me >= -7\n").unwrap();
        assert_eq!(i, "");
        assert_eq!(
            s,
            Instruction {
                reg: "a".to_string(),
                op: Operation::Inc,
                arg: -904,
                cond: Condition {
                    reg: "me".to_string(),
                    op: CondOp::Geq,
                    arg: -7
                }
            }
        );
    }
}
