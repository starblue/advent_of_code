use std::collections::HashMap;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug)]
struct StorageNode {
    pos: Point2d,
    size: usize,
    used: usize,
}
impl StorageNode {
    fn name(&self) -> String {
        format!("/dev/grid/node-x{}-y{}", self.pos.x(), self.pos.y())
    }
    fn size(&self) -> usize {
        self.size
    }
    fn used(&self) -> usize {
        self.used
    }
    fn avail(&self) -> usize {
        self.size - self.used
    }
    fn percent_used(&self) -> usize {
        100 * self.used / self.size
    }
    fn is_empty(&self) -> bool {
        self.used() == 0
    }
}
impl fmt::Display for StorageNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{:<22} {:4}T {:4}T  {:4}T  {:3}%",
            self.name(),
            self.size(),
            self.used(),
            self.avail(),
            self.percent_used(),
        )
    }
}

#[derive(Clone, Debug)]
enum Line {
    StorageNode(StorageNode),
    Other(String),
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Line::StorageNode(node) => write!(f, "{}", node),
            Line::Other(string) => write!(f, "{}", string),
        }
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(many1(none_of("\r\n"))), String::from)(i)
}

fn line_storage_node(i: &str) -> IResult<&str, Line> {
    let (i, _) = tag("/dev/grid/node-x")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag("-y")(i)?;
    let (i, y) = uint(i)?;
    let (i, _) = multispace1(i)?;
    let (i, size) = uint(i)?;
    let (i, _) = tag("T")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, used) = uint(i)?;
    let (i, _) = tag("T")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, _avail) = uint(i)?;
    let (i, _) = tag("T")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, _percent_used) = uint(i)?;
    let (i, _) = tag("%")(i)?;
    let (i, _) = line_ending(i)?;

    let pos = p2d(i64::try_from(x).unwrap(), i64::try_from(y).unwrap());
    Ok((i, Line::StorageNode(StorageNode { pos, size, used })))
}
fn line_other(i: &str) -> IResult<&str, Line> {
    let (i, s) = string(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Line::Other(s)))
}
fn line(i: &str) -> IResult<&str, Line> {
    alt((line_storage_node, line_other))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Line>> {
    many1(line)(i)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    /// The position of the data we want to access.
    data_pos: Point2d,
    /// The position of the empty server.
    hole_pos: Point2d,
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for line in &input {
    //     println!("{}", line);
    // }

    let storage_nodes = input
        .iter()
        .filter_map(|line| match line {
            Line::StorageNode(node) => Some(node),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for node_a in &storage_nodes {
        for node_b in &storage_nodes {
            if !node_a.is_empty() && node_a.pos != node_b.pos && node_a.used() <= node_b.avail() {
                count += 1;
            }
        }
    }
    let result_a = count;

    let map = storage_nodes
        .into_iter()
        .map(|sn| (sn.pos, sn))
        .collect::<HashMap<_, _>>();

    let (&hole_pos, _) = map
        .iter()
        .find(|(_pos, sn)| sn.is_empty())
        .expect("no empty server found");
    let &data_pos = map
        .keys()
        .filter(|pos| pos.y() == 0)
        .max_by_key(|pos| pos.x())
        .expect("no data server found");

    let start_node = Node { data_pos, hole_pos };
    let target_data_pos = p2d(0, 0);
    let successors = |n: &Node| {
        let dest_sn = map[&n.hole_pos];
        n.hole_pos
            .neighbors_l1()
            .filter_map(|new_hole_pos| {
                if let Some(src_sn) = map.get(&new_hole_pos) {
                    // Can we move the data from the source storage node
                    // to the destination storage node?
                    if src_sn.used() <= dest_sn.size() {
                        let data_pos = {
                            if n.data_pos == new_hole_pos {
                                // We move the target data.
                                // Hole and data swap positions.
                                n.hole_pos
                            } else {
                                n.data_pos
                            }
                        };
                        Some((
                            Node {
                                data_pos,
                                hole_pos: new_hole_pos,
                            },
                            1,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    let heuristic = |n: &Node| n.data_pos.distance_l1(target_data_pos);
    let success = |n: &Node| n.data_pos == target_data_pos;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.unwrap();
    let result_b = cost;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
