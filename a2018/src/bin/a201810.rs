use std::collections::HashSet;
use std::i64;
use std::io;
use std::str::FromStr;

use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::opt;
use nom::recognize;
use nom::tag;
use nom::tuple;

use gamedim::BBox;
use gamedim::Point2d;
use gamedim::Vec2d;

#[derive(Clone, Debug)]
struct Record {
    pos: Point2d,
    v: Vec2d,
}

#[derive(Clone, Debug)]
enum Error {}

named!(int64<&str, i64>,
    map_res!(recognize!(tuple!(opt!(char!('-')), digit1)), FromStr::from_str)
);

named!(
    vec2d<&str, Vec2d>,
    do_parse!(
        char!('<') >>
        multispace0 >>
        x: int64 >>
        multispace0 >>
        char!(',') >>
        multispace0 >>
        y: int64 >>
        multispace0 >>
        char!('>') >>
            (Vec2d::new(x, y)))
);

named!(
    record<&str, Record>,
    do_parse!(
        tag!("position=") >>
        multispace0 >>
        pos: vec2d >>
        multispace1 >>
        tag!("velocity=") >>
        multispace0 >>
        v: vec2d >>
            (Record { pos: Point2d::from(pos), v }))
);

fn pos_at_t(r: &Record, t: i64) -> Point2d {
    r.pos + t * r.v
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
        let result = record(line.trim_end());
        //println!("{:?}", result);

        let r = result.unwrap().1;
        records.push(r);
    }

    let mut t = 0;
    let mut last_area = usize::MAX;
    loop {
        let mut bbox = BBox::from_point(pos_at_t(&records[0], t));
        for r in records.iter().skip(1) {
            let p = pos_at_t(r, t);
            bbox = bbox.extend_to(p);
        }
        let area = bbox.area();
        if area > last_area {
            let ps = records
                .iter()
                .map(|r| pos_at_t(r, t - 1))
                .collect::<HashSet<_>>();
            for y in bbox.y_range() {
                for x in bbox.x_range() {
                    print!(
                        "{}",
                        if ps.contains(&Point2d::new(x, y)) {
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
        last_area = bbox.area();

        t += 1;
    }

    println!("{}", t - 1);
}
