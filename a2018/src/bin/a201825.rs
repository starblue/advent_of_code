use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point4d {
    x: i64,
    y: i64,
    z: i64,
    u: i64,
}

fn manhatten_distance(p1: &Point4d, p2: &Point4d) -> i64 {
    let dx = (p1.x - p2.x).abs();
    let dy = (p1.y - p2.y).abs();
    let dz = (p1.z - p2.z).abs();
    let du = (p1.u - p2.u).abs();
    dx + dy + dz + du
}

#[derive(Clone, Debug)]
enum Error {}

named!(int64<&str, i64>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit)), FromStr::from_str)
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
            (Point4d { x, y, z, u })
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

impl Point4d {
    fn neighbour(&self, other: &Point4d) -> bool {
        manhatten_distance(self, other) <= 3
    }
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
            if points[i].neighbour(&points[j]) {
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
