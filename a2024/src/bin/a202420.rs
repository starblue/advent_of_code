use core::fmt;
use core::hash::Hash;
use core::hash::Hasher;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::dijkstra_all;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

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
impl Square {
    fn is_track(&self) -> bool {
        *self != Square::Wall
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
}
impl<'a> Node<'a> {
    fn start(input: &'a Input) -> Node<'a> {
        let p = input.start_pos().unwrap();
        Node { input, p }
    }
    fn goal(input: &'a Input) -> Node<'a> {
        let p = input.goal_pos().unwrap();
        Node { input, p }
    }
    fn successors(&self) -> Vec<(Node<'a>, i64)> {
        let input = self.input;
        self.p
            .neighbors_l1()
            .filter_map(|p| {
                if input.map[p].is_track() {
                    Some((Node { input, p }, 1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
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
    // println!("{}", input);

    // Cost from the start to all track positions.
    let start_node_costs = dijkstra_all(&Node::start(&input), Node::successors);
    let mut start_costs = start_node_costs
        .iter()
        .map(|(k, &(_, v))| (k.p, v))
        .collect::<HashMap<Point2d, i64>>();
    start_costs.insert(input.start_pos()?, 0);

    // Cost from all track positions positions to the goal.
    let goal_node_costs = dijkstra_all(&Node::goal(&input), Node::successors);
    let mut goal_costs = goal_node_costs
        .iter()
        .map(|(k, &(_, v))| (k.p, v))
        .collect::<HashMap<Point2d, i64>>();
    goal_costs.insert(input.goal_pos()?, 0);

    let bbox = input.bbox();

    // Minimal cost without cheating.
    let cost = start_costs[&input.goal_pos()?];

    let mut count = 0;
    for p in bbox {
        if !input.map[p].is_track() {
            for d in [v2d(1, 0), v2d(0, -1), v2d(-1, 0), v2d(0, 1)] {
                let p0 = p - d;
                let p1 = p + d;
                if bbox.contains(&p0)
                    && input.map[p0].is_track()
                    && bbox.contains(&p1)
                    && input.map[p1].is_track()
                {
                    // Try cheating by tunneling from `p0` to `p1` through
                    // the wall at `p`.

                    let start_cost0 = start_costs[&p0];
                    let goal_cost0 = goal_costs[&p0];
                    let goal_cost1 = goal_costs[&p1];

                    // Cost without cheating.
                    let cost0 = start_cost0 + goal_cost0;

                    if cost0 == cost {
                        // We are on the main track from start to end.

                        // Cost with cheating.
                        let cost1 = start_cost0 + 2 + goal_cost1;

                        let saving = cost0 - cost1;
                        if saving >= 100 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    let result1 = count;

    let t = 20;
    let mut count = 0;
    for p0 in bbox {
        if input.map[p0].is_track() {
            let bbox1 = BBox2d::from_corners(p0 - v2d(t, t), p0 + v2d(t, t))
                .intersection(&bbox)
                .ok_or("internal error: bounding boxes don't intersect")?;
            for p1 in bbox1 {
                let d = p0.distance_l1(p1);
                if d <= t && input.map[p1].is_track() {
                    // Try cheating by tunneling from `p0` to `p1`.

                    let start_cost0 = start_costs[&p0];
                    let goal_cost0 = goal_costs[&p0];
                    let goal_cost1 = goal_costs[&p1];

                    // Cost without cheating.
                    let cost0 = start_cost0 + goal_cost0;

                    if cost0 == cost {
                        // We are on the main track from start to end.

                        // Cost with cheating.
                        let cost1 = start_cost0 + d + goal_cost1;

                        let saving = cost0 - cost1;
                        if saving >= 100 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
