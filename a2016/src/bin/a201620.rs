use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Range {
    min: i64,
    max: i64,
}
impl Range {
    fn len(&self) -> i64 {
        self.max - self.min + 1
    }
}
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}-{}", self.min, self.max)
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn range(i: &str) -> IResult<&str, Range> {
    let (i, min) = uint(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, max) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Range { min, max }))
}

fn input(i: &str) -> IResult<&str, Vec<Range>> {
    many1(range)(i)
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
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for range in &input {
    //     println!("{}", range);
    // }

    // Store ranges indexed by max.
    let mut filter: BTreeMap<i64, Range> = BTreeMap::new();
    for r in &input {
        // bounds of new interval to insert
        let mut min = r.min;
        let mut max = r.max;
        let mut to_remove = Vec::new();
        for (&fr_max, &fr) in filter.range((min - 1)..) {
            if max + 1 < fr.min {
                // There is a gap between r and fr.
                // This and any following ranges in the filter
                // will not be modified, so stop here.
                break;
            } else {
                // ranges r and fr are consecutive, merge them
                to_remove.push(fr_max);
                min = min.min(fr.min);
                max = max.max(fr.max);
            }
        }
        for k in to_remove {
            filter.remove(&k);
        }
        filter.insert(max, Range { min, max });
    }
    let result_a = filter.iter().next().unwrap().1.max + 1;

    let mut allowed = 0x1_0000_0000;
    for r in filter.values() {
        allowed -= r.len();
    }
    let result_b = allowed;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
