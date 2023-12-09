use core::fmt;
use core::mem::swap;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Left,
    Right,
}
impl Instruction {
    fn to_char(self) -> char {
        match self {
            Instruction::Left => 'L',
            Instruction::Right => 'R',
        }
    }
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Id(String);
impl Id {
    fn start1() -> Id {
        Id("AAA".to_string())
    }
    fn goal1() -> Id {
        Id("ZZZ".to_string())
    }
    fn is_start2(&self) -> bool {
        self.0.ends_with('A')
    }
    fn is_goal2(&self) -> bool {
        self.0.ends_with('Z')
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
struct Node {
    id: Id,
    left: Id,
    right: Id,
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} = ({}, {})", self.id, self.left, self.right)
    }
}

#[derive(Clone, Debug)]
struct Map {
    nodes: Vec<Node>,
    map: HashMap<Id, Node>,
}
impl Map {
    fn new(nodes: Vec<Node>) -> Map {
        let map = nodes
            .iter()
            .map(|n| (n.id.clone(), n.clone()))
            .collect::<HashMap<_, _>>();
        Map { nodes, map }
    }
}
impl Map {
    fn left(&self, id: &Id) -> &Id {
        &self.map[id].left
    }
    fn right(&self, id: &Id) -> &Id {
        &self.map[id].right
    }
    fn go(&self, id: &Id, instruction: Instruction) -> &Id {
        match instruction {
            Instruction::Left => self.left(id),
            Instruction::Right => self.right(id),
        }
    }
    fn start2_ids(&self) -> Vec<&Id> {
        self.nodes
            .iter()
            .map(|n| &n.id)
            .filter(|id| id.is_start2())
            .collect::<Vec<_>>()
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    instructions: Vec<Instruction>,
    map: Map,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for instruction in &self.instructions {
            write!(f, "{}", instruction)?;
        }
        writeln!(f)?;
        writeln!(f)?;
        writeln!(f, "{}", self.map)?;
        Ok(())
    }
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Left, tag("L")),
        value(Instruction::Right, tag("R")),
    ))(i)
}

fn id(i: &str) -> IResult<&str, Id> {
    let (i, cs) = many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))(i)?;
    Ok((i, Id(cs.into_iter().collect::<String>())))
}

fn node(i: &str) -> IResult<&str, Node> {
    let (i, node_id) = id(i)?;
    let (i, _) = tag(" = (")(i)?;
    let (i, left) = id(i)?;
    let (i, _) = tag(", ")(i)?;
    let (i, right) = id(i)?;
    let (i, _) = tag(")")(i)?;
    Ok((
        i,
        Node {
            id: node_id,
            left,
            right,
        },
    ))
}

fn map(i: &str) -> IResult<&str, Map> {
    let (i, nodes) = separated_list1(line_ending, node)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Map::new(nodes)))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, instructions) = many1(instruction)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, map) = map(i)?;
    Ok((i, Input { instructions, map }))
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 && b == 0 {
        (0, 0, 0)
    } else {
        let mut a = a;
        let mut sa = 1;
        let mut ta = 0;
        let mut b = b;
        let mut sb = 0;
        let mut tb = 1;
        if a < b {
            swap(&mut a, &mut b);
            swap(&mut sa, &mut sb);
            swap(&mut ta, &mut tb);
        }
        while b > 0 {
            let d = a / b;
            let r = a % b; // r == a - d * b
            let s_new = sa - d * sb;
            let t_new = ta - d * tb;
            a = b;
            sa = sb;
            ta = tb;
            b = r;
            sb = s_new;
            tb = t_new;
        }
        (a, sa, ta)
    }
}

fn chinese_remainder(mr0: (i64, i64), mr1: (i64, i64)) -> Option<(i64, i64)> {
    let (m0, r0) = mr0;
    let (m1, r1) = mr1;

    let (gcd, s, t) = egcd(m0, m1);
    if r0 % gcd != 0 || r1 % gcd != 0 {
        // no solution
        None
    } else {
        let m = m0 / gcd * m1;
        let r = (r0 * m1 * t + r1 * m0 * s).rem_euclid(m);
        Some((m, r))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State<'a> {
    instruction_index: usize,
    node_id: &'a Id,
}
impl<'a> State<'a> {
    fn new(instruction_index: usize, node_id: &'a Id) -> State<'a> {
        State {
            instruction_index,
            node_id,
        }
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut instructions = input.instructions.iter().copied().cycle();

    let mut pos = &Id::start1();
    let mut count = 0;
    while pos != &Id::goal1() {
        let instruction = instructions
            .next()
            .ok_or(util::runtime_error!("unexpected end of instructions"))?;
        pos = input.map.go(pos, instruction);
        count += 1;
    }
    let result1 = count;

    let mut mrs = vec![(1, 0)];
    for start_id in input.map.start2_ids().into_iter() {
        let mut state = State {
            instruction_index: 0,
            node_id: start_id,
        };
        let mut seen = HashMap::new();
        let mut goal_indices = HashSet::new();
        let mut count = 0;
        while !seen.contains_key(&state) {
            seen.insert(state.clone(), count);
            if state.node_id.is_goal2() {
                goal_indices.insert(count);
            }
            let instruction = input.instructions[state.instruction_index];
            let new_node_id = input.map.go(state.node_id, instruction);
            let new_instruction_index = (state.instruction_index + 1) % input.instructions.len();
            state = State::new(new_instruction_index, new_node_id);
            count += 1;
        }
        let loop0_start = seen[&state];
        let loop1_start = count;
        let loop_len = loop1_start - loop0_start;

        let mut new_mrs = Vec::new();
        for mr in mrs {
            for &goal_index in &goal_indices {
                if let Some(new_mr) = chinese_remainder(mr, (loop_len, goal_index)) {
                    new_mrs.push(new_mr);
                }
            }
        }
        mrs = new_mrs;
    }

    // For each starting value, the goal state is reached after exactly
    // the number of steps needed for one iteration through its loop,
    // hence the remainder is zero.
    // We add the modulus, because the loop is only entered after some steps,
    // so that the goal is not reached in the first iteration.
    let result2 = mrs.iter().map(|&(m, r)| m + r).min().ok_or("no solution")?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
