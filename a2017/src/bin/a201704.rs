use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

fn password(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn line(i: &str) -> IResult<&str, Vec<String>> {
    let (i, passwords) = separated_list1(tag(" "), password)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, passwords))
}

fn input(i: &str) -> IResult<&str, Vec<Vec<String>>> {
    many1(line)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;

    let mut count = 0;
    for line in &input {
        let mut words = HashSet::new();
        let mut valid = true;
        for w in line {
            if !words.contains(w) {
                words.insert(w);
            } else {
                valid = false;
            }
        }
        if valid {
            count += 1;
        }
    }
    let result_a = count;

    let mut count = 0;
    for line in &input {
        let mut words = HashSet::new();
        let mut valid = true;
        for w in line {
            let mut chars = w.chars().collect::<Vec<_>>();
            chars.sort();
            let w = chars.iter().collect::<String>();
            if !words.contains(&w) {
                words.insert(w);
            } else {
                valid = false;
            }
        }
        if valid {
            count += 1;
        }
    }
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
