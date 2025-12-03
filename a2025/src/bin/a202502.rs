use core::str::FromStr;

use std::fmt;
use std::fmt::Display;
use std::io;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn uint(i: &str) -> IResult<&str, i128> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

#[derive(Clone, Copy, Debug)]
struct IdRange {
    min: i128,
    max: i128,
}
impl Display for IdRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

fn id_range(i: &str) -> IResult<&str, IdRange> {
    let (i, min) = uint(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = uint(i)?;
    Ok((i, IdRange { min, max }))
}

fn input(i: &str) -> IResult<&str, Vec<IdRange>> {
    separated_list1(char(','), id_range)(i)
}

fn moebius(n: u32) -> i128 {
    [0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1][n as usize]
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;

    // let mut sep = "";
    // for id_range in &input {
    //     print!("{}{}", sep, id_range);
    //     sep = ",";
    // }
    // println!();

    let mut sum = 0;
    for id_range in &input {
        for i in 1.. {
            // Check ids with `2 * i` digits.
            let divisor = 10_i128.pow(i) + 1;
            let min = id_range.min.max(10_i128.pow(2 * i - 1));
            let max = id_range.max.min(10_i128.pow(2 * i) - 1);
            if min > id_range.max {
                break;
            }
            if max >= min {
                let inv_min_multiple = (min + divisor - 1) / divisor;
                let inv_max_multiple = max / divisor;
                let inv_sum = divisor
                    * (inv_max_multiple - inv_min_multiple + 1)
                    * (inv_min_multiple + inv_max_multiple)
                    / 2;
                sum += inv_sum;
            }
        }
    }
    let result1 = sum;

    let mut sum = 0;
    for id_range in &input {
        let max_digits = id_range.max.ilog10() + 1;
        // `k` is the number of repetitions.
        for k in 2..=max_digits {
            for i in 1.. {
                let n = k * i;
                // Check ids with `n` digits.
                let divisor = (0..k).map(|j| 10_i128.pow(j * i)).sum::<i128>();
                let min = id_range.min.max(10_i128.pow(n - 1));
                let max = id_range.max.min(10_i128.pow(n) - 1);
                if min > id_range.max {
                    break;
                }
                if max >= min {
                    let inv_min_multiple = (min + divisor - 1) / divisor;
                    let inv_max_multiple = max / divisor;
                    let inv_sum = divisor
                        * (inv_max_multiple - inv_min_multiple + 1)
                        * (inv_min_multiple + inv_max_multiple)
                        / 2;
                    sum += -moebius(k) * inv_sum;
                }
            }
        }
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
