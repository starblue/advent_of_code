use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vector;

#[derive(Clone, Copy, Debug)]
struct VentLine {
    p0: Point2d,
    p1: Point2d,
}
impl fmt::Display for VentLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let p0 = self.p0;
        let p1 = self.p1;
        write!(f, "{},{} -> {},{}", p0.x(), p0.y(), p1.x(), p1.y())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn point(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = int(i)?;
    Ok((i, p2d(x, y)))
}

fn vent_line(i: &str) -> IResult<&str, VentLine> {
    let (i, p0) = point(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, p1) = point(i)?;
    Ok((i, VentLine { p0, p1 }))
}

fn input(i: &str) -> IResult<&str, Vec<VentLine>> {
    separated_list1(line_ending, vent_line)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for vent_line in &input {
    //     println!("{}", vent_line);
    // }

    let mut counts = HashMap::new();
    for vent_line in &input {
        let p0 = vent_line.p0;
        let p1 = vent_line.p1;
        if p0.x() == p1.x() || p0.y() == p1.y() {
            let bbox = BBox2d::from_corners(p0, p1);
            for p in bbox.iter() {
                let e = counts.entry(p).or_insert(0);
                *e += 1;
            }
        }
    }
    let result_a = counts.values().filter(|&&c| c >= 2).count();

    let mut counts = HashMap::new();
    for vent_line in &input {
        let p0 = vent_line.p0;
        let p1 = vent_line.p1;

        let dv = p1 - p0;
        let dxa = dv.x().abs();
        let dya = dv.y().abs();
        assert!(dxa == 0 || dya == 0 || dxa == dya);

        let s = dv.signum();
        let n = dxa.max(dya);
        for i in 0..=n {
            let p = p0 + i * s;
            let e = counts.entry(p).or_insert(0);
            *e += 1;
        }
    }
    let result_b = counts.values().filter(|&&c| c >= 2).count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
