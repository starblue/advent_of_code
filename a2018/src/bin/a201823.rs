use core::fmt;

use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::*;

use gamedim::p3d;
use gamedim::Point3d;
use gamedim::Vector;

fn manhatten_distance(p1: Point3d, p2: Point3d) -> i64 {
    (p1 - p2).norm_l1()
}

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
impl Nanobot {
    fn in_range(&self, p: Point3d) -> bool {
        manhatten_distance(self.pos, p) <= self.r
    }
}

#[derive(Clone, Debug)]
enum Error {}

named!(int64<&str, i64>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit)), FromStr::from_str)
);

named!(nanobot<&str, Nanobot>,
    do_parse!(
        tag!("pos=<") >>
        x: int64 >>
        tag!(",") >>
        y: int64 >>
        tag!(",") >>
        z: int64 >>
        tag!(">, r=") >>
        r: int64 >>
            (Nanobot { pos: p3d(x, y, z), r })
    )
);

named!(nanobots<&str, Vec<Nanobot>>,
    many1!(
        do_parse!(
            bot: nanobot >>
            line_ending >>
                (bot)
        )
    )
);

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
        .filter(|b| manhatten_distance(b.pos, max_bot.pos) <= max_bot.r)
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
