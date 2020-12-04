use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::char;
use nom::digit;
use nom::do_parse;
use nom::line_ending;
use nom::map_res;
use nom::named;
use nom::separated_pair;
use nom::ws;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Record {
    id: i32,
    pos: Point,
    size: (i32, i32),
}

#[derive(Clone, Debug)]
enum Error {}

named!(int32<&str, i32>,
    map_res!(digit, FromStr::from_str)
);

named!(id<&str, i32>, do_parse!(char!('#') >> id: int32 >> (id) ));

named!(
    position<&str, Point>,
    do_parse!(x: int32 >> char!(',') >> y: int32 >> (Point { x, y }))
);

named!(size<&str, (i32, i32)>, separated_pair!(int32, char!('x'), int32));

named!(
    record<&str, Record>,
    do_parse!(
        id: id
            >> ws!(char!('@'))
            >> pos: position
            >> ws!(char!(':'))
            >> size: size
            >> line_ending
            >> (Record { id, pos, size })
    )
);

fn main() {
    let mut records = Vec::new();

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.is_empty() {
            break;
        }

        // parse line
        let result = record(&line);
        // println!("{:?}", result);

        let r = result.unwrap().1;
        records.push(r);
    }

    let mut fabric = repeat(repeat(0).take(1000).collect::<Vec<u32>>())
        .take(1000)
        .collect::<Vec<_>>();
    for r in &records {
        // record fabric use
        for x in r.pos.x..(r.pos.x + r.size.0) {
            for y in r.pos.y..(r.pos.y + r.size.1) {
                fabric[y as usize][x as usize] += 1;
            }
        }
    }

    let mut id = 0;
    for r in &records {
        let mut double_use = false;
        for x in r.pos.x..(r.pos.x + r.size.0) {
            for y in r.pos.y..(r.pos.y + r.size.1) {
                double_use |= fabric[y as usize][x as usize] > 1;
            }
        }
        if !double_use {
            id = r.id;
            break;
        }
    }

    println!("{}", id);
}
