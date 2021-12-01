use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::combinator::map_res;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Debug)]
struct Record {
    id: i64,
    bbox: BBox2d,
}

#[derive(Clone, Debug)]
enum Error {}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn id(i: &str) -> IResult<&str, i64> {
    let (i, _) = char('#')(i)?;
    let (i, id) = int(i)?;
    Ok((i, id))
}

fn position(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = int(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y) = int(i)?;
    Ok((i, p2d(x, y)))
}

fn size(i: &str) -> IResult<&str, Vec2d> {
    let (i, x) = int(i)?;
    let (i, _) = char('x')(i)?;
    let (i, y) = int(i)?;
    Ok((i, v2d(x, y)))
}

fn record(i: &str) -> IResult<&str, Record> {
    let (i, id) = id(i)?;
    let (i, _) = multispace1(i)?;
    let (i, _) = char('@')(i)?;
    let (i, _) = multispace1(i)?;
    let (i, pos) = position(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = multispace1(i)?;
    let (i, size) = size(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, {
        let bbox = BBox::new(pos, size);
        Record { id, bbox }
    }))
}

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

    let area = fabric.bbox().iter().filter(|&p| fabric[p] > 1).count();
    let result_a = area;
    println!("a: {}", result_a);

    let id = records
        .iter()
        .find(|r| !r.bbox.iter().any(|p| fabric[p] > 1))
        .map(|r| r.id);
    let result_b = id.unwrap();
    println!("b: {}", result_b);
}
