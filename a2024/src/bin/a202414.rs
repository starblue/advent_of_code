use core::fmt;

use std::collections::HashMap;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
struct Robot {
    p: Point2d,
    v: Vec2d,
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "p={},{} v={},{}",
            self.p.x(),
            self.p.y(),
            self.v.x(),
            self.v.y()
        )
    }
}

fn robot(i: &str) -> IResult<&str, Robot> {
    let (i, _) = tag("p=")(i)?;
    let (i, x) = i64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = i64(i)?;
    let p = p2d(x, y);

    let (i, _) = tag(" v=")(i)?;
    let (i, x) = i64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = i64(i)?;
    let v = v2d(x, y);

    Ok((i, Robot { p, v }))
}

fn input(i: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, robot)(i)
}

fn quadrant(v: Vec2d) -> Option<usize> {
    if v.x() > 0 && v.y() > 0 {
        Some(0)
    } else if v.x() < 0 && v.y() > 0 {
        Some(1)
    } else if v.x() < 0 && v.y() < 0 {
        Some(2)
    } else if v.x() > 0 && v.y() < 0 {
        Some(3)
    } else {
        None
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for robot in &input {
    //     println!("{}", robot);
    // }

    let bbox = bb2d(0..101, 0..103);
    let mid_p = bbox.center();
    let mut counts = [0_i64; 4];
    for robot in &input {
        let new_p = (robot.p + 100 * robot.v) % bbox;
        if let Some(quadrant) = quadrant(new_p - mid_p) {
            counts[quadrant] += 1;
        }
    }
    let result1 = counts.iter().product::<i64>();

    let mut robots = input;
    let mut t = 0;
    loop {
        // Check if the robots form a picture.

        let mut position_counts = HashMap::new();
        for robot in &robots {
            let entry = position_counts.entry(robot.p).or_insert(0);
            *entry += 1;
        }
        // In a picture no two robots should be in the same square.
        if position_counts.values().all(|&n| n <= 1) {
            break;
        }

        // Move the robots for one second.
        for robot in &mut robots {
            robot.p = (robot.p + robot.v) % bbox;
        }
        t += 1;
    }
    let result2 = t;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
