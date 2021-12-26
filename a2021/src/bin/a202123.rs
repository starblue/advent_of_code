use core::hash::Hash;

use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}
impl AmphipodType {
    fn step_energy(&self) -> i64 {
        match self {
            AmphipodType::Amber => 1,
            AmphipodType::Bronze => 10,
            AmphipodType::Copper => 100,
            AmphipodType::Desert => 1000,
        }
    }
    fn to_char(self) -> char {
        match self {
            AmphipodType::Amber => 'A',
            AmphipodType::Bronze => 'B',
            AmphipodType::Copper => 'C',
            AmphipodType::Desert => 'D',
        }
    }
}
impl fmt::Display for AmphipodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Amphipod(AmphipodType),
    Wall,
    Open,
    None,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Amphipod(a) => a.to_char(),
            Square::Wall => '#',
            Square::Open => '.',
            Square::None => ' ',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Burrow {
    map: Array2d<i64, Square>,
    // The maximal y value an amphipod can move to at the lower end of a room.
    y_max: i64,
}
impl Burrow {
    fn new(map: Array2d<i64, Square>) -> Burrow {
        let mut y_max = map.bbox().y_min();
        for p in map.bbox() {
            if matches!(map[p], Square::Open | Square::Amphipod(_)) {
                y_max = y_max.max(p.y())
            }
        }
        Burrow { map, y_max }
    }
    fn get(&self, p: Point2d) -> Option<&Square> {
        self.map.get(p)
    }
    fn amphipod(&self, p: Point2d) -> Option<AmphipodType> {
        match self.get(p) {
            Some(Square::Amphipod(a)) => Some(*a),
            _ => None,
        }
    }

    fn is_open(&self, p: Point2d) -> bool {
        self.get(p) == Some(&Square::Open)
    }
    fn is_amphipod(&self, t: AmphipodType, p: Point2d) -> bool {
        self.get(p) == Some(&Square::Amphipod(t))
    }

    fn dest_x(&self, t: AmphipodType) -> i64 {
        match t {
            AmphipodType::Amber => 3,
            AmphipodType::Bronze => 5,
            AmphipodType::Copper => 7,
            AmphipodType::Desert => 9,
        }
    }

    /// Returns the distance to the upper square of the destination.
    ///
    /// The minimal number of moves to the destination for both
    /// amphipods of a type is the sum of their distances plus 1,
    /// because one of them has to additionally move to the lower square.
    /// An amphipod already in the lower square gets a distance of -1
    /// in order to cancel that additional cost.
    fn dest_distance(&self, t: AmphipodType, p: Point2d) -> i64 {
        let dest_x = self.dest_x(t);
        if p.x() == dest_x {
            2 - p.y()
        } else {
            p.distance_l1(p2d(dest_x, 1)) + 1
        }
    }
    fn amphipods(&self) -> Vec<(AmphipodType, Point2d)> {
        self.map
            .bbox()
            .iter()
            .filter_map(|p| self.amphipod(p).map(|t| (t, p)))
            .collect()
    }
    fn hallway() -> Vec<Point2d> {
        vec![
            p2d(1, 1),
            p2d(2, 1),
            p2d(4, 1),
            p2d(6, 1),
            p2d(8, 1),
            p2d(10, 1),
            p2d(11, 1),
        ]
    }
    fn at_hallway(&self, p: Point2d) -> bool {
        Burrow::hallway().contains(&p)
    }
    fn at_destination(&self, t: AmphipodType, p: Point2d) -> bool {
        let x = p.x();
        let y = p.y();
        x == self.dest_x(t)
            && (2..=self.y_max).contains(&y)
            && ((y + 1)..=self.y_max).all(|y1| self.is_amphipod(t, p2d(x, y1)))
    }
    fn destination(&self, t: AmphipodType) -> Option<Point2d> {
        let x = self.dest_x(t);
        let mut p = p2d(x, self.y_max);
        while self.is_amphipod(t, p) {
            p += v2d(0, -1);
        }
        if self.is_open(p) {
            Some(p)
        } else {
            None
        }
    }
    fn move_cost(&self, t: AmphipodType, p: Point2d, new_p: Point2d) -> Option<i64> {
        if self.at_hallway(p) {
            let mut p = p;
            let mut cost = 0;

            // Move through hallway to destination room.
            let dest_x = new_p.x();
            let d = if p.x() < dest_x {
                v2d(1, 0)
            } else {
                v2d(-1, 0)
            };
            while p.x() != dest_x {
                p += d;
                cost += t.step_energy();
                if !self.is_open(p) {
                    // Can't move there.
                    return None;
                }
            }

            // Move into destination room.
            let d = v2d(0, 1);
            while p != new_p {
                p += d;
                cost += t.step_energy();
                if !self.is_open(p) {
                    // Can't move there.
                    return None;
                }
            }
            if !self.is_open(p) {
                // Can't move there.
                return None;
            }
            Some(cost)
        } else {
            let mut p = p;
            let mut cost = 0;

            // Move into hallway.
            let d = v2d(0, -1);
            while p.y() != new_p.y() {
                p += d;
                cost += t.step_energy();
                if !self.is_open(p) {
                    // Can't move there.
                    return None;
                }
            }

            // Move through hallway to destination room.
            let d = if p.x() < new_p.x() {
                v2d(1, 0)
            } else {
                v2d(-1, 0)
            };
            while p != new_p {
                p += d;
                cost += t.step_energy();
                if !self.is_open(p) {
                    // Can't move there.
                    return None;
                }
            }
            Some(cost)
        }
    }
    fn possible_moves(&self, p: Point2d) -> Vec<(Point2d, i64)> {
        let mut result = Vec::new();
        let t = self.amphipod(p).expect("no amphipod at start position");
        if self.at_destination(t, p) {
            // No more moves are possible.
        } else if self.at_hallway(p) {
            // Find possible moves into the destination room.
            if let Some(new_p) = self.destination(t) {
                if let Some(cost) = self.move_cost(t, p, new_p) {
                    result.push((new_p, cost))
                }
            }
        } else {
            // In starting position and not in the destination.
            // Find possible move into the hallway.
            for new_p in Burrow::hallway() {
                if let Some(cost) = self.move_cost(t, p, new_p) {
                    result.push((new_p, cost))
                }
            }
        }
        result
    }
    fn do_move(&self, p: Point2d, new_p: Point2d) -> Burrow {
        let mut new_burrow = self.clone();
        let t = self.amphipod(p).expect("no amphipod at start position");
        new_burrow.map[p] = Square::Open;
        new_burrow.map[new_p] = Square::Amphipod(t);
        new_burrow
    }
}
impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.map.bbox().y_range() {
            for x in self.map.bbox().x_range() {
                write!(f, "{}", self.map[p2d(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    burrow: Burrow,
}
impl Node {
    fn new(burrow: Burrow) -> Node {
        Node { burrow }
    }
    fn successors(&self) -> impl Iterator<Item = (Node, i64)> {
        let successors = self
            .burrow
            .amphipods()
            .into_iter()
            .flat_map(|(_t, p)| {
                self.burrow
                    .possible_moves(p)
                    .into_iter()
                    .map(move |(np, c)| (Node::new(self.burrow.do_move(p, np)), c))
            })
            .collect::<Vec<_>>();
        successors.into_iter()
    }
    fn heuristic(&self) -> i64 {
        1111_i64
            + self
                .burrow
                .amphipods()
                .into_iter()
                .map(|(t, p)| t.step_energy() * self.burrow.dest_distance(t, p))
                .sum::<i64>()
    }
    fn success(&self) -> bool {
        self.burrow
            .amphipods()
            .into_iter()
            .all(|(t, p)| self.burrow.at_destination(t, p))
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Amphipod(AmphipodType::Amber), tag("A")),
        value(Square::Amphipod(AmphipodType::Bronze), tag("B")),
        value(Square::Amphipod(AmphipodType::Copper), tag("C")),
        value(Square::Amphipod(AmphipodType::Desert), tag("D")),
        value(Square::Wall, tag("#")),
        value(Square::Open, tag(".")),
        value(Square::None, tag(" ")),
    ))(i)
}

fn row(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vec<Square>>> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    Ok((i, rows))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let burrow = Burrow::new(Array2d::from_vec(input.clone()));
    let start_node = Node::new(burrow);
    let search_result = astar(
        &start_node,
        Node::successors,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) = search_result.unwrap();
    let result_a = cost;

    let mut input = input;
    input.insert(3, row("  #D#C#B#A#  ").unwrap().1);
    input.insert(4, row("  #D#B#A#C#  ").unwrap().1);
    let burrow = Burrow::new(Array2d::from_vec(input));
    let start_node = Node::new(burrow);
    let search_result = astar(
        &start_node,
        Node::successors,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) = search_result.unwrap();
    let result_b = cost;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
