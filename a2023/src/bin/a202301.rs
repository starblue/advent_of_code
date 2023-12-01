use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

fn line(i: &str) -> IResult<&str, String> {
    let (i, cs) = many1(none_of("\n"))(i)?;
    Ok((i, cs.into_iter().collect::<String>()))
}

fn input(i: &str) -> IResult<&str, Vec<String>> {
    separated_list1(line_ending, line)(i)
}

fn calibration_value1(s: &str) -> util::Result<u32> {
    let mut first_digit = None;
    let mut last_digit = None;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            if first_digit == None {
                first_digit = Some(d);
            }
            last_digit = Some(d);
        }
    }
    let first_digit = first_digit.ok_or(util::runtime_error!("no first digit found"))?;
    let last_digit = last_digit.ok_or(util::runtime_error!("no last digit found"))?;
    Ok(first_digit * 10 + last_digit)
}

const DIGITS2: &[(u32, &str)] = &[
    (1, "1"),
    (2, "2"),
    (3, "3"),
    (4, "4"),
    (5, "5"),
    (6, "6"),
    (7, "7"),
    (8, "8"),
    (9, "9"),
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn calibration_value2(s: &str) -> util::Result<u32> {
    Ok({
        let mut first_digit = None;
        let mut last_digit = None;
        for i in 0..s.len() {
            for (value, pattern) in DIGITS2 {
                if s[i..].starts_with(pattern) {
                    if first_digit == None {
                        first_digit = Some(value);
                    }
                    last_digit = Some(value);
                }
            }
        }
        let first_digit = first_digit.ok_or(util::runtime_error!("no first digit found"))?;
        let last_digit = last_digit.ok_or(util::runtime_error!("no last digit found"))?;
        first_digit * 10 + last_digit
    })
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for line in &input {
    //     println!("{}", line);
    // }

    let result1 = input
        .iter()
        .map(|line| calibration_value1(line))
        .sum::<util::Result<u32>>()?;

    let result2 = input
        .iter()
        .map(|line| calibration_value2(line))
        .sum::<util::Result<u32>>()?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
