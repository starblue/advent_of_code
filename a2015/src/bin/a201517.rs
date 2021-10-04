use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn line(i: &str) -> IResult<&str, i64> {
    let (i, n) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, n))
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    many1(line)(i)
}

/// Number of combinations getting a total using the given containers
fn count1<'a>(table: &mut HashMap<(&'a [i64], i64), i64>, cs: &'a [i64], total: i64) -> i64 {
    if let Some(&result) = table.get(&(cs, total)) {
        result
    } else {
        let result = {
            if total < 0 {
                0
            } else if total == 0 {
                1
            } else if cs.is_empty() {
                0
            } else {
                let len1 = cs.len() - 1;
                let c = cs[len1];
                let count0 = count1(table, &cs[..len1], total);
                let count1 = count1(table, &cs[..len1], total - c);
                count0 + count1
            }
        };
        table.insert((cs, total), result);
        result
    }
}

/// Number of combinations getting a total using the given containers
fn count2<'a>(
    table: &mut HashMap<(&'a [i64], usize, i64), i64>,
    cs: &'a [i64],
    used: usize,
    total: i64,
) -> i64 {
    if let Some(&result) = table.get(&(cs, used, total)) {
        result
    } else {
        let result = {
            if total < 0 {
                0
            } else if total == 0 {
                1
            } else if used == 0 || used > cs.len() {
                0
            } else {
                let len1 = cs.len() - 1;
                let c = cs[len1];
                let count0 = count2(table, &cs[..len1], used, total);
                let count1 = count2(table, &cs[..len1], used - 1, total - c);
                count0 + count1
            }
        };
        table.insert((cs, used, total), result);
        result
    }
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
    // for n in &input {
    //     println!("{}", n);
    // }

    let eggnog_volume = 150;
    let result_a = count1(&mut HashMap::new(), &input[..], eggnog_volume);

    let mut used = 0;
    let mut count;
    loop {
        count = count2(&mut HashMap::new(), &input[..], used, eggnog_volume);
        if count != 0 {
            break;
        }
        used += 1;
    }
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
