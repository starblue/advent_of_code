use std::fmt;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use util::DisjointSets;

#[derive(Clone, Debug)]
struct Program {
    id: usize,
    neighbors: Vec<usize>,
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)?;
        let mut sep = " <-> ";
        for n in &self.neighbors {
            write!(f, "{}{}", sep, n)?;
            sep = ", ";
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn program(i: &str) -> IResult<&str, Program> {
    let (i, id) = uint(i)?;
    let (i, _) = tag(" <-> ")(i)?;
    let (i, neighbors) = separated_list1(tag(", "), uint)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Program { id, neighbors }))
}

fn input(i: &str) -> IResult<&str, Vec<Program>> {
    many1(program)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for program in &input {
    //     println!("{}", program);
    // }

    // Use disjoint sets to represent groups.
    let mut groups = DisjointSets::new();
    for p in &input {
        groups.add(p.id);
    }
    for p in &input {
        for &n_id in &p.neighbors {
            groups.union(&p.id, &n_id);
        }
    }
    let result_a = groups.set_size(&0);

    let result_b = groups.set_reprs().len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
