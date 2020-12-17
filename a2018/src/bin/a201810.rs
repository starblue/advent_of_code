use std::collections::HashSet;
use std::i64;
use std::io;
use std::str::FromStr;

use nom::char;
use nom::digit;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::opt;
use nom::recognize;
use nom::tag;
use nom::tuple;
use nom::ws;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Vec2d {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug)]
struct Record {
    pos: Vec2d,
    v: Vec2d,
}

#[derive(Clone, Debug)]
enum Error {}

named!(int64<&str, i64>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit)), FromStr::from_str)
);

named!(
    vec2d<&str, Vec2d>,
    do_parse!(
        ws!(char!('<')) >>
            x: int64 >>
            ws!(char!(',')) >>
            y: int64 >>
            char!('>') >>
            (Vec2d { x, y }))
);

named!(
    record<&str, Record>,
    do_parse!(
        ws!(tag!("position=")) >>
            pos: vec2d >>
            ws!(tag!("velocity=")) >>
            v: vec2d >>
            (Record { pos, v }))
);

fn pos_at_t(r: &Record, t: i64) -> Vec2d {
    Vec2d {
        x: r.pos.x + t * r.v.x,
        y: r.pos.y + t * r.v.y,
    }
}

fn main() {
    let mut records = Vec::new();

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.trim().is_empty() {
            break;
        }

        // parse line
        let result = record(&line.trim_end());
        //println!("{:?}", result);

        let r = result.unwrap().1;
        records.push(r);
    }

    let mut t = 0;
    let mut last_d = i64::MAX;
    loop {
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        for r in &records {
            let pt = pos_at_t(r, t);
            min_x = min_x.min(pt.x);
            min_y = min_y.min(pt.y);
            max_x = max_x.max(pt.x);
            max_y = max_y.max(pt.y);
        }
        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let d = dx.min(dy);
        if d > last_d {
            let ps = records
                .iter()
                .map(|r| pos_at_t(r, t - 1))
                .collect::<HashSet<_>>();
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    print!(
                        "{}",
                        if ps.contains(&Vec2d { x, y }) {
                            "#"
                        } else {
                            "."
                        }
                    );
                }
                println!();
            }
            break;
        }
        last_d = d;

        t += 1;
    }

    println!("{}", t - 1);
}
