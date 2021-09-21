use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::alt;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map;
use nom::map_res;
use nom::named;
use nom::recognize;
use nom::tag;

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
    Value(u16),
}
impl Atom {
    fn eval(&self, env: &HashMap<Wire, u16>) -> Option<u16> {
        match self {
            Atom::Wire(w) => env.get(w).copied(),
            Atom::Value(value) => Some(*value),
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

named!(uint<&str, u16>,
    map_res!(digit1, FromStr::from_str)
);

named!(id<&str, String>,
    map!(recognize!(alpha1), String::from)
);

named!(wire<&str, Wire>,
    do_parse!(
        id: id >> (Wire { id })
    )
);

named!(atom<&str, Atom>,
    alt!(
        do_parse!(wire: wire >> (Atom::Wire(wire))) |
        do_parse!(n: uint >> (Atom::Value(n)))
    )
);

named!(not<&str, Gate>,
    do_parse!(
        tag!("NOT ") >>
        atom: atom >> (Gate::Not(atom))
    )
);

named!(and<&str, Gate>,
    do_parse!(
        atom0: atom >>
        tag!(" AND ") >>
        atom1: atom >> (Gate::And(atom0, atom1))
    )
);

named!(or<&str, Gate>,
    do_parse!(
        atom0: atom >>
        tag!(" OR ") >>
        atom1: atom >> (Gate::Or(atom0, atom1))
    )
);

named!(lshift<&str, Gate>,
    do_parse!(
        atom: atom >>
        tag!(" LSHIFT ") >>
        n: uint >> (Gate::LShift(atom, n))
    )
);

named!(rshift<&str, Gate>,
    do_parse!(
        atom: atom >>
        tag!(" RSHIFT ") >>
        n: uint >> (Gate::RShift(atom, n))
    )
);

named!(gate<&str, Gate>,
    alt!(not | and | or | lshift | rshift)
);

named!(source<&str, Source>,
    alt!(
        do_parse!(gate: gate >> (Source::Gate(gate))) |
        do_parse!(atom: atom >> (Source::Atom(atom)))
    )
);

named!(instruction<&str, Instruction>,
    do_parse!(
        source: source >>
        tag!(" -> ") >>
        wire: wire >>
        line_ending >> (Instruction { source, wire })
    )
);

named!(input<&str, Vec<Instruction>>,
    many1!(instruction)
);

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
