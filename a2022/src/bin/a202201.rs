use core::str::FromStr;

use std::io;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use util::runtime_error;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn item(i: &str) -> IResult<&str, i64> {
    let (i, n) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, n))
}

fn elf(i: &str) -> IResult<&str, Vec<i64>> {
    many1(item)(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, elf)(i)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for elf in &input {
    //     for item in elf {
    //         println!("{}", item);
    //     }
    //     println!();
    // }

    let mut elves = input
        .iter()
        .map(|elf| elf.iter().sum::<i64>())
        .collect::<Vec<_>>();
    elves.sort();
    elves.reverse();

    let result1 = elves
        .first()
        .ok_or_else(|| runtime_error!("input is empty"))?;

    let result2 = elves.iter().take(3).sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
