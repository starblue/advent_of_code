use core::str::FromStr;

use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::IResult;

use lowdim::p3d;
use lowdim::v3d;
use lowdim::BBox3d;
use lowdim::Point3d;

use util::runtime_error;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn point3d(i: &str) -> IResult<&str, Point3d> {
    let (i, x) = int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, z) = int(i)?;
    Ok((i, p3d(x, y, z)))
}

fn input(i: &str) -> IResult<&str, Vec<Point3d>> {
    separated_list0(line_ending, point3d)(i)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for cube in &input {
    //     println!("{},{},{}", cube.x(), cube.y(), cube.z());
    // }

    let cubes = input.into_iter().collect::<HashSet<_>>();

    let mut count = 0;
    for cube in &cubes {
        for nc in cube.neighbors_l1() {
            if !cubes.contains(&nc) {
                count += 1;
            }
        }
    }
    let result1 = count;

    let input_bbox = BBox3d::enclosing(cubes.iter()).ok_or(runtime_error!("no input"))?;
    let min = input_bbox.min() - v3d(1, 1, 1);
    let max = input_bbox.max() + v3d(1, 1, 1);
    let bbox = BBox3d::from_corners(min, max);

    // Flood fill the outside
    let mut outside = HashSet::new();
    let mut stack = vec![bbox.min()];
    while let Some(p) = stack.pop() {
        if bbox.contains(&p) && !cubes.contains(&p) && !outside.contains(&p) {
            stack.extend(p.neighbors_l1());
            outside.insert(p);
        }
    }
    let mut count = 0;
    for cube in &cubes {
        for nc in cube.neighbors_l1() {
            if outside.contains(&nc) {
                count += 1;
            }
        }
    }
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
