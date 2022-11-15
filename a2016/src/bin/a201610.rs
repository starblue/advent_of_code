use std::collections::HashMap;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Destination {
    Bot(usize),
    Output(usize),
}
impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Destination::Bot(id) => write!(f, "bot {}", id),
            Destination::Output(id) => write!(f, "output {}", id),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Value {
    pub id: usize,
    pub bot_id: usize,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "value {} goes to bot {}", self.id, self.bot_id)
    }
}

#[derive(Clone, Copy, Debug)]
struct Bot {
    pub id: usize,
    pub low_destination: Destination,
    pub high_destination: Destination,
}
impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "bot {} gives low to {} and high to {}",
            self.id, self.low_destination, self.high_destination,
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Value(Value),
    Bot(Bot),
}
impl Instruction {}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Instruction::Value(value) => write!(f, "{}", value),
            Instruction::Bot(bot) => write!(f, "{}", bot),
        }
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn destination_bot(i: &str) -> IResult<&str, Destination> {
    let (i, _) = tag("bot ")(i)?;
    let (i, id) = uint(i)?;
    Ok((i, Destination::Bot(id)))
}
fn destination_output(i: &str) -> IResult<&str, Destination> {
    let (i, _) = tag("output ")(i)?;
    let (i, id) = uint(i)?;
    Ok((i, Destination::Output(id)))
}
fn destination(i: &str) -> IResult<&str, Destination> {
    alt((destination_bot, destination_output))(i)
}

fn instruction_value(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("value ")(i)?;
    let (i, id) = uint(i)?;
    let (i, _) = tag(" goes to bot ")(i)?;
    let (i, bot_id) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction::Value(Value { id, bot_id })))
}
fn instruction_bot(i: &str) -> IResult<&str, Instruction> {
    let (i, _) = tag("bot ")(i)?;
    let (i, id) = uint(i)?;
    let (i, _) = tag(" gives low to ")(i)?;
    let (i, low_destination) = destination(i)?;
    let (i, _) = tag(" and high to ")(i)?;
    let (i, high_destination) = destination(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Instruction::Bot(Bot {
            id,
            low_destination,
            high_destination,
        }),
    ))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((instruction_value, instruction_bot))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for i in &input {
    //     println!("{}", i);
    // }

    let bots = input
        .iter()
        .filter_map(|i| {
            if let Instruction::Bot(bot) = i {
                Some((bot.id, bot))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    let mut bot_states = HashMap::new();
    for &i in &input {
        if let Instruction::Value(Value { id, bot_id }) = i {
            let entry = bot_states.entry(bot_id).or_insert_with(Vec::new);
            entry.push(id);
        }
    }
    let mut output_states = HashMap::new();
    let mut pending: Vec<(Destination, usize)> = Vec::new();
    let mut bot_61_17_id = None;
    loop {
        while let Some((destination, value)) = pending.pop() {
            match destination {
                Destination::Bot(id) => {
                    let entry = bot_states.entry(id).or_insert_with(Vec::new);
                    entry.push(value);
                }
                Destination::Output(id) => {
                    let entry = output_states.entry(id).or_insert_with(Vec::new);
                    entry.push(value);
                }
            }
        }
        if let Some((&id, v)) = bot_states.iter_mut().find(|(_id, v)| v.len() == 2) {
            v.sort();
            let low = v[0];
            let high = v[1];
            v.clear();
            if low == 17 && high == 61 {
                bot_61_17_id = Some(id);
            }
            let instruction = &bots[&id];
            pending.push((instruction.low_destination, low));
            pending.push((instruction.high_destination, high));
        } else {
            break;
        }
    }
    let result_a = bot_61_17_id.expect("no bot comparing 61 and 17 found");

    let result_b = output_states[&0][0] * output_states[&1][0] * output_states[&2][0];

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
