use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
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
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for program in &input {
    //     println!("{}", program);
    // }

    // Use disjoint sets to represent groups.
    let mut groups = DisjointSets::new();
    // Map from program ids to internal ids in disjoint sets structure.
    let ids = input
        .iter()
        .map(|p| {
            let id = groups.add();
            (p.id, id)
        })
        .collect::<HashMap<_, _>>();
    for p in &input {
        for &n_id in &p.neighbors {
            groups.union(ids[&p.id], ids[&n_id]);
        }
    }
    let result_a = groups.set_size(ids[&0]);

    let result_b = groups.set_reprs().len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
