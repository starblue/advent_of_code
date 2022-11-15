use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

fn line(i: &str) -> IResult<&str, String> {
    let (i, cs) = many1(none_of("\n"))(i)?;
    Ok((i, cs.into_iter().collect::<String>()))
}

fn input(i: &str) -> IResult<&str, Vec<String>> {
    separated_list1(line_ending, line)(i)
}

fn char_score_1(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => {
            panic!("unexpected character {:?}", c);
        }
    }
}

fn line_score_1(line: &str) -> Option<i64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => {
                    panic!("internal error, found character {:?}", c);
                }
            });
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            if stack.pop() == Some(c) {
                // We popped the expected character, done.
            } else {
                // We found the first illegal character.
                return Some(char_score_1(c));
            }
        } else {
            panic!("unexpected character {:?}", c);
        }
    }
    None
}

fn char_score_2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => {
            panic!("unexpected character {:?}", c);
        }
    }
}

fn line_score_2(line: &str) -> Option<i64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => {
                    panic!("internal error, found character {:?}", c);
                }
            });
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            if stack.pop() == Some(c) {
                // We popped the expected character, done.
            } else {
                // We found an illegal character.
                return None;
            }
        } else {
            panic!("unexpected character {:?}", c);
        }
    }
    let mut score = 0;
    while let Some(c) = stack.pop() {
        score = 5 * score + char_score_2(c);
    }
    Some(score)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for line in &input {
    //     println!("{}", line);
    // }

    let result_a = input
        .iter()
        .filter_map(|line| line_score_1(line))
        .sum::<i64>();

    let mut scores = input
        .iter()
        .filter_map(|line| line_score_2(line))
        .collect::<Vec<_>>();
    scores.sort();
    let result_b = scores[(scores.len() - 1) / 2];

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
