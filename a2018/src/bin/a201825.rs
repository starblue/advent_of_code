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
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::p4d;
use lowdim::Point4d;

use util::DisjointSets;

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
    Ok((i, p4d(x, y, z, u)))
}

fn input(i: &str) -> IResult<&str, Vec<Point4d>> {
    separated_list1(line_ending, point)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let points = result.unwrap().1;
    // for p in &points {
    //     println!("{:?}", p);
    // }

    let mut constellations = DisjointSets::new();
    for &p in &points {
        constellations.add(p);
    }
    let n = points.len();
    for i in 0..(n - 1) {
        let pi = points[i];
        for j in (i + 1)..n {
            let pj = points[j];
            if pi.distance_l1(pj) <= 3 {
                constellations.union(&pi, &pj);
            }
        }
    }
    let result_a = constellations.set_reprs().len();

    println!("a: {}", result_a);
}
