use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Reindeer {
    name: String,
    speed: i64,
    fly_duration: i64,
    rest_duration: i64,
}
impl Reindeer {
    fn cycle_duration(&self) -> i64 {
        self.fly_duration + self.rest_duration
    }
    fn distance(&self, t: i64) -> i64 {
        let q = t / self.cycle_duration();
        let r = t % self.cycle_duration();
        let total_fly_duration = q * self.fly_duration + r.min(self.fly_duration);
        self.speed * total_fly_duration
    }
}
impl fmt::Display for Reindeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} can fly {} km/s for {} seconds, but then must rest for {} seconds.",
            self.name, self.speed, self.fly_duration, self.rest_duration,
        )
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn reindeer(i: &str) -> IResult<&str, Reindeer> {
    let (i, name) = string(i)?;
    let (i, _) = tag(" can fly ")(i)?;
    let (i, speed) = uint(i)?;
    let (i, _) = tag(" km/s for ")(i)?;
    let (i, fly_duration) = uint(i)?;
    let (i, _) = tag(" seconds, but then must rest for ")(i)?;
    let (i, rest_duration) = uint(i)?;
    let (i, _) = tag(" seconds.")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Reindeer {
            name,
            speed,
            fly_duration,
            rest_duration,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Reindeer>> {
    many1(reindeer)(i)
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
    // for r in &input {
    //     println!("{}", r);
    // }

    let time_limit = 2503;

    let result_a = input.iter().map(|r| r.distance(time_limit)).max().unwrap();

    let mut table = HashMap::new();
    for t in 1..=time_limit {
        let mut max_distance = 0;
        let mut leading_reindeers = Vec::new();
        for r in &input {
            let distance = r.distance(t);
            match distance.cmp(&max_distance) {
                Ordering::Greater => {
                    max_distance = distance;
                    leading_reindeers.clear();
                    leading_reindeers.push(&r.name);
                }
                Ordering::Equal => {
                    leading_reindeers.push(&r.name);
                }
                Ordering::Less => {
                    // not leading, do nothing
                }
            }
        }

        for r in leading_reindeers {
            let entry = table.entry(r).or_insert(0);
            *entry += 1;
        }
    }
    let result_b = table.values().max().unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
