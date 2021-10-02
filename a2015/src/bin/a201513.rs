use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Record {
    name: String,
    gain: i64,
    other: String,
}
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} would {} {} happiness units by sitting next to {}.",
            self.name,
            if self.gain >= 0 { "gain" } else { "lose" },
            self.gain.abs(),
            self.other
        )
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn happiness_gain(i: &str) -> IResult<&str, i64> {
    let (i, _) = tag("gain ")(i)?;
    let (i, gain) = uint(i)?;
    let (i, _) = tag(" happiness units")(i)?;
    Ok((i, gain))
}

fn happiness_lose(i: &str) -> IResult<&str, i64> {
    let (i, _) = tag("lose ")(i)?;
    let (i, loss) = uint(i)?;
    let (i, _) = tag(" happiness units")(i)?;
    Ok((i, -loss))
}

fn gain(i: &str) -> IResult<&str, i64> {
    alt((happiness_gain, happiness_lose))(i)
}

fn record(i: &str) -> IResult<&str, Record> {
    let (i, name) = string(i)?;
    let (i, _) = tag(" would ")(i)?;
    let (i, gain) = gain(i)?;
    let (i, _) = tag(" by sitting next to ")(i)?;
    let (i, other) = string(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Record { name, gain, other }))
}

fn input(i: &str) -> IResult<&str, Vec<Record>> {
    many1(record)(i)
}

pub fn factorial(n: usize) -> usize {
    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }
    result
}

pub fn nth_permutation<T: Copy>(es: &[T], n: usize) -> Vec<T> {
    let mut result = Vec::new();
    let mut es = es.to_vec();
    let mut n = n;
    while !es.is_empty() {
        let tail_len = es.len() - 1;
        let tail_perms_len = factorial(tail_len);
        let i = n / tail_perms_len;
        n %= tail_perms_len;
        let e = es.remove(i);
        result.push(e);
    }
    result
}

fn seating_gain<'map, 'a: 'map>(
    seating: &'a [&'map str],
    gains: &'map HashMap<(&'map str, &'map str), i64>,
) -> i64 {
    let mut gain = 0;
    for i in 0..(seating.len() - 1) {
        let p0 = seating[i];
        let p1 = seating[i + 1];
        gain += gains[&(p0, p1)];
    }
    gain += gains[&(seating[0], seating[seating.len() - 1])];
    gain
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
    //println!("{:#?}", input);
    // for r in &input {
    //     println!("{}", r);
    // }

    let mut people = HashSet::new();
    for r in &input {
        people.insert(&r.name[..]);
        people.insert(&r.other[..]);
    }
    let mut people = people.into_iter().collect::<Vec<_>>();
    people.sort();

    let mut gains = HashMap::new();
    for r in &input {
        let entry = gains.entry((&r.name[..], &r.other[..])).or_insert(0);
        *entry += r.gain;
        let entry = gains.entry((&r.other[..], &r.name[..])).or_insert(0);
        *entry += r.gain;
    }

    let seating0 = nth_permutation(&people, 0);
    let mut max_gain = seating_gain(&seating0[..], &gains);
    for n in 1..factorial(people.len()) {
        let seating = nth_permutation(&people, n);
        let gain = seating_gain(&seating[..], &gains);
        if gain > max_gain {
            max_gain = gain;
        }
    }
    let result_a = max_gain;

    people.push("self");
    for p in &people {
        gains.insert((&p[..], "self"), 0);
        gains.insert(("self", &p[..]), 0);
    }

    let seating0 = nth_permutation(&people, 0);
    let mut max_gain = seating_gain(&seating0[..], &gains);
    for n in 1..factorial(people.len()) {
        let seating = nth_permutation(&people, n);
        let gain = seating_gain(&seating[..], &gains);
        if gain > max_gain {
            max_gain = gain;
        }
    }
    let result_b = max_gain;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
