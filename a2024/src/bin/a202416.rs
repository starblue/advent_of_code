use core::fmt;
use core::hash::Hash;
use core::hash::Hasher;

use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;
use pathfinding::prelude::dijkstra_all;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Wall,
    Start,
    Goal,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Wall => '#',
            Square::Start => 'S',
            Square::Goal => 'E',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char('.')),
        value(Square::Wall, char('#')),
        value(Square::Start, char('S')),
        value(Square::Goal, char('E')),
    ))(i)
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn start_pos(&self) -> Result<Point2d> {
        Ok(self
            .map
            .bbox()
            .iter()
            .find(|&p| self.map[p] == Square::Start)
            .ok_or("start not found")?)
    }
    fn goal_pos(&self) -> Result<Point2d> {
        Ok(self
            .map
            .bbox()
            .iter()
            .find(|&p| self.map[p] == Square::Goal)
            .ok_or("start not found")?)
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
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
        let p = input.start_pos().unwrap();
        let v = v2d(1, 0);
        Node { input, p, v }
    }
    fn goal_east(input: &'a Input) -> Node<'a> {
        let p = input.goal_pos().unwrap();
        let v = v2d(1, 0);
        Node { input, p, v }
    }
    fn goal_north(input: &'a Input) -> Node<'a> {
        let p = input.goal_pos().unwrap();
        let v = v2d(0, -1);
        Node { input, p, v }
    }
    fn try_move_forward(&self) -> Option<Node<'a>> {
        let input = &self.input;
        let p = self.p + self.v;
        let v = self.v;
        if input.map[p] != Square::Wall {
            Some(Node { input, p, v })
        } else {
            None
        }
    }
    fn try_move_backward(&self) -> Option<Node<'a>> {
        let input = &self.input;
        let p = self.p - self.v;
        let v = self.v;
        if input.map[p] != Square::Wall {
            Some(Node { input, p, v })
        } else {
            None
        }
    }
    fn turn_left(&self) -> Node<'a> {
        let input = &self.input;
        let p = self.p;
        let v = self.v.rotate_left();
        Node { input, p, v }
    }
    fn turn_right(&self) -> Node<'a> {
        let input = &self.input;
        let p = self.p;
        let v = self.v.rotate_right();
        Node { input, p, v }
    }
    fn successors(&self) -> Vec<(Node<'a>, i64)> {
        let mut result = Vec::new();
        if let Some(node_cost) = self.try_move_forward() {
            result.push((node_cost, 1));
        }
        result.push((self.turn_left(), 1000));
        result.push((self.turn_right(), 1000));
        result
    }
    fn predecessors(&self) -> Vec<(Node<'a>, i64)> {
        let mut result = Vec::new();
        if let Some(node_cost) = self.try_move_backward() {
            result.push((node_cost, 1));
        }
        result.push((self.turn_left(), 1000));
        result.push((self.turn_right(), 1000));
        result
    }
    fn heuristic(&self) -> i64 {
        self.p.distance_l1(self.input.goal_pos().unwrap())
    }
    fn success(&self) -> bool {
        self.p == self.input.goal_pos().unwrap()
    }
}
impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node<'a>) -> bool {
        self.p == other.p && self.v == other.v
    }
}
impl Eq for Node<'_> {}
impl Hash for Node<'_> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.p.hash(state);
        self.v.hash(state);
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let start_node = Node::start(&input);
    let search_result = astar(
        &start_node,
        Node::successors,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) = search_result.ok_or("no path found")?;
    let result1 = cost;

    // Cost from the start to nodes.
    let start_costs = dijkstra_all(&start_node, Node::successors);
    // Cost from nodes to the goal, arriving there facing east.
    let goal_costs_e = dijkstra_all(&Node::goal_east(&input), Node::predecessors);
    // Cost from nodes to the goal, arriving there facing north.
    let goal_costs_n = dijkstra_all(&Node::goal_north(&input), Node::predecessors);

    let mut best_path_tiles = HashSet::new();
    best_path_tiles.insert(input.start_pos()?);
    best_path_tiles.insert(input.goal_pos()?);

    for (n, (_, start_cost)) in &start_costs {
        let mut goal_cost = i64::MAX;
        if let Some((_, c1)) = goal_costs_e.get(n) {
            goal_cost = goal_cost.min(*c1);
        }
        if let Some((_, c1)) = goal_costs_n.get(n) {
            goal_cost = goal_cost.min(*c1);
        }
        if start_cost + goal_cost == cost {
            best_path_tiles.insert(n.p);
        }
    }
    let result2 = best_path_tiles.len();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
