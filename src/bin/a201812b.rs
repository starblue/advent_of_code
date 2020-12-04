use std::collections::HashMap;
use std::fmt;
use std::io;

use nom::alt;
use nom::call;
use nom::char;
use nom::do_parse;
use nom::error_position;
use nom::many0;
use nom::many_m_n;
use nom::named;
use nom::tag;
use nom::value;

const STEPS: i64 = 50_000_000_000;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rule {
    left: Vec<bool>,
    right: bool,
}

#[derive(Clone, Debug)]
enum Error {}

named!(pot<&str, bool>,
    alt!(
        value!(true, char!('#')) |
        value!(false, char!('.'))
    )
);

named!(initial_state<&str, Vec<bool>>,
    do_parse!(tag!("initial state: ") >> v: many0!(pot) >> (v))
);

named!(rule<&str, Rule>,
    do_parse!(
        left: many_m_n!(5, 5, pot) >>
            tag!(" => ") >>
            right: pot >> (Rule { left, right })
    )
);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    /// number of leftmost pot in states
    leftmost: i64,
    states: Vec<bool>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.leftmost)?;
        for &s in &self.states {
            write!(f, "{}", if s { "*" } else { "." })?;
        }
        Ok(())
    }
}

fn step(rules: &HashMap<Vec<bool>, bool>, mut state: State) -> State {
    // pad states by 4 so rules with overlap of 1 can apply
    state.leftmost -= 4;
    for _ in 0..4 {
        state.states.insert(0, false);
    }
    for _ in 0..4 {
        state.states.push(false);
    }

    // new state
    let mut states = (&state.states)
        .windows(5)
        .map(|w| rules[w])
        .collect::<Vec<_>>();
    let mut leftmost = state.leftmost + 2;

    // remove false from ends
    while let Some(false) = states.pop() {}
    // re-append the true we removed
    states.push(true);

    while let Some(false) = states.first() {
        states.remove(0);
        leftmost += 1;
    }
    State { leftmost, states }
}

fn main() {
    let mut line = String::new();

    // read initial state
    io::stdin().read_line(&mut line).expect("I/O error");
    // parse line
    let result = initial_state(&line);
    //println!("{:?}", result);
    let mut state = State {
        leftmost: 0,
        states: result.unwrap().1,
    };

    // read empty line
    line.clear();
    io::stdin().read_line(&mut line).expect("I/O error");

    let mut rules = HashMap::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.is_empty() {
            break;
        }

        // parse line
        let result = rule(&line.trim_end());
        //println!("{:?}", result);

        let rule = result.unwrap().1;
        rules.insert(rule.left, rule.right);
    }

    let mut last_state = state.clone();
    let mut i = 0;
    loop {
        state = step(&rules, state);
        i += 1;
        if state.states == last_state.states {
            // we found a loop
            break;
        }
        last_state = state.clone();
    }

    // fast-forward
    let delta = state.leftmost - last_state.leftmost;
    state.leftmost += (STEPS - i) * delta;

    let mut sum = 0;
    let mut i = state.leftmost;
    for s in state.states {
        if s {
            sum += i;
        }
        i += 1;
    }
    println!("{}", sum);
}
