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

fn get(state: &[i64], pa: usize, pm: i64) -> i64 {
    match pm {
        0 => {
            let a = state[pa] as usize;
            state[a]
        }
        1 => state[pa],
        _ => {
            panic!("illegal parameter mode");
        }
    }
}

fn run(state: Vec<i64>, input: Vec<i64>) -> i64 {
    let mut state = state;
    let mut read_p = 0;
    let mut output = 0;

    let mut ip = 0;
    loop {
        // println!("{}: {:?}", ip, &state[..20]);

        let ins = state[ip] % 100;
        let pm0 = (state[ip] / 100) % 10;
        let pm1 = (state[ip] / 1000) % 10;
        match ins {
            1 => {
                let p0 = get(&state, ip + 1, pm0);
                let p1 = get(&state, ip + 2, pm1);
                let a2 = state[ip + 3] as usize;
                ip += 4;

                state[a2] = p0 + p1;
            }
            2 => {
                let p0 = get(&state, ip + 1, pm0);
                let p1 = get(&state, ip + 2, pm1);
                let a2 = state[ip + 3] as usize;
                ip += 4;

                state[a2] = p0 * p1;
            }
            3 => {
                let a0 = state[ip + 1] as usize;
                ip += 2;

                state[a0] = input[read_p];
                read_p += 1;
            }
            4 => {
                let p0 = get(&state, ip + 1, pm0);
                ip += 2;

                output = p0;
                // println!("output: {}", output);
            }
            5 => {
                let p0 = get(&state, ip + 1, pm0);
                let p1 = get(&state, ip + 2, pm1);
                ip += 3;
                if p0 != 0 {
                    ip = p1 as usize;
                }
            }
            6 => {
                let p0 = get(&state, ip + 1, pm0);
                let p1 = get(&state, ip + 2, pm1);
                ip += 3;
                if p0 == 0 {
                    ip = p1 as usize;
                }
            }
            7 => {
                let p0 = get(&state, ip + 1, pm0);
                let p1 = get(&state, ip + 2, pm1);
                let a2 = state[ip + 3] as usize;
                ip += 4;
                state[a2] = {
                    if p0 < p1 {
                        1
                    } else {
                        0
                    }
                };
            }
            8 => {
                let p0 = get(&state, ip + 1, pm0);
                let p1 = get(&state, ip + 2, pm1);
                let a2 = state[ip + 3] as usize;
                ip += 4;
                state[a2] = {
                    if p0 == p1 {
                        1
                    } else {
                        0
                    }
                };
            }
            99 => {
                break;
            }
            _ => {
                println!("illegal opcode {}: {} {} {}", ip, ins, pm0, pm1);
                panic!("illegal opcode");
            }
        }
    }

    output
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let state = result.unwrap().1;

    let result_a = run(state.clone(), vec![1]);
    let result_b = run(state, vec![5]);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
