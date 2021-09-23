use std::collections::VecDeque;
use std::io;
use std::io::Read;
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
    mem: Vec<i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    ip: usize,
    saved_ip: usize,
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
                let a = self.state.load(pa) as usize;
                self.state.load(a)
            }
            1 => self.state.load(pa),
            _ => {
                panic!("illegal value parameter mode");
            }
        }
    }
    fn address_parameter(&mut self) -> usize {
        let pa = self.state.ip;
        self.state.ip += 1;
        let pm = self.mode % 10;
        self.mode /= 10;
        match pm {
            0 => self.state.load(pa) as usize,
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
            mem: mem.to_vec(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            ip: 0,
            saved_ip: 0,
            halted: false,
        }
    }
    fn load(&mut self, a: usize) -> i64 {
        let v = self.mem[a];
        //println!("{}: {} load {}", self.id, a, v);
        v
    }
    fn store(&mut self, a: usize, v: i64) {
        self.mem[a] = v;
        //println!("{}: {} store {}", self.id, a, v);
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
                        self.ip = v1 as usize;
                    }
                }
                6 => {
                    // branch if zero
                    let v0 = ds.value_parameter();
                    let v1 = ds.value_parameter();
                    if v0 == 0 {
                        self.ip = v1 as usize;
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

fn run(id: &str, mem: &[i64], input: Vec<i64>) -> i64 {
    let mut state = State::new(id, mem);
    for i in input {
        state.push_input(i);
    }
    while !state.is_halted() {
        state.step();
    }
    if let Some(v) = state.pop_output() {
        v
    } else {
        panic!("no output at end of run");
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
    //println!("{:?}", result);

    let mem = result.unwrap().1;

    let mut max_out_a = 0;
    for a in 0..5 {
        for b in 0..5 {
            if b != a {
                for c in 0..5 {
                    if c != a && c != b {
                        for d in 0..5 {
                            if d != a && d != b && d != c {
                                for e in 0..5 {
                                    if e != a && e != b && e != c && e != d {
                                        let i0 = 0;
                                        let i1 = run("A", &mem, vec![a, i0]);
                                        let i2 = run("B", &mem, vec![b, i1]);
                                        let i3 = run("C", &mem, vec![c, i2]);
                                        let i4 = run("D", &mem, vec![d, i3]);
                                        let out = run("E", &mem, vec![e, i4]);
                                        max_out_a = max_out_a.max(out);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut max_out_b = 0;
    for a in 5..10 {
        for b in 5..10 {
            if b != a {
                for c in 5..10 {
                    if c != a && c != b {
                        for d in 5..10 {
                            if d != a && d != b && d != c {
                                for e in 5..10 {
                                    if e != a && e != b && e != c && e != d {
                                        let mut state_a = State::new("A", &mem);
                                        let mut state_b = State::new("B", &mem);
                                        let mut state_c = State::new("C", &mem);
                                        let mut state_d = State::new("D", &mem);
                                        let mut state_e = State::new("E", &mem);

                                        // input phase
                                        state_a.push_input(a);
                                        state_b.push_input(b);
                                        state_c.push_input(c);
                                        state_d.push_input(d);
                                        state_e.push_input(e);

                                        // initial input
                                        state_a.push_input(0);

                                        let mut out = 0;

                                        // run
                                        while !(state_a.is_halted()
                                            && state_b.is_halted()
                                            && state_c.is_halted()
                                            && state_d.is_halted()
                                            && state_e.is_halted())
                                        {
                                            // execute one instruction on all machines
                                            state_a.step();
                                            state_b.step();
                                            state_c.step();
                                            state_d.step();
                                            state_e.step();

                                            // connect outputs to inputs
                                            while let Some(v) = state_a.pop_output() {
                                                state_b.push_input(v);
                                            }
                                            while let Some(v) = state_b.pop_output() {
                                                state_c.push_input(v);
                                            }
                                            while let Some(v) = state_c.pop_output() {
                                                state_d.push_input(v);
                                            }
                                            while let Some(v) = state_d.pop_output() {
                                                state_e.push_input(v);
                                            }
                                            while let Some(v) = state_e.pop_output() {
                                                state_a.push_input(v);

                                                // keep last value as output
                                                out = v;
                                            }
                                        }
                                        max_out_b = max_out_b.max(out);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let result_a = max_out_a;
    let result_b = max_out_b;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
