use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Vec2d;

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), int64)(i)
}

#[derive(Clone, Debug)]
struct State {
    #[allow(unused)]
    id: String,
    mem: HashMap<i64, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    ip: i64,
    rb: i64,
    saved_ip: i64,
    halted: bool,
}

#[derive(Debug)]
struct DecodeState<'a> {
    state: &'a mut State,
    instruction: i64,
    mode: i64,
}
impl<'a> DecodeState<'a> {
    fn new(state: &'a mut State, instruction: i64, mode: i64) -> DecodeState<'a> {
        DecodeState {
            state,
            instruction,
            mode,
        }
    }
    fn instruction(&self) -> i64 {
        self.instruction
    }
    fn value_parameter(&mut self) -> i64 {
        let pa = self.state.ip;
        self.state.ip += 1;
        let pm = self.mode % 10;
        self.mode /= 10;
        match pm {
            0 => {
                let a = self.state.load(pa);
                self.state.load(a)
            }
            1 => self.state.load(pa),
            2 => {
                let a = self.state.load(pa);
                self.state.load(self.state.rb + a)
            }
            _ => {
                panic!("illegal value parameter mode");
            }
        }
    }
    fn address_parameter(&mut self) -> i64 {
        let pa = self.state.ip;
        self.state.ip += 1;
        let pm = self.mode % 10;
        self.mode /= 10;
        match pm {
            0 => self.state.load(pa),
            2 => self.state.rb + self.state.load(pa),
            _ => {
                panic!("illegal address parameter mode");
            }
        }
    }
}

impl State {
    fn new(id: &str, mem: &[i64]) -> State {
        State {
            id: id.to_string(),
            mem: mem
                .iter()
                .enumerate()
                .map(|(k, v)| (k as i64, *v))
                .collect(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            ip: 0,
            rb: 0,
            saved_ip: 0,
            halted: false,
        }
    }
    fn load(&mut self, a: i64) -> i64 {
        if a >= 0 {
            let v = *self.mem.entry(a).or_insert(0);
            //println!("{}: {} load {}", self.id, a, v);
            v
        } else {
            panic!("attempt to load from negative address");
        }
    }
    fn store(&mut self, a: i64, v: i64) {
        if a >= 0 {
            *self.mem.entry(a).or_insert(0) = v;
        //println!("{}: {} store {}", self.id, a, v);
        } else {
            panic!("attempt to store to negative address");
        }
    }
    fn push_input(&mut self, input: i64) {
        self.input.push_back(input)
    }
    fn pop_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }
    fn is_halted(&self) -> bool {
        self.halted
    }
    fn start_decode(&mut self) -> DecodeState {
        self.saved_ip = self.ip;
        let opcode = self.load(self.ip);
        self.ip += 1;
        let instruction = opcode % 100;
        let mode = opcode / 100;
        DecodeState::new(self, instruction, mode)
    }
    fn undo_decode(&mut self) {
        self.ip = self.saved_ip;
    }
    fn step(&mut self) {
        if !self.halted {
            //println!("{} step", self.id);
            let mut ds = self.start_decode();
            let instruction = ds.instruction();
            match instruction {
                1 => {
                    // add
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    let a2 = ds.address_parameter();
                    self.store(a2, v0 + v1);
                }
                2 => {
                    // multiply
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    let a2 = ds.address_parameter();
                    self.store(a2, v0 * v1);
                }
                3 => {
                    // input
                    let a0 = ds.address_parameter();
                    if let Some(v) = self.input.pop_front() {
                        self.store(a0, v);
                    } else {
                        // no input available, do nothing
                        self.undo_decode();
                    }
                }
                4 => {
                    // output
                    let v0 = ds.value_parameter();
                    self.output.push_back(v0);
                }
                5 => {
                    // branch if non-zero
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    if v0 != 0 {
                        self.ip = v1;
                    }
                }
                6 => {
                    // branch if zero
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    if v0 == 0 {
                        self.ip = v1;
                    }
                }
                7 => {
                    // test lt
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    let a2 = ds.address_parameter();
                    self.store(a2, if v0 < v1 { 1 } else { 0 });
                }
                8 => {
                    // test eq
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    let a2 = ds.address_parameter();
                    self.store(a2, if v0 == v1 { 1 } else { 0 });
                }
                9 => {
                    // adjust relative base
                    let v0 = ds.value_parameter();
                    self.rb += v0;
                }
                99 => {
                    self.halted = true;
                }
                _ => {
                    panic!("illegal opcode");
                }
            }
        }
    }
}

const WALL: i64 = 0;
const MOVED: i64 = 1;
const FOUND: i64 = 2;

fn delta(command: i64) -> Vec2d {
    match command {
        1 => v2d(0, 1),
        2 => v2d(0, -1),
        3 => v2d(-1, 0),
        4 => v2d(1, 0),
        _ => panic!("Unknown command"),
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let mem = result.unwrap().1;

    let commands = (1..=4).collect::<Vec<_>>();

    // Do a breadth-first search using a queue (FIFO).
    // To do this and avoid walking the robot back we clone the IntCode state
    // (which might be considered cheating).
    let mut queue = commands
        .iter()
        .map(|&c| {
            let mut state = State::new("D", &mem);
            state.push_input(c);
            (state, p2d(0, 0), 1)
        })
        .collect::<VecDeque<_>>();

    // We avoid revisiting positions which have already been visited.
    let mut visited = HashSet::new();

    let mut min_count = 0;
    // We must assign some dummy state here.
    // This will be assigned for real when we find the oxygen tank.
    let mut state_at_oxygen = State::new("dummy", &[]);
    while let Some((mut state, pos, count)) = queue.pop_front() {
        visited.insert(pos);
        if !state.is_halted() {
            let mut output = None;
            while !state.is_halted() && output.is_none() {
                state.step();
                output = state.pop_output();
            }
            match output {
                Some(WALL) => {
                    // We hit a wall, forget this state.
                }
                Some(MOVED) => {
                    // We moved, enqueue further moves.
                    for &command in &commands {
                        let new_pos = pos + delta(command);
                        if !visited.contains(&new_pos) {
                            let mut state = state.clone();
                            state.push_input(command);
                            queue.push_back((state, new_pos, count + 1));
                        }
                    }
                }
                Some(FOUND) => {
                    // We found the oxygen tank, finish.
                    min_count = count;
                    state_at_oxygen = state;
                    break;
                }
                _ => panic!("unexpected output value"),
            }
        }
    }
    let result_a = min_count;

    // Do a new breadth-first search starting at the oxygen tank.
    let mut queue = commands
        .iter()
        .map(|&c| {
            let mut state = state_at_oxygen.clone();
            state.push_input(c);
            (state, p2d(0, 0), 1)
        })
        .collect::<VecDeque<_>>();
    let mut visited = HashSet::new();

    let mut max_count = 0;
    while let Some((mut state, pos, count)) = queue.pop_front() {
        visited.insert(pos);
        if !state.is_halted() {
            let mut output = None;
            while !state.is_halted() && output.is_none() {
                state.step();
                output = state.pop_output();
            }
            match output {
                Some(WALL) => {
                    // We hit a wall, forget this state.
                }
                Some(MOVED) => {
                    // We moved, enqueue further moves.
                    for &command in &commands {
                        let new_pos = pos + delta(command);
                        if !visited.contains(&new_pos) {
                            let mut state = state.clone();
                            state.push_input(command);
                            queue.push_back((state, new_pos, count + 1));
                        }
                    }
                    max_count = max_count.max(count);
                }
                Some(FOUND) => {
                    max_count = max_count.max(count);
                }
                _ => panic!("unexpected output value"),
            }
        }
    }
    let result_b = max_count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
