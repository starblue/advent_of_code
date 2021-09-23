use std::collections::HashSet;
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

use lowdim::p4d;
use lowdim::Point4d;

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn point(i: &str) -> IResult<&str, Point4d> {
    let (i, x) = int64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = int64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, z) = int64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, u) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, p4d(x, y, z, u)))
}

fn points(i: &str) -> IResult<&str, Vec<Point4d>> {
    many1(point)(i)
}

fn find(reprs: &mut Vec<usize>, i: usize) -> usize {
    let mut i = i;
    while reprs[i] != i {
        let prev = i;
        i = reprs[i];
        reprs[prev] = reprs[i];
    }
    i
}

fn union(reprs: &mut Vec<usize>, i: usize, j: usize) {
    let i_repr = find(reprs, i);
    let j_repr = find(reprs, j);
    if i_repr != j_repr {
        reprs[j_repr] = i_repr;
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = points(&input_data);
    //println!("{:?}", result);

    let points = result.unwrap().1;
    // for p in &points {
    //     println!("{:?}", p);
    // }

    let n = points.len();
    let mut reprs = (0..n).collect::<Vec<_>>();
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if points[i].distance_l1(points[j]) <= 3 {
                union(&mut reprs, i, j);
            }
        }
    }

    let mut constellations = HashSet::new();
    for i in 0..n {
        let i_repr = find(&mut reprs, i);
        constellations.insert(i_repr);
    }

    println!("1: {}", constellations.len());
    println!("2: {}", 0);
}
