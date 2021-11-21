use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Val {
    Zero,
    One,
}
impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Val::Zero => write!(f, "0"),
            Val::One => write!(f, "1"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    write: Val,
    move_: Move,
    next_state: char,
}
impl Instruction {
    fn execute(&self, tape_state: &mut TapeState) {
        tape_state.write(self.write);
        tape_state.move_(self.move_);
        tape_state.set_state(self.next_state);
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "    - Write the value {}.", self.write)?;
        writeln!(f, "    - Move one slot to the {}.", self.move_)?;
        writeln!(f, "    - Continue with state {}.", self.next_state)
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    name: char,
    instruction0: Instruction,
    instruction1: Instruction,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "In state {}:", self.name)?;
        writeln!(f, "  If the current value is 0:")?;
        write!(f, "{}", self.instruction0)?;
        writeln!(f, "  If the current value is 1:")?;
        write!(f, "{}", self.instruction1)
    }
}

#[derive(Clone, Debug)]
struct Input {
    initial_state: char,
    steps: i64,
    states: Vec<State>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "Begin in state {}.", self.initial_state)?;
        writeln!(
            f,
            "Perform a diagnostic checksum after {} steps.",
            self.steps
        )?;
        for state in &self.states {
            writeln!(f)?;
            write!(f, "{}", state)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct TapeState {
    head_pos: i64,
    state: char,
    tape: HashMap<i64, Val>,
}
impl TapeState {
    fn init(initial_state: char) -> TapeState {
        TapeState {
            head_pos: 0,
            state: initial_state,
            tape: HashMap::new(),
        }
    }
    fn step(&mut self, states: &HashMap<char, State>) {
        let state = states[&self.state];
        match self.read() {
            Val::Zero => state.instruction0.execute(self),
            Val::One => state.instruction1.execute(self),
        }
    }
    fn diagnostic_checksum(&self) -> usize {
        self.tape.values().filter(|&&val| val == Val::One).count()
    }

    fn read(&mut self) -> Val {
        let entry = self.tape.entry(self.head_pos).or_insert(Val::Zero);
        *entry
    }
    fn write(&mut self, val: Val) {
        let entry = self.tape.entry(self.head_pos).or_insert(Val::Zero);
        *entry = val;
    }
    fn move_(&mut self, move_: Move) {
        self.head_pos += {
            match move_ {
                Move::Left => -1,
                Move::Right => 1,
            }
        };
    }
    fn set_state(&mut self, state: char) {
        self.state = state;
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn val(i: &str) -> IResult<&str, Val> {
    alt((value(Val::Zero, tag("0")), value(Val::One, tag("1"))))(i)
}

fn move_(i: &str) -> IResult<&str, Move> {
    alt((
        value(Move::Left, tag("left")),
        value(Move::Right, tag("right")),
    ))(i)
}

fn name(i: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("    - Write the value ")(i)?;
    let (i, v) = val(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("    - Move one slot to the ")(i)?;
    let (i, m) = move_(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("    - Continue with state ")(i)?;
    let (i, n) = name(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Instruction {
            write: v,
            move_: m,
            next_state: n,
        },
    ))
}

fn state(i: &str) -> IResult<&str, State> {
    let (i, _) = tag("In state ")(i)?;
    let (i, n) = name(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("  If the current value is 0:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, i0) = instruction(i)?;
    let (i, _) = tag("  If the current value is 1:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, i1) = instruction(i)?;
    Ok((
        i,
        State {
            name: n,
            instruction0: i0,
            instruction1: i1,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("Begin in state ")(i)?;
    let (i, initial_state) = name(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Perform a diagnostic checksum after ")(i)?;
    let (i, steps) = uint(i)?;
    let (i, _) = tag(" steps.")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, states) = separated_list1(line_ending, state)(i)?;
    Ok((
        i,
        Input {
            initial_state,
            steps,
            states,
        },
    ))
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
    // print!("{}", input);

    let states = input
        .states
        .iter()
        .map(|&s| (s.name, s))
        .collect::<HashMap<_, _>>();
    let mut tape_state = TapeState::init(input.initial_state);
    for _ in 0..input.steps {
        tape_state.step(&states);
    }
    let result_a = tape_state.diagnostic_checksum();

    println!("a: {}", result_a);
}
