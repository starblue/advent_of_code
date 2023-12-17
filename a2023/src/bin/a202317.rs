use core::fmt;
use core::hash::Hash;
use core::hash::Hasher;
use core::str::FromStr;

use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;
use lowdim::Vector;

use util::runtime_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Square {
    heat_loss: i64,
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        assert!((1..=9).contains(&self.heat_loss));
        write!(f, "{}", self.heat_loss)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn start_pos(&self) -> Point2d {
        self.bbox().min()
    }
    fn goal_pos(&self) -> Point2d {
        self.bbox().max()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range() {
            for x in self.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    let (i, heat_loss) = map_res(recognize(one_of("123456789")), FromStr::from_str)(i)?;
    Ok((i, Square { heat_loss }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

#[derive(Clone, Copy, Debug)]
struct Node<'a> {
    input: &'a Input,
    p: Point2d,
    v: Vec2d,
}
impl<'a> Node<'a> {
    fn start(input: &'a Input) -> Node<'a> {
        let p = input.start_pos();
        let v = v2d(0, 0);
        Node { input, p, v }
    }
    fn successors1(&self) -> Vec<(Node<'a>, i64)> {
        self.successors(1, 3)
    }
    fn successors2(&self) -> Vec<(Node<'a>, i64)> {
        self.successors(4, 10)
    }
    fn successors(&self, min_dist: i64, max_dist: i64) -> Vec<(Node<'a>, i64)> {
        let input = &self.input;
        let mut result = Vec::new();
        for v in Vec2d::unit_vecs_l1() {
            if v != self.v && v != -self.v {
                let mut p = self.p;
                let mut n = 0;
                let mut heat_loss = 0;
                loop {
                    p += v;
                    n += 1;
                    if n > max_dist {
                        break;
                    }
                    if let Some(&square) = input.map.get(p) {
                        heat_loss += square.heat_loss;
                    } else {
                        break;
                    }
                    if n >= min_dist {
                        result.push((Node { input, p, v }, heat_loss));
                    }
                }
            }
        }
        result
    }
    fn heuristic(&self) -> i64 {
        self.p.distance_l1(self.input.goal_pos())
    }
    fn success(&self) -> bool {
        self.p == self.input.goal_pos()
    }
}
impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node<'a>) -> bool {
        self.p == other.p && self.v == other.v
    }
}
impl<'a> Eq for Node<'a> {}
impl<'a> Hash for Node<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.p.hash(state);
        self.v.hash(state);
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let start_node = Node::start(&input);
    let search_result = astar(
        &start_node,
        Node::successors1,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) = search_result.ok_or_else(|| runtime_error!("no path found"))?;
    let result1 = cost;

    let start_node = Node::start(&input);
    let search_result = astar(
        &start_node,
        Node::successors2,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) = search_result.ok_or_else(|| runtime_error!("no path found"))?;
    let result2 = cost;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
