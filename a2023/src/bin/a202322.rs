use core::fmt;
use core::iter::once;
use core::str::FromStr;

use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::p3d;
use lowdim::v3d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::BBox3d;
use lowdim::Point3d;

use util::runtime_error;

#[derive(Clone, Copy, Debug)]
struct Brick {
    p0: Point3d,
    p1: Point3d,
}
impl Brick {
    fn bbox(&self) -> BBox3d {
        BBox3d::from_corners(self.p0, self.p1)
    }
    fn bbox_xy(&self) -> BBox2d {
        BBox2d::from_corners(p2d(self.p0.x(), self.p0.y()), p2d(self.p1.x(), self.p1.y()))
    }
}
impl fmt::Display for Brick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.p0.x(),
            self.p0.y(),
            self.p0.z(),
            self.p1.x(),
            self.p1.y(),
            self.p1.z()
        )
    }
}
fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}
fn point3d(i: &str) -> IResult<&str, Point3d> {
    let (i, x) = uint(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = uint(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, z) = uint(i)?;
    Ok((i, p3d(x, y, z)))
}
fn brick(i: &str) -> IResult<&str, Brick> {
    let (i, p0) = point3d(i)?;
    let (i, _) = tag("~")(i)?;
    let (i, p1) = point3d(i)?;
    Ok((i, Brick { p0, p1 }))
}

#[derive(Clone, Debug)]
struct Input {
    bricks: Vec<Brick>,
}
impl Input {
    fn new(bricks: Vec<Brick>) -> Input {
        Input { bricks }
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for brick in &self.bricks {
            writeln!(f, "{}", brick)?;
        }
        Ok(())
    }
}
fn input(i: &str) -> IResult<&str, Input> {
    map(separated_list1(line_ending, brick), Input::new)(i)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut bricks = input.bricks.clone();
    let bbox = BBox3d::enclosing(bricks.iter().flat_map(|b| once(&b.p0).chain(once(&b.p1))))
        .ok_or(runtime_error!("no bricks found"))?;

    bricks.sort_by_key(|b| b.bbox().z_min());

    let bbxy = BBox2d::from_corners(
        p2d(bbox.x_min(), bbox.y_min()),
        p2d(bbox.x_max(), bbox.y_max()),
    );
    let mut heights = Array2d::new(bbxy, 0);
    let mut brick_index: Array2d<i64, Option<usize>> = Array2d::new(bbxy, None);

    // supports[i]: The bricks immediately above brick i.
    let mut supports: Vec<HashSet<_>> = Vec::new();
    // supported_by[i]: The bricks immediately below brick i.
    let mut supported_by = Vec::new();

    for (i, brick) in bricks.iter_mut().enumerate() {
        let z_min = brick.bbox().z_min();
        let dz = brick
            .bbox_xy()
            .iter()
            .map(|p| z_min - heights[p] - 1)
            .min()
            .ok_or(runtime_error!("internal error: empty brick"))?;

        if dz < 0 {
            return Err(runtime_error!("internal error: overlap"));
        }

        // Move the brick down.
        let d = v3d(0, 0, -dz);
        brick.p0 += d;
        brick.p1 += d;

        // Update heights and support.
        let mut support = HashSet::new();
        let z_min = brick.bbox().z_min();
        let z_max = brick.bbox().z_max();
        for p in brick.bbox_xy() {
            if heights[p] == z_min - 1 {
                if let Some(j) = brick_index[p] {
                    support.insert(j);
                    supports[j].insert(i);
                }
            }

            heights[p] = z_max;
            brick_index[p] = Some(i);
        }
        supports.push(HashSet::new());
        supported_by.push(support);
    }
    let lone_supports = supported_by
        .iter()
        .filter_map(|support| {
            if support.len() == 1 {
                support.iter().next()
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    let result1 = bricks.len() - lone_supports.len();

    let mut count = 0;
    for i in 0..bricks.len() {
        let mut removed = HashSet::new();
        let mut stack = Vec::new();

        // Remove brick i, schedule a check for bricks immediately above.
        removed.insert(i);
        for &k in &supports[i] {
            stack.push(k);
        }

        while let Some(j) = stack.pop() {
            if supported_by[j].iter().all(|k| removed.contains(k)) {
                // All bricks supporting brick j are gone, remove it.
                removed.insert(j);
                for &k in &supports[j] {
                    stack.push(k);
                }
            }
        }
        // Only count bricks other than the initial one.
        count += removed.len() - 1;
    }
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
