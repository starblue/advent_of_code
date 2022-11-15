use core::fmt;
use core::ops::RangeInclusive;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::bb3d;
use lowdim::Array3d;
use lowdim::BBox3d;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Step {
    on: bool,
    cuboid: BBox3d,
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} x={}..{},y={}..{},z={}..{}",
            if self.on { "on" } else { "off" },
            self.cuboid.x_min(),
            self.cuboid.x_max(),
            self.cuboid.y_min(),
            self.cuboid.y_max(),
            self.cuboid.z_min(),
            self.cuboid.z_max()
        )
    }
}

fn range(i: &str) -> IResult<&str, RangeInclusive<i64>> {
    let (i, min) = i64(i)?;
    let (i, _) = tag("..")(i)?;
    let (i, max) = i64(i)?;
    Ok((i, RangeInclusive::new(min, max)))
}

fn on(i: &str) -> IResult<&str, bool> {
    alt((value(true, tag("on")), value(false, tag("off"))))(i)
}

fn step(i: &str) -> IResult<&str, Step> {
    let (i, on) = on(i)?;
    let (i, _) = tag(" x=")(i)?;
    let (i, x_range) = range(i)?;
    let (i, _) = tag(",y=")(i)?;
    let (i, y_range) = range(i)?;
    let (i, _) = tag(",z=")(i)?;
    let (i, z_range) = range(i)?;
    Ok((
        i,
        Step {
            on,
            cuboid: bb3d(x_range, y_range, z_range),
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(line_ending, step)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for step in &input {
    //     println!("{}", step);
    // }

    let area = bb3d(-50..=50, -50..=50, -50..=50);
    let mut core = Array3d::new(area, false);
    for step in &input {
        if let Some(bbox) = step.cuboid.intersection(&area) {
            for p in bbox {
                core[p] = step.on;
            }
        }
    }
    let result_a = area.iter().filter(|&p| core[p]).count();

    let mut bboxes: Vec<(i64, BBox3d<i64>)> = Vec::new();
    for step in &input {
        let mut new_bboxes = Vec::new();
        if step.on {
            // count positively
            new_bboxes.push((1_i64, step.cuboid));
        }
        for &(s, b) in &bboxes {
            if let Some(bbox) = step.cuboid.intersection(&b) {
                new_bboxes.push((-s, bbox));
            }
        }
        bboxes.append(&mut new_bboxes);
    }
    let result_b = bboxes
        .into_iter()
        .map(|(s, b)| {
            let v = i64::try_from(b.volume()).unwrap();
            s * v
        })
        .sum::<i64>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
