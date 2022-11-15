use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::str::FromStr;

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
struct Distance {
    location0: String,
    location1: String,
    distance: i64,
}
impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} to {} = {}",
            self.location0, self.location1, self.distance
        )
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn line(i: &str) -> IResult<&str, Distance> {
    let (i, location0) = string(i)?;
    let (i, _) = tag(" to ")(i)?;
    let (i, location1) = string(i)?;
    let (i, _) = tag(" = ")(i)?;
    let (i, distance) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Distance {
            location0,
            location1,
            distance,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Distance>> {
    many1(line)(i)
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

fn path_cost<'map, 'path: 'map>(
    path: &'path [&'map str],
    distances: &'map HashMap<(&'map str, &'map str), i64>,
) -> i64 {
    let mut cost = 0;
    for i in 0..(path.len() - 1) {
        let loc0 = path[i];
        let loc1 = path[i + 1];
        cost += distances[&(loc0, loc1)];
    }
    cost
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    //println!("{:#?}", input);
    // for d in &input {
    //     println!("{}", d);
    // }

    let mut locations = HashSet::new();
    for d in &input {
        locations.insert(&d.location0[..]);
        locations.insert(&d.location1[..]);
    }
    let mut locations = locations.into_iter().collect::<Vec<_>>();
    locations.sort();

    let mut distances = HashMap::new();
    for d in &input {
        distances.insert((&d.location0[..], &d.location1[..]), d.distance);
        distances.insert((&d.location1[..], &d.location0[..]), d.distance);
    }

    let path0 = nth_permutation(&locations, 0);
    let mut min_cost = path_cost(&path0[..], &distances);
    for n in 1..factorial(locations.len()) {
        let path = nth_permutation(&locations, n);
        let cost = path_cost(&path[..], &distances);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    let result_a = min_cost;

    let path0 = nth_permutation(&locations, 0);
    let mut max_cost = path_cost(&path0[..], &distances);
    for n in 1..factorial(locations.len()) {
        let path = nth_permutation(&locations, n);
        let cost = path_cost(&path[..], &distances);
        if cost > max_cost {
            max_cost = cost;
        }
    }
    let result_b = max_cost;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
