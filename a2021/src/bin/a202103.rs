use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

fn bit(i: &str) -> IResult<&str, i64> {
    alt((value(0, tag("0")), value(1, tag("1"))))(i)
}
fn number(i: &str) -> IResult<&str, Vec<i64>> {
    many1(bit)(i)
}
fn input(i: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, number)(i)
}

fn to_i64(bits: &[i64]) -> i64 {
    bits.iter().fold(0, |a, bit| 2 * a + bit)
}

fn bit_counts(numbers: &[Vec<i64>], i: usize) -> (i64, i64) {
    let mut count0 = 0;
    let mut count1 = 0;
    for c in numbers {
        if c[i] == 0 {
            count0 += 1;
        } else {
            count1 += 1;
        }
    }
    (count0, count1)
}

fn least_common_bit(numbers: &[Vec<i64>], i: usize) -> i64 {
    let (count0, count1) = bit_counts(numbers, i);
    match count0.cmp(&count1) {
        Ordering::Less => 0,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

fn most_common_bit(numbers: &[Vec<i64>], i: usize) -> i64 {
    let (count0, count1) = bit_counts(numbers, i);
    match count0.cmp(&count1) {
        Ordering::Less => 1,
        Ordering::Greater => 0,
        Ordering::Equal => 1,
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for number in &input {
    //     for bit in number {
    //         print!("{}", bit);
    //     }
    //     println!();
    // }

    // All numbers in the input have the same bit length.
    let len = input[0].len();

    let mut counts = HashMap::new();
    for number in &input {
        for (i, &bit) in number.iter().enumerate() {
            let entry = counts.entry((i, bit)).or_insert(0);
            *entry += 1;
        }
    }

    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();
    for i in 0..len {
        let count0 = counts[&(i, 0)];
        let count1 = counts[&(i, 1)];
        match count0.cmp(&count1) {
            Ordering::Less => {
                gamma_bits.push(1);
                epsilon_bits.push(0);
            }
            Ordering::Greater => {
                gamma_bits.push(0);
                epsilon_bits.push(1);
            }
            Ordering::Equal => {
                panic!("equal counts at position {}", i);
            }
        }
    }
    let gamma = to_i64(&gamma_bits);
    let epsilon = to_i64(&epsilon_bits);
    let result_a = gamma * epsilon;

    let mut ogr_candidates = input.clone();
    for i in 0..len {
        let bit = most_common_bit(&ogr_candidates, i);

        ogr_candidates = ogr_candidates
            .iter()
            .filter(|c| c[i] == bit)
            .cloned()
            .collect::<Vec<_>>();
        if ogr_candidates.len() == 1 {
            break;
        }
    }
    let ogr = to_i64(&ogr_candidates[0]);

    let mut csr_candidates = input;
    for i in 0..len {
        let bit = least_common_bit(&csr_candidates, i);

        csr_candidates = csr_candidates
            .iter()
            .filter(|c| c[i] == bit)
            .cloned()
            .collect::<Vec<_>>();
        if csr_candidates.len() == 1 {
            break;
        }
    }
    let csr = to_i64(&csr_candidates[0]);

    let result_b = ogr * csr;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
