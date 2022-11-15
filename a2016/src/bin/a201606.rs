use std::collections::HashMap;
use std::io;

use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

fn message(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn line(i: &str) -> IResult<&str, String> {
    let (i, message) = message(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, message))
}

fn input(i: &str) -> IResult<&str, Vec<String>> {
    many1(line)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for msg in &input {
    //     println!("{}", msg);
    // }

    let len = input.iter().map(|s| s.len()).max().unwrap();
    let mut counts = (0..len).map(|_| HashMap::new()).collect::<Vec<_>>();
    for msg in &input {
        for (i, c) in msg.chars().enumerate() {
            let map = &mut counts[i];
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }
    }
    let result_a = counts
        .iter()
        .map(|m| {
            m.iter()
                .max_by_key(|(_, count)| *count)
                .map(|(ch, _)| ch)
                .unwrap()
        })
        .collect::<String>();

    let result_b = counts
        .iter()
        .map(|m| {
            m.iter()
                .min_by_key(|(_, count)| *count)
                .map(|(ch, _)| ch)
                .unwrap()
        })
        .collect::<String>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
