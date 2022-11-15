use core::str::FromStr;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), int)(i)
}

fn simulate(timers: &[i64], steps: usize) -> usize {
    let mut counts = (0..=8)
        .map(|t0| timers.iter().filter(|&&t1| t1 == t0).count())
        .collect::<Vec<_>>();
    for _ in 0..steps {
        let mut new_counts = Vec::new();
        for t in 0..=8 {
            let new_count = {
                if t == 6 {
                    counts[0] + counts[7]
                } else if t == 8 {
                    counts[0]
                } else {
                    counts[t + 1]
                }
            };
            new_counts.push(new_count);
        }
        counts = new_counts;
    }
    counts.into_iter().sum::<usize>()
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // let mut sep = "";
    // for n in &input {
    //     print!("{}{}", sep, n);
    //     sep = ",";
    // }
    // println!();

    let result_a = simulate(&input, 80);
    let result_b = simulate(&input, 256);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
