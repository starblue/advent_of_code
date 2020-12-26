use core::iter::repeat;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::*;

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
    waiting_for_input: bool,
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

#[allow(unused)]
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
            waiting_for_input: false,
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
    fn is_waiting_for_input(&self) -> bool {
        self.waiting_for_input && self.input.is_empty()
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
                        self.waiting_for_input = false;
                    } else if let Some(v) = self.default_input {
                        self.store(a0, v);
                        self.waiting_for_input = true;
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

    let n = 50;

    let mut states_a = (0..n)
        .map(|i| {
            let mut state = State::new("NIC", &mem);
            state.set_default_input(Some(-1));
            state.push_input(i as i64);
            state
        })
        .collect::<Vec<_>>();
    let mut outputs_a = repeat(VecDeque::new()).take(n).collect::<Vec<_>>();

    let mut result_a = None;
    while result_a == None {
        // execute one instruction on all machines
        for i in 0..n {
            let state = &mut states_a[i];
            let output = &mut outputs_a[i];
            state.step();
            while let Some(value) = state.pop_output() {
                output.push_back(value);
            }
        }
        for i in 0..n {
            let output = &mut outputs_a[i];
            if output.len() >= 3 {
                let adr = output.pop_front().unwrap() as usize;
                let x = output.pop_front().unwrap();
                let y = output.pop_front().unwrap();

                if adr < n {
                    states_a[adr].push_input(x);
                    states_a[adr].push_input(y);
                } else if adr == 255 {
                    result_a = Some(y);
                } else {
                    println!("{} {} {}", adr, x, y);
                }
            }
        }
    }

    let mut states_b = (0..n)
        .map(|i| {
            let mut state = State::new("NIC", &mem);
            state.set_default_input(Some(-1));
            state.push_input(i as i64);
            state
        })
        .collect::<Vec<_>>();
    let mut outputs_b = repeat(VecDeque::new()).take(n).collect::<Vec<_>>();
    let mut nat_input = None;
    let mut nat_ys = HashSet::new();

    let mut result_b = None;
    while result_b == None {
        // execute one instruction on all machines
        for i in 0..n {
            let state = &mut states_b[i];
            let output = &mut outputs_b[i];
            state.step();
            while let Some(value) = state.pop_output() {
                output.push_back(value);
            }
        }
        if states_b.iter().all(|s| s.is_waiting_for_input()) {
            // all NICs are idle, send NAT packet
            if let Some((x, y)) = nat_input {
                states_b[0].push_input(x);
                states_b[0].push_input(y);
                if nat_ys.contains(&y) {
                    result_b = Some(y);
                } else {
                    nat_ys.insert(y);
                }
            }
        }
        for i in 0..n {
            let output = &mut outputs_b[i];
            if output.len() >= 3 {
                let adr = output.pop_front().unwrap() as usize;
                let x = output.pop_front().unwrap();
                let y = output.pop_front().unwrap();

                if adr < n {
                    states_b[adr].push_input(x);
                    states_b[adr].push_input(y);
                } else if adr == 255 {
                    nat_input = Some((x, y));
                } else {
                    println!("{} {} {}", adr, x, y);
                }
            }
        }
    }

    let result_a = result_a.unwrap();
    let result_b = result_b.unwrap();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
