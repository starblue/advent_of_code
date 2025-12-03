use std::fmt;
use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Battery {
    joltage: u8,
}
impl Battery {
    fn joltage(&self) -> u64 {
        u64::from(self.joltage)
    }
}
impl Battery {
    fn to_char(self) -> char {
        char::from(self.joltage + b'0')
    }
}
impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn battery(i: &str) -> IResult<&str, Battery> {
    let (i, c) = one_of("123456789")(i)?;
    let joltage = u8::try_from(c.to_digit(10).unwrap()).unwrap();
    Ok((i, Battery { joltage }))
}

fn row(i: &str) -> IResult<&str, Vec<Battery>> {
    many1(battery)(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vec<Battery>>> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    Ok((i, rows))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for line in &input {
    //     for battery in line {
    //         print!("{}", battery);
    //     }
    //     println!();
    // }

    let mut sum = 0;
    for line in &input {
        let mut max_joltage = 0;
        let mut max_first_battery = 0;
        for battery in line {
            max_joltage = max_joltage.max(10 * max_first_battery + battery.joltage());
            max_first_battery = max_first_battery.max(battery.joltage());
        }
        sum += max_joltage;
    }
    let result_1 = sum;

    let battery_count = 12;

    let mut sum = 0;
    for line in &input {
        // The maximal joltage so far formed by `i` digits.
        let mut max_joltages = vec![0; battery_count + 1];
        for battery in line {
            for i in (1..=battery_count).rev() {
                max_joltages[i] = max_joltages[i].max(10 * max_joltages[i - 1] + battery.joltage());
            }
        }
        sum += max_joltages[battery_count];
    }
    let result_2 = sum;

    println!("a: {}", result_1);
    println!("b: {}", result_2);
}
