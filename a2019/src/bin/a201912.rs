use core::iter::repeat;

use std::collections::HashMap;
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

use num::integer::lcm;

use lowdim::p3d;
use lowdim::v3d;
use lowdim::Point3d;
use lowdim::Vec3d;
use lowdim::Vector;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn line(i: &str) -> IResult<&str, Point3d> {
    let (i, _) = tag("<x=")(i)?;
    let (i, x) = int(i)?;
    let (i, _) = tag(", y=")(i)?;
    let (i, y) = int(i)?;
    let (i, _) = tag(", z=")(i)?;
    let (i, z) = int(i)?;
    let (i, _) = tag(">")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, p3d(x, y, z)))
}

fn input(i: &str) -> IResult<&str, Vec<Point3d>> {
    many1(line)(i)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ps: Vec<Point3d>,
    vs: Vec<Vec3d>,
}
impl State {
    fn new(ps: &[Point3d]) -> State {
        State {
            ps: ps.to_owned(),
            vs: repeat(v3d(0, 0, 0)).take(ps.len()).collect::<Vec<_>>(),
        }
    }
    fn step(&mut self) {
        // apply gravity
        for i in 0..self.ps.len() {
            let p0 = self.ps[i];
            let mut a = v3d(0, 0, 0);
            for p1 in self.ps.iter() {
                a += (p1 - p0).signum();
            }
            self.vs[i] += a;
        }
        // apply velocity
        for i in 0..self.vs.len() {
            self.ps[i] += self.vs[i];
        }
    }
    #[allow(unused)]
    fn dump(&self) {
        for i in 0..self.ps.len() {
            let p = self.ps[i];
            let v = self.vs[i];
            print!(
                "{:1}({:4},{:4},{:4}) ({:4},{:4},{:4}) / ",
                i,
                p.x(),
                p.y(),
                p.z(),
                v.x(),
                v.y(),
                v.z()
            );
        }
        println!();
    }
    fn energy(&self) -> i64 {
        let p0 = p3d(0, 0, 0);
        let mut energy = 0;
        for i in 0..self.ps.len() {
            let potential_energy = (self.ps[i] - p0).norm_l1();
            let kinetic_energy = self.vs[i].norm_l1();
            energy += potential_energy * kinetic_energy;
        }
        energy
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State1 {
    ps: Vec<i64>,
    vs: Vec<i64>,
}
impl State1 {
    fn new(ps: &[i64]) -> State1 {
        State1 {
            ps: ps.to_owned(),
            vs: repeat(0).take(ps.len()).collect::<Vec<_>>(),
        }
    }
    fn step(&mut self) {
        // apply gravity
        for i in 0..self.ps.len() {
            let mut a = 0;
            for j in 0..self.ps.len() {
                a += (self.ps[j] - self.ps[i]).signum();
            }
            self.vs[i] += a;
        }
        // apply velocity
        for i in 0..self.vs.len() {
            self.ps[i] += self.vs[i];
        }
    }
}

fn detect_loop(ps: Vec<i64>) -> (i64, i64) {
    let mut state = State1::new(&ps);
    let mut first_seen = HashMap::new();
    let mut count = 0;
    loop {
        if let Some(&c) = first_seen.get(&state) {
            return (c, count);
        }
        first_seen.insert(state.clone(), count);
        state.step();
        count += 1;
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
    let result = input(&input_data);
    //println!("{:?}", result);

    let ps = result.unwrap().1;
    //println!("{:?}", s);

    let mut state_a = State::new(&ps);
    for _i in 0..1000 {
        state_a.step();
    }

    // detect loop separately per coordinate
    let xs = ps.iter().map(|p| p.x()).collect::<Vec<_>>();
    let (c0x, c1x) = detect_loop(xs);
    let ys = ps.iter().map(|p| p.y()).collect::<Vec<_>>();
    let (c0y, c1y) = detect_loop(ys);
    let zs = ps.iter().map(|p| p.z()).collect::<Vec<_>>();
    let (c0z, c1z) = detect_loop(zs);

    assert_eq!(0, c0x);
    assert_eq!(0, c0y);
    assert_eq!(0, c0z);

    let result_a = state_a.energy();
    let result_b = lcm(c1x, lcm(c1y, c1z));
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
