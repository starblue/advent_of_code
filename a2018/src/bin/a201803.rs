use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::do_parse;
use nom::map_res;
use nom::named;

use gamedim::p2d;
use gamedim::v2d;
use gamedim::Array2d;
use gamedim::BBox;
use gamedim::BBox2d;
use gamedim::Point2d;
use gamedim::Vec2d;

#[derive(Clone, Debug)]
struct Record {
    id: i64,
    bbox: BBox2d,
}

#[derive(Clone, Debug)]
enum Error {}

named!(int<&str, i64>,
    map_res!(digit1, FromStr::from_str)
);

named!(
    id<&str, i64>,
    do_parse!(char!('#') >> id: int >> (id) )
);

named!(
    position<&str, Point2d>,
    do_parse!(x: int >> char!(',') >> y: int >> (p2d(x, y)))
);

named!(
    size<&str, Vec2d>,
    do_parse!(x: int >> char!('x') >> y: int >> (v2d(x, y)))
);

named!(
    record<&str, Record>,
    do_parse!(
        id: id >>
        multispace1 >>
        char!('@') >>
        multispace1 >>
        pos: position >>
        char!(':') >>
        multispace1 >>
        size: size >>
        line_ending >>
            ({
                let bbox = BBox::new(pos, size);
                Record { id, bbox }
            })
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

    let mut fabric = Array2d::from_vec(
        repeat(repeat(0).take(1000).collect::<Vec<u32>>())
            .take(1000)
            .collect::<Vec<_>>(),
    );
    for r in &records {
        // record fabric use
        for p in r.bbox.iter() {
            fabric[p] += 1;
        }
    }

    let area = fabric.bounds().iter().filter(|&p| fabric[p] > 1).count();
    let result_a = area;
    println!("a: {}", result_a);

    let id = records
        .iter()
        .find(|r| !r.bbox.iter().any(|p| fabric[p] > 1))
        .map(|r| r.id);
    let result_b = id.unwrap();
    println!("b: {}", result_b);
}
