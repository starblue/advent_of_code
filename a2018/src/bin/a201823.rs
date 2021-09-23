use core::fmt;

use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::p3d;
use lowdim::Point3d;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Nanobot {
    pos: Point3d,
    r: i64,
}
impl fmt::Display for Nanobot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<{},{},{}>, r={}",
            self.pos.x(),
            self.pos.y(),
            self.pos.z(),
            self.r
        )
    }
}

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn nanobot(i: &str) -> IResult<&str, Nanobot> {
    let (i, _) = tag("pos=<")(i)?;
    let (i, x) = int64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = int64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, z) = int64(i)?;
    let (i, _) = tag(">, r=")(i)?;
    let (i, r) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Nanobot {
            pos: p3d(x, y, z),
            r,
        },
    ))
}

fn nanobots(i: &str) -> IResult<&str, Vec<Nanobot>> {
    many1(nanobot)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = nanobots(&input_data);
    //println!("{:?}", result);

    let bots = result.unwrap().1;
    // for b in &bots {
    //     println!("{:?}", b);
    // }

    let mut max_bot = bots[0];
    for b in &bots {
        if b.r > max_bot.r {
            max_bot = *b;
        }
    }

    let result_a = bots
        .iter()
        .filter(|b| b.pos.distance_l1(max_bot.pos) <= max_bot.r)
        .count();

    let p_min = p3d(std::i64::MIN, std::i64::MIN, std::i64::MIN);
    let p_max = p3d(std::i64::MAX, std::i64::MAX, std::i64::MAX);
    let min = bots.iter().map(|b| b.pos).fold(p_max, |m, p| m.min(p));
    let max = bots.iter().map(|b| b.pos).fold(p_min, |m, p| m.max(p));
    let d = max - min;
    println!("min: {:?}", min);
    println!("max: {:?}", max);
    println!("d: {:?}", d);

    let result_b = 0;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
