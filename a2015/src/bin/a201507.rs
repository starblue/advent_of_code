use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Wire {
    id: String,
}

fn apply_not(i: Option<u16>) -> Option<u16> {
    i.map(|v| !v)
}
fn apply_and(i0: Option<u16>, i1: Option<u16>) -> Option<u16> {
    match (i0, i1) {
        (Some(v0), Some(v1)) => Some(v0 & v1),
        _ => None,
    }
}
fn apply_or(i0: Option<u16>, i1: Option<u16>) -> Option<u16> {
    match (i0, i1) {
        (Some(v0), Some(v1)) => Some(v0 | v1),
        _ => None,
    }
}
fn apply_lshift(i: Option<u16>, shift: u16) -> Option<u16> {
    i.map(|v| v << shift)
}
fn apply_rshift(i: Option<u16>, shift: u16) -> Option<u16> {
    i.map(|v| v >> shift)
}

#[derive(Clone, Debug)]
enum Atom {
    Wire(Wire),
    Number(u16),
}
impl Atom {
    fn eval(&self, env: &HashMap<Wire, u16>) -> Option<u16> {
        match self {
            Atom::Wire(w) => env.get(w).copied(),
            Atom::Number(value) => Some(*value),
        }
    }
}

#[derive(Clone, Debug)]
enum Gate {
    Not(Atom),
    And(Atom, Atom),
    Or(Atom, Atom),
    LShift(Atom, u16),
    RShift(Atom, u16),
}
impl Gate {
    fn eval(&self, env: &HashMap<Wire, u16>) -> Option<u16> {
        match self {
            Gate::Not(a) => apply_not(a.eval(env)),
            Gate::And(a0, a1) => apply_and(a0.eval(env), a1.eval(env)),
            Gate::Or(a0, a1) => apply_or(a0.eval(env), a1.eval(env)),
            Gate::LShift(a, shift) => apply_lshift(a.eval(env), *shift),
            Gate::RShift(a, shift) => apply_rshift(a.eval(env), *shift),
        }
    }
}

#[derive(Clone, Debug)]
enum Source {
    Gate(Gate),
    Atom(Atom),
}
impl Source {
    fn eval(&self, env: &HashMap<Wire, u16>) -> Option<u16> {
        match self {
            Source::Gate(g) => g.eval(env),
            Source::Atom(a) => a.eval(env),
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    source: Source,
    wire: Wire,
}

fn uint(i: &str) -> IResult<&str, u16> {
    map_res(digit1, FromStr::from_str)(i)
}

fn id(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn wire(i: &str) -> IResult<&str, Wire> {
    let (i, id) = id(i)?;
    Ok((i, Wire { id }))
}

fn atom_wire(i: &str) -> IResult<&str, Atom> {
    let (i, wire) = wire(i)?;
    Ok((i, Atom::Wire(wire)))
}

fn atom_number(i: &str) -> IResult<&str, Atom> {
    let (i, number) = uint(i)?;
    Ok((i, Atom::Number(number)))
}

fn atom(i: &str) -> IResult<&str, Atom> {
    alt((atom_wire, atom_number))(i)
}

fn not(i: &str) -> IResult<&str, Gate> {
    let (i, _) = tag("NOT ")(i)?;
    let (i, atom) = atom(i)?;
    Ok((i, Gate::Not(atom)))
}

fn and(i: &str) -> IResult<&str, Gate> {
    let (i, atom0) = atom(i)?;
    let (i, _) = tag(" AND ")(i)?;
    let (i, atom1) = atom(i)?;
    Ok((i, Gate::And(atom0, atom1)))
}

fn or(i: &str) -> IResult<&str, Gate> {
    let (i, atom0) = atom(i)?;
    let (i, _) = tag(" OR ")(i)?;
    let (i, atom1) = atom(i)?;
    Ok((i, Gate::Or(atom0, atom1)))
}

fn lshift(i: &str) -> IResult<&str, Gate> {
    let (i, atom) = atom(i)?;
    let (i, _) = tag(" LSHIFT ")(i)?;
    let (i, n) = uint(i)?;
    Ok((i, Gate::LShift(atom, n)))
}

fn rshift(i: &str) -> IResult<&str, Gate> {
    let (i, atom) = atom(i)?;
    let (i, _) = tag(" RSHIFT ")(i)?;
    let (i, n) = uint(i)?;
    Ok((i, Gate::RShift(atom, n)))
}

fn gate(i: &str) -> IResult<&str, Gate> {
    alt((not, and, or, lshift, rshift))(i)
}

fn source_gate(i: &str) -> IResult<&str, Source> {
    let (i, gate) = gate(i)?;
    Ok((i, Source::Gate(gate)))
}

fn source_atom(i: &str) -> IResult<&str, Source> {
    let (i, atom) = atom(i)?;
    Ok((i, Source::Atom(atom)))
}

fn source(i: &str) -> IResult<&str, Source> {
    alt((source_gate, source_atom))(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, source) = source(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, wire) = wire(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instruction { source, wire }))
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    many1(instruction)(i)
}

fn eval(instructions: Vec<Instruction>, env: &mut HashMap<Wire, u16>) {
    let mut instructions = instructions;
    while !instructions.is_empty() {
        let mut defered_instructions = Vec::new();
        for i in instructions {
            if let Some(value) = i.source.eval(env) {
                env.insert(i.wire.clone(), value);
            } else {
                defered_instructions.push(i);
            }
        }
        instructions = defered_instructions;
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
    // println!("{:#?}", input);

    let wire_a = Wire {
        id: "a".to_string(),
    };
    let wire_b = Wire {
        id: "b".to_string(),
    };

    let mut env = HashMap::new();
    eval(input.clone(), &mut env);
    let result_a = *env.get(&wire_a).expect("no signal on wire a");

    let mut env = HashMap::new();
    env.insert(wire_b.clone(), result_a);
    eval(
        input
            .into_iter()
            .filter(|i| i.wire != wire_b)
            .collect::<Vec<_>>(),
        &mut env,
    );
    let result_b = *env.get(&wire_a).expect("no signal on wire a");

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
