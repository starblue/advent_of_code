use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

fn answers(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn person(i: &str) -> IResult<&str, String> {
    let (i, answers) = answers(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, answers))
}

fn group(i: &str) -> IResult<&str, Vec<String>> {
    let (i, persons) = many1(person)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, persons))
}

fn input(i: &str) -> IResult<&str, Vec<Vec<String>>> {
    many1(group)(i)
}

fn answer_set(s: &str) -> HashSet<char> {
    let mut result = HashSet::new();
    for answer in s.chars() {
        result.insert(answer);
    }
    result
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let groups = result.unwrap().1;

    let mut sum = 0;
    for group in &groups {
        let mut answers = HashSet::new();
        for person in group {
            for answer in person.chars() {
                answers.insert(answer);
            }
        }
        sum += answers.len();
    }
    let result_a = sum;

    let mut sum = 0;
    for group in &groups {
        let mut answers = answer_set(&group[0]);
        for person in group.iter().skip(1) {
            answers = answers
                .intersection(&answer_set(&person))
                .copied()
                .collect::<HashSet<_>>();
        }
        sum += answers.len();
    }
    let result_b = sum;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
