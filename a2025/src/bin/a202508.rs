use core::str::FromStr;

use std::fmt;
use std::fmt::Display;
use std::io;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p3d;
use lowdim::Point3d;
use util::DisjointSets;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct JunctionBox(Point3d);
impl JunctionBox {
    fn distance_squared(&self, other: JunctionBox) -> i64 {
        self.0.distance_l2_squared(other.0)
    }
    fn x(&self) -> i64 {
        self.0.x()
    }
}
impl Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.0.x(), self.0.y(), self.0.z())
    }
}

fn junction_box(i: &str) -> IResult<&str, JunctionBox> {
    let (i, x) = uint(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y) = uint(i)?;
    let (i, _) = char(',')(i)?;
    let (i, z) = uint(i)?;
    Ok((i, JunctionBox(p3d(x, y, z))))
}

fn input(i: &str) -> IResult<&str, Vec<JunctionBox>> {
    separated_list1(line_ending, junction_box)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for junction_box in &input {
    //     println!("{}", junction_box);
    // }

    let mut pairs = input
        .iter()
        .enumerate()
        .flat_map(|(i, &jb0)| input[(i + 1)..].iter().map(move |&jb1| (jb0, jb1)))
        .collect::<Vec<_>>();
    pairs.sort_by_key(|&(jb0, jb1)| jb0.distance_squared(jb1));

    let mut circuits = DisjointSets::new();
    for &jb in &input {
        circuits.add(jb);
    }

    for (jb0, jb1) in &pairs[..1000] {
        circuits.union(jb0, jb1);
    }
    let mut circuit_sizes = circuits
        .set_reprs()
        .iter()
        .map(|r| circuits.set_size(r))
        .collect::<Vec<_>>();
    // Sort sizes in reverse order.
    circuit_sizes.sort_by(|s0, s1| s1.cmp(s0));
    let result1 = (0..3).map(|i| circuit_sizes[i]).product::<usize>();

    let mut result2 = 0;
    for (jb0, jb1) in &pairs {
        circuits.union(jb0, jb1);
        if circuits.set_count() == 1 {
            result2 = jb0.x() * jb1.x();
            break;
        }
    }

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
