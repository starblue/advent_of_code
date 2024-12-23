use std::collections::HashMap;
use std::io;

use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn line(i: &str) -> IResult<&str, String> {
    let (i, s) = string(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, s))
}

fn input(i: &str) -> IResult<&str, Vec<String>> {
    many1(line)(i)
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn has_double_letter(s: &str) -> bool {
    let mut it = s.chars();
    if let Some(c) = it.next() {
        let mut previous_char = c;
        for c in it {
            if c == previous_char {
                return true;
            }
            previous_char = c;
        }
        false
    } else {
        false
    }
}

fn is_nice_1(s: &str) -> bool {
    (s.chars().filter(|&c| is_vowel(c)).count() >= 3)
        && has_double_letter(s)
        && !s.contains("ab")
        && !s.contains("cd")
        && !s.contains("pq")
        && !s.contains("xy")
}

fn has_letter_pair_twice(s: &str) -> bool {
    let mut counts = HashMap::new();

    let mut it = s.chars();
    if let Some(c) = it.next() {
        let mut previous_char = c;
        let mut previous_pair = None;
        for c in it {
            let pair = (previous_char, c);
            if Some(pair) != previous_pair {
                let count = counts.entry(pair).or_insert(0);
                *count += 1;
            }
            previous_char = c;
            previous_pair = Some(pair);
        }
    }
    counts.values().filter(|&&count| count >= 2).count() >= 1
}

fn cond_2_2(s: &str) -> bool {
    let cs = s.chars().collect::<Vec<_>>();
    cs.windows(3).filter(|w| w[0] == w[2]).count() >= 1
}

fn is_nice_2(s: &str) -> bool {
    has_letter_pair_twice(s) && cond_2_2(s)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;

    let result_a = input.iter().filter(|s| is_nice_1(s)).count();
    let result_b = input.iter().filter(|s| is_nice_2(s)).count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
