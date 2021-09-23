use std::collections::HashSet;
use std::i64;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::BBox;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Debug)]
struct Record {
    pos: Point2d,
    v: Vec2d,
}

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn vec2d(i: &str) -> IResult<&str, Vec2d> {
    let (i, _) = char('<')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, x) = int64(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, y) = int64(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char('>')(i)?;
    Ok((i, Vec2d::new(x, y)))
}

fn record(i: &str) -> IResult<&str, Record> {
    let (i, _) = tag("position=")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, pos) = vec2d(i)?;
    let (i, _) = multispace1(i)?;
    let (i, _) = tag("velocity=")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, v) = vec2d(i)?;
    Ok((
        i,
        Record {
            pos: Point2d::from(pos),
            v,
        },
    ))
}

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
