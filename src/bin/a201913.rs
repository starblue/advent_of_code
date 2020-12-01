use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::*;

use twodim::p2d;
use twodim::Point2d;

named!(
    int64<&str, i64>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit)), FromStr::from_str)
);

named!(
    input<&str, Vec<i64>>,
    separated_list!(tag!(","), int64)
);

#[derive(Clone, Debug)]
struct State {
    id: String,
    mem: HashMap<i64, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    ip: i64,
    rb: i64,
    default_input: Option<i64>,
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
            default_input: None,
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
    fn set_default_input(&mut self, input: Option<i64>) {
        self.default_input = input;
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
                    } else if let Some(v) = self.default_input {
                        self.store(a0, v);
                    } else {
                        // no input available, block at this instruction
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

const EMPTY: i64 = 0;
const WALL: i64 = 1;
const BLOCK: i64 = 2;
const PADDLE: i64 = 3;
const BALL: i64 = 4;

struct Board(HashMap<Point2d, i64>);
impl Board {
    fn new() -> Board {
        Board(HashMap::new())
    }
    fn paint(&mut self, p: Point2d, t: i64) {
        self.0.insert(p, t);
    }
    fn tile_count(&self, t: i64) -> usize {
        self.0.iter().filter(|(_, t1)| **t1 == t).count()
    }
    fn dump(&self) {
        let mut x_min = std::i64::MAX;
        let mut x_max = std::i64::MIN;
        let mut y_min = std::i64::MAX;
        let mut y_max = std::i64::MIN;
        for &p in self.0.keys() {
            x_min = x_min.min(p.x);
            x_max = x_max.max(p.x);
            y_min = y_min.min(p.y);
            y_max = y_max.max(p.y);
        }
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if let Some(&c) = self.0.get(&p2d(x, y)) {
                    print!("{}", vec![' ', 'W', 'B', '-', 'o'][c as usize]);
                } else {
                    print!(" ")
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push_str("\n");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let mut mem = result.unwrap().1;

    let mut state_a = State::new("A", &mem);
    let mut board_a = Board::new();

    while !state_a.is_halted() {
        let mut output = Vec::new();
        while !state_a.is_halted() && output.len() < 3 {
            state_a.step();
            // a step can produce at most one output
            if let Some(d) = state_a.pop_output() {
                output.push(d);
            }
        }
        if output.len() >= 3 {
            let x = output[0];
            let y = output[1];
            let t = output[2];
            board_a.paint(p2d(x, y), t);
        }
        //board_a.dump();
    }

    let mut state_b = State::new("B", &mem);
    state_b.store(0, 2);
    let mut board_b = Board::new();
    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    while !state_b.is_halted() {
        let mut output = Vec::new();
        while !state_b.is_halted() && output.len() < 3 {
            state_b.step();
            // a step can produce at most one output
            if let Some(d) = state_b.pop_output() {
                output.push(d);
            }
        }
        if output.len() >= 3 {
            let x = output[0];
            let y = output[1];
            let t = output[2];
            if x == -1 && y == 0 {
                score = t;
            } else {
                board_b.paint(p2d(x, y), t);
                if t == BALL {
                    ball_x = x;
                } else if t == PADDLE {
                    paddle_x = x;
                } else {
                    // do nothing
                }
            }
        }

        // control paddle
        state_b.set_default_input(Some({
            if ball_x < paddle_x {
                -1
            } else if ball_x > paddle_x {
                1
            } else {
                0
            }
        }));

        // println!("Score {}", score);
        // board_b.dump();
        // println!();
    }

    let result_a = board_a.tile_count(BLOCK);
    let result_b = score;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
