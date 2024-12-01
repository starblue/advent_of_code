use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn line(i: &str) -> IResult<&str, (usize, usize)> {
    let (i, n0) = uint(i)?;
    let (i, _) = space1(i)?;
    let (i, n1) = uint(i)?;
    Ok((i, (n0, n1)))
}

fn input(i: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(line_ending, line)(i)
}

fn delta(n0: usize, n1: usize) -> usize {
    if n0 >= n1 {
        n0 - n1
    } else {
        n1 - n0
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for line in &input {
    //     println!("{}   {}", line.0, line.1);
    // }

    let mut list0 = input.iter().map(|&(n, _)| n).collect::<Vec<_>>();
    let mut list1 = input.iter().map(|&(_, n)| n).collect::<Vec<_>>();
    list0.sort();
    list1.sort();

    let result1 = list0
        .iter()
        .zip(list1.iter())
        .map(|(&n0, &n1)| delta(n0, n1))
        .sum::<usize>();

    let mut counts0 = HashMap::new();
    for n in list0 {
        let entry = counts0.entry(n).or_insert(0);
        *entry += 1;
    }
    let mut counts1 = HashMap::new();
    for n in list1 {
        let entry = counts1.entry(n).or_insert(0);
        *entry += 1;
    }

    let result2 = counts0
        .iter()
        .map(|(n, count0)| n * count0 * counts1.get(n).unwrap_or(&0))
        .sum::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
