use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, uint)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // let mut sep = "";
    // for n in &input {
    //     print!("{}{}", sep, n);
    //     sep = " ";
    // }
    // println!();

    let mut stones = input.clone();
    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let len = stone.ilog10() + 1;
                if len % 2 == 0 {
                    let pow10 = 10_i64.pow(len / 2);
                    new_stones.push(stone / pow10);
                    new_stones.push(stone % pow10);
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        stones = new_stones;
    }
    let result1 = stones.len();

    let mut stone_counts = HashMap::new();
    for &stone in &input {
        let entry = stone_counts.entry(stone).or_insert(0);
        *entry += 1;
    }
    for _ in 0..75 {
        let mut new_stone_counts = HashMap::new();
        for (stone, count) in stone_counts {
            if stone == 0 {
                let entry = new_stone_counts.entry(1).or_insert(0);
                *entry += count;
            } else {
                let len = stone.ilog10() + 1;
                if len % 2 == 0 {
                    let pow10 = 10_i64.pow(len / 2);
                    let entry0 = new_stone_counts.entry(stone / pow10).or_insert(0);
                    *entry0 += count;
                    let entry1 = new_stone_counts.entry(stone % pow10).or_insert(0);
                    *entry1 += count;
                } else {
                    let entry = new_stone_counts.entry(stone * 2024).or_insert(0);
                    *entry += count;
                }
            }
        }
        stone_counts = new_stone_counts;
    }
    let result2 = stone_counts.values().sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
