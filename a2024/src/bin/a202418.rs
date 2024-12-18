use core::fmt;
use core::hash::Hash;
use core::hash::Hasher;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::Array2d;
use lowdim::Point2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Safe,
    Corrupted,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Safe => '.',
            Square::Corrupted => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn line(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = i64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = i64(i)?;
    Ok((i, p2d(x, y)))
}

fn input(i: &str) -> IResult<&str, Vec<Point2d>> {
    separated_list1(line_ending, line)(i)
}

#[derive(Clone, Copy, Debug)]
struct Node<'a> {
    map: &'a Array2d<i64, Square>,
    p: Point2d,
}
impl<'a> Node<'a> {
    fn start(map: &'a Array2d<i64, Square>) -> Node<'a> {
        let p = map.bbox().min();
        Node { map, p }
    }
    fn successors(&self) -> Vec<(Node<'a>, i64)> {
        let mut result = Vec::new();
        for p in self.p.neighbors_l1() {
            if self.map.get(p) == Some(&Square::Safe) {
                let map = self.map;
                let node = Node { map, p };
                result.push((node, 1));
            }
        }
        result
    }
    fn heuristic(&self) -> i64 {
        self.p.distance_l1(self.map.bbox().max())
    }
    fn success(&self) -> bool {
        self.p == self.map.bbox().max()
    }
}
impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node<'a>) -> bool {
        self.p == other.p
    }
}
impl Eq for Node<'_> {}
impl Hash for Node<'_> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.p.hash(state);
    }
}
fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for point in &input {
    //     println!("{},{}", point.x(), point.y());
    // }

    let bbox = bb2d(0..=70, 0..=70);

    let mut map = Array2d::with(bbox, |_| Square::Safe);
    for &p in input.iter().take(1024) {
        map[p] = Square::Corrupted;
    }

    let start_node = Node::start(&map);
    let search_result = astar(
        &start_node,
        Node::successors,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) = search_result.ok_or("no path found")?;
    let result1 = cost;

    let mut map = Array2d::with(bbox, |_| Square::Safe);
    let mut last_byte_pos = None;
    for &p in &input {
        map[p] = Square::Corrupted;
        let start_node = Node::start(&map);
        let search_result = astar(
            &start_node,
            Node::successors,
            Node::heuristic,
            Node::success,
        );
        if search_result.is_none() {
            last_byte_pos = Some(p);
            break;
        }
    }
    let pos = last_byte_pos.ok_or::<Error>("exit not prevented".into())?;
    let result2 = format!("{},{}", pos.x(), pos.y());

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
