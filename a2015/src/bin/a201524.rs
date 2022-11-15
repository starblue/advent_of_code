use std::cmp;
use std::io;
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

/// Push all subsequences with sum `target_sum` into `gs`.
fn groups(
    gs: &mut Vec<Vec<i64>>,
    target_sum: i64,
    group: &mut Vec<i64>,
    group_sum: i64,
    rest: &[i64],
) {
    if !rest.is_empty() {
        let item = rest[0];
        let new_rest = &rest[1..];

        // try not including this item
        groups(gs, target_sum, group, group_sum, new_rest);

        let new_group_sum = group_sum + item;
        if new_group_sum <= target_sum {
            // try including this item

            group.push(item);
            if new_group_sum == target_sum {
                gs.push(group.clone());
            } else {
                groups(gs, target_sum, group, new_group_sum, new_rest);
            }
            group.pop();
        }
    }
}

fn entanglement(group: &[i64]) -> i128 {
    group.iter().map(|&item| i128::from(item)).product::<i128>()
}

fn group_cmp(g0: &Vec<i64>, g1: &Vec<i64>) -> cmp::Ordering {
    g0.len()
        .cmp(&g1.len())
        .then(entanglement(g0).cmp(&entanglement(g1)))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{:#?}", input);
    // for (i, n) in input.iter().enumerate() {
    //     println!("{}: {}", i, n);
    // }

    let total_weight = input.iter().sum::<i64>();

    let group_weight = total_weight / 3;
    let mut gs = Vec::new();
    groups(&mut gs, group_weight, &mut Vec::new(), 0, &input);
    // Sort groups according to Santa's preferences.
    gs.sort_by(group_cmp);

    // Find the first group where the rest can be split up into two groups.
    // It turns out that the very first one can be split in 25162 ways.
    let mut group = None;
    for g in gs.iter() {
        let rest = input
            .iter()
            .filter(|item| !g.contains(item))
            .cloned()
            .collect::<Vec<_>>();
        let mut rs = Vec::new();
        groups(&mut rs, group_weight, &mut Vec::new(), 0, &rest[..]);
        // println!("Rest splits: {}", rs.len());
        if !rs.is_empty() {
            group = Some(g);
            break;
        }
    }
    let result_a = entanglement(group.unwrap());

    let group_weight = total_weight / 4;
    let mut gs = Vec::new();
    groups(&mut gs, group_weight, &mut Vec::new(), 0, &input);
    // Sort groups according to Santa's preferences.
    gs.sort_by(group_cmp);
    // We are lazy and just assume that for the first group
    // the rest can be split into three groups of equal weight.
    // It turns out to be correct, unsurprisingly.
    let result_b = entanglement(&gs[0]);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
