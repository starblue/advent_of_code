use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map_res;
use nom::named;
use nom::opt;
use nom::recognize;
use nom::tag;
use nom::tuple;

use lowdim::p4d;
use lowdim::Point4d;

#[derive(Clone, Debug)]
enum Error {}

named!(int64<&str, i64>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit1)), FromStr::from_str)
);

named!(point<&str, Point4d>,
    do_parse!(
        x: int64 >>
        tag!(",") >>
        y: int64 >>
        tag!(",") >>
        z: int64 >>
        tag!(",") >>
        u: int64 >>
            (p4d(x, y, z, u))
    )
);

named!(points<&str, Vec<Point4d>>,
    many1!(
        do_parse!(
            p: point >>
            line_ending >>
                (p)
        )
    )
);

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
