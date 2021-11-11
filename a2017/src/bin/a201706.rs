use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    let (i, row) = separated_list1(tag("\t"), uint)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, row))
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

    let input = result.unwrap().1;
    // let mut sep = "";
    // for n in &input {
    //     print!("{}{}", sep, n);
    //     sep = "\t";
    // }
    // println!();

    let len = input.len();

    let mut state = input;
    let mut count = 0;
    let mut seen = HashMap::new();
    let loop_size;
    loop {
        if let Some(previous_count) = seen.get(&state) {
            loop_size = count - previous_count;
            break;
        } else {
            seen.insert(state.clone(), count);
        }

        // compute new state
        let mut i;
        let mut value;
        {
            let p = state
                .iter()
                .enumerate()
                .max_by(|&(i1, v1), &(i2, v2)| v1.cmp(v2).then(i2.cmp(&i1)))
                .unwrap();
            i = p.0;
            value = *p.1;
        }

        state[i] = 0;
        while value > 0 {
            i = (i + 1) % len;
            state[i] += 1;
            value -= 1;
        }
        count += 1;
    }
    let result_a = count;

    let result_b = loop_size;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
