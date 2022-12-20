use core::str::FromStr;

use std::io;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use util::runtime_error;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list0(line_ending, int)(i)
}

fn mix(moves: &[i64], indices: &mut Vec<usize>) -> util::Result<()> {
    let len = i64::try_from(moves.len())?;
    for (k, &m) in moves.iter().enumerate() {
        let i = indices
            .iter()
            .position(|&n| n == k)
            .ok_or(runtime_error!("number not found"))?;
        let i = i64::try_from(i)?;
        let j = (i + m).rem_euclid(len - 1);

        let i = usize::try_from(i)?;
        let j = usize::try_from(j)?;
        indices.remove(i);
        indices.insert(j, k);
    }
    Ok(())
}

fn coordinate_sum(moves: &[i64], indices: &[usize]) -> util::Result<i64> {
    let i = indices
        .iter()
        .position(|&n| moves[n] == 0)
        .ok_or(runtime_error!("zero not found"))?;
    let len = moves.len();
    let m1000 = moves[indices[(i + 1000) % len]];
    let m2000 = moves[indices[(i + 2000) % len]];
    let m3000 = moves[indices[(i + 3000) % len]];
    Ok(m1000 + m2000 + m3000)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for n in &input {
    //     println!("{}", n);
    // }

    let moves = input.clone();
    let mut indices = (0..input.len()).collect::<Vec<_>>();
    mix(&moves, &mut indices)?;
    let result1 = coordinate_sum(&moves, &indices)?;

    let key = 811_589_153;
    let moves = moves.into_iter().map(|m| m * key).collect::<Vec<_>>();
    let mut indices = (0..input.len()).collect::<Vec<_>>();
    for _ in 0..10 {
        mix(&moves, &mut indices)?;
    }
    let result2 = coordinate_sum(&moves, &indices)?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
