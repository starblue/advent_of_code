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

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), int64)(i)
}

fn run(state: Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut state = state;
    state[1] = noun;
    state[2] = verb;

    let mut ip = 0;
    loop {
        match state[ip] {
            1 => {
                let a0 = state[ip + 1] as usize;
                let a1 = state[ip + 2] as usize;
                let a2 = state[ip + 3] as usize;
                ip += 4;

                state[a2] = state[a0] + state[a1];
            }
            2 => {
                let a0 = state[ip + 1] as usize;
                let a1 = state[ip + 2] as usize;
                let a2 = state[ip + 3] as usize;
                ip += 4;

                state[a2] = state[a0] * state[a1];
            }
            99 => {
                break;
            }
            _ => {
                panic!("illegal opcode");
            }
        }
    }

    state[0]
}

fn find_inputs(state: &[i64], result: i64) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(state.to_vec(), noun, verb) == result {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no inputs found");
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let state = result.unwrap().1;

    let result_a = run(state.clone(), 12, 2);
    let result_b = find_inputs(&state, 19690720);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
