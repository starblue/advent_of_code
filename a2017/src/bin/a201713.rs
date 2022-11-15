use std::fmt;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Scanner {
    depth: u32,
    range: u32,
}
impl Scanner {
    fn cycle_size(&self) -> u32 {
        2 * (self.range - 1)
    }
    fn severity(&self) -> u32 {
        self.depth * self.range
    }
}
impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.depth, self.range)
    }
}

fn uint(i: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn scanner(i: &str) -> IResult<&str, Scanner> {
    let (i, depth) = uint(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, range) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Scanner { depth, range }))
}

fn input(i: &str) -> IResult<&str, Vec<Scanner>> {
    many1(scanner)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for scanner in &input {
    //     println!("{}", scanner);
    // }

    let result_a = input
        .iter()
        .map(|s| {
            if s.depth % s.cycle_size() == 0 {
                s.severity()
            } else {
                0
            }
        })
        .sum::<u32>();

    let mut t = 0;
    while input.iter().any(|s| (t + s.depth) % s.cycle_size() == 0) {
        t += 1;
    }
    let result_b = t;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
