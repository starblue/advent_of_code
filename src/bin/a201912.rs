use core::iter::repeat;

use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::*;

use num::integer::lcm;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

named!(
    int32<&str, i32>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit)), FromStr::from_str)
);

named!(
    line<&str, Vec3d>,
    do_parse!(
        tag!("<x=") >>
        x: int32 >> 
        tag!(", y=") >>
        y: int32 >> 
        tag!(", z=") >>
        z: int32 >> 
        tag!(">") >>
        line_ending >> (Vec3d{ x, y, z })
    )
);

named!(
    input<&str, Vec<Vec3d>>,
    many1!(line)
);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ps: Vec<Vec3d>,
    vs: Vec<Vec3d>,
}
impl State {
    fn new(ps: &Vec<Vec3d>) -> State {
        State {
            ps: ps.to_owned(),
            vs: repeat(Vec3d { x: 0, y: 0, z: 0 })
                .take(ps.len())
                .collect::<Vec<_>>(),
        }
    }
    fn step(&mut self) {
        // apply gravity
        for i in 0..self.ps.len() {
            let mut a = Vec3d { x: 0, y: 0, z: 0 };
            for j in 0..self.ps.len() {
                a.x += (self.ps[j].x - self.ps[i].x).signum();
                a.y += (self.ps[j].y - self.ps[i].y).signum();
                a.z += (self.ps[j].z - self.ps[i].z).signum();
            }
            self.vs[i].x += a.x;
            self.vs[i].y += a.y;
            self.vs[i].z += a.z;
        }
        // apply velocity
        for i in 0..self.vs.len() {
            self.ps[i].x += self.vs[i].x;
            self.ps[i].y += self.vs[i].y;
            self.ps[i].z += self.vs[i].z;
        }
    }
    #[allow(unused)]
    fn dump(&self) {
        for i in 0..self.ps.len() {
            let p = self.ps[i];
            let v = self.vs[i];
            print!(
                "{:1}({:4},{:4},{:4}) ({:4},{:4},{:4}) / ",
                i, p.x, p.y, p.z, v.x, v.y, v.z
            );
        }
        println!();
    }
    fn energy(&self) -> i32 {
        let mut energy = 0;
        for i in 0..self.ps.len() {
            let potential_energy = self.ps[i].x.abs() + self.ps[i].y.abs() + self.ps[i].z.abs();
            let kinetic_energy = self.vs[i].x.abs() + self.vs[i].y.abs() + self.vs[i].z.abs();
            energy += potential_energy * kinetic_energy;
        }
        energy
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State1 {
    ps: Vec<i32>,
    vs: Vec<i32>,
}
impl State1 {
    fn new(ps: &Vec<i32>) -> State1 {
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

fn detect_loop(ps: Vec<i32>) -> (i64, i64) {
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
    input_data.push_str("\n");

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
    let xs = ps.iter().map(|p| p.x).collect::<Vec<_>>();
    let (c0x, c1x) = detect_loop(xs);
    let ys = ps.iter().map(|p| p.y).collect::<Vec<_>>();
    let (c0y, c1y) = detect_loop(ys);
    let zs = ps.iter().map(|p| p.z).collect::<Vec<_>>();
    let (c0z, c1z) = detect_loop(zs);

    assert_eq!(0, c0x);
    assert_eq!(0, c0y);
    assert_eq!(0, c0z);

    let result_a = state_a.energy();
    let result_b = lcm(c1x, lcm(c1y, c1z));
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
