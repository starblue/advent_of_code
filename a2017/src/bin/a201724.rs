use std::collections::HashSet;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Component(i64, i64);
impl Component {
    fn strength(&self) -> i64 {
        self.0 + self.1
    }
}
impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}/{}", self.0, self.1)
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn component(i: &str) -> IResult<&str, Component> {
    let (i, p0) = uint(i)?;
    let (i, _) = tag("/")(i)?;
    let (i, p1) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Component(p0, p1)))
}

fn input(i: &str) -> IResult<&str, Vec<Component>> {
    many1(component)(i)
}

#[derive(Clone, Debug)]
struct State {
    components: HashSet<Component>,
    port: i64,
    length: i64,
    strength: i64,
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for component in &input {
    //     println!("{}", component);
    // }

    let state = State {
        components: input.iter().cloned().collect::<HashSet<_>>(),
        port: 0,
        length: 0,
        strength: 0,
    };
    let mut states = vec![state];
    let mut max_strength = 0;
    let mut max_length = 0;
    let mut max_length_max_strength = 0;
    while let Some(state) = states.pop() {
        max_strength = max_strength.max(state.strength);
        if state.length > max_length {
            max_length = state.length;
            max_length_max_strength = state.strength;
        } else if state.length == max_length {
            max_length_max_strength = max_length_max_strength.max(state.strength)
        }
        // try to extend state
        for component in &state.components {
            if state.port == component.0 || state.port == component.1 {
                // We can connect this component.
                let new_port = if state.port == component.0 {
                    component.1
                } else {
                    component.0
                };
                let mut new_components = state.components.clone();
                new_components.remove(component);
                let new_state = State {
                    components: new_components,
                    port: new_port,
                    length: state.length + 1,
                    strength: state.strength + component.strength(),
                };
                states.push(new_state);
            }
        }
    }
    let result_a = max_strength;

    let result_b = max_length_max_strength;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
