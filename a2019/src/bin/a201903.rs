use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::alt;
use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::separated_list1;
use nom::tag;
use nom::value;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug)]
struct Link {
    dir: Vec2d,
    steps: usize,
}

named!(
    uint<&str, usize>,
    map_res!(digit1, FromStr::from_str)
);

named!(dir<&str, Vec2d>,
    alt!(
        value!(v2d(1, 0), char!('R')) |
        value!(v2d(0, 1), char!('U')) |
        value!(v2d(-1, 0), char!('L')) |
        value!(v2d(0, -1), char!('D'))
    )
);

named!(link<&str, Link>,
    do_parse!(
        dir: dir >>
        steps: uint >> (Link { dir, steps })
    )
);

named!(
    path<&str, Vec<Link>>,
    separated_list1!(tag!(","), link)
);

named!(
    input<&str, (Vec<Link>,Vec<Link>)>,
    do_parse!(
        p0: path >>
        line_ending >>
        p1: path >> ((p0, p1))
    )
);

fn positions(path: Vec<Link>) -> HashMap<Point2d, i64> {
    let mut ps = HashMap::new();
    let mut p = p2d(0, 0);
    let mut d = 0;
    for Link { dir, steps } in path {
        for _ in 0..steps {
            p += dir;
            d += 1;
            ps.insert(p, d);
        }
    }
    ps
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

    let (p0, p1) = result.unwrap().1;

    let pm0 = positions(p0);
    let pm1 = positions(p1);
    let ps0 = pm0.keys().collect::<HashSet<_>>();
    let ps1 = pm1.keys().collect::<HashSet<_>>();

    let mut min_dist_a = std::i64::MAX;
    let mut min_dist_b = std::i64::MAX;
    for p in ps0.intersection(&ps1) {
        let dist_a = p.x().abs() + p.y().abs();
        if dist_a < min_dist_a {
            min_dist_a = dist_a;
        }
        let dist_b = pm0[p] + pm1[p];
        if dist_b < min_dist_b {
            min_dist_b = dist_b;
        }
    }

    let result_a = min_dist_a;
    let result_b = min_dist_b;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
