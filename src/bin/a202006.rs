use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::alpha;
use nom::do_parse;
use nom::line_ending;
use nom::many1;
use nom::map;
use nom::named;
use nom::recognize;

named!(answers<&str, String>,
    map!(recognize!(alpha), String::from)
);
named!(person<&str, String>,
    do_parse!(
        answers: answers >>
        line_ending >> (answers)
    )
);
named!(group<&str, Vec<String>>,
    do_parse!(
        persons: many1!(person) >>
        line_ending >> (persons)
    )
);
named!(
    input<&str, Vec<Vec<String>>>,
    many1!(group)
);

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
