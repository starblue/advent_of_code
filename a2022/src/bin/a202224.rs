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

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

use util::runtime_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_vec(self) -> Vec2d {
        match self {
            Direction::Up => v2d(0, -1),
            Direction::Down => v2d(0, 1),
            Direction::Left => v2d(-1, 0),
            Direction::Right => v2d(1, 0),
        }
    }
    fn to_char(self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}
impl TryFrom<Vec2d> for Direction {
    type Error = util::Error;
    fn try_from(v: Vec2d) -> Result<Self, util::Error> {
        if v == v2d(0, -1) {
            Ok(Direction::Up)
        } else if v == v2d(0, 1) {
            Ok(Direction::Down)
        } else if v == v2d(-1, 0) {
            Ok(Direction::Left)
        } else if v == v2d(1, 0) {
            Ok(Direction::Right)
        } else {
            Err(runtime_error!("not a direction"))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Clear,
    Wall,
    Blizzard(Direction),
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Clear => '.',
            Square::Wall => '#',
            Square::Blizzard(d) => d.to_char(),
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn start_pos(&self) -> util::Result<Point2d> {
        let bbox = self.map.bbox();
        let pos = bbox
            .x_range()
            .map(|x| {
                let y = bbox.y_min();
                p2d(x, y)
            })
            .find(|&p| self.map[p] == Square::Clear)
            .ok_or(runtime_error!("start position not found"))?;
        Ok(pos)
    }
    fn goal_pos(&self) -> util::Result<Point2d> {
        let bbox = self.map.bbox();
        let pos = bbox
            .x_range()
            .map(|x| {
                let y = bbox.y_max();
                p2d(x, y)
            })
            .find(|&p| self.map[p] == Square::Clear)
            .ok_or(runtime_error!("goal position not found"))?;
        Ok(pos)
    }
    fn blizzards(&self) -> Vec<Blizzard> {
        self.map
            .bbox()
            .iter()
            .flat_map(|pos| {
                if let Square::Blizzard(d) = self.map[pos] {
                    let dir = d.to_vec();
                    Some(Blizzard { pos, dir })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range() {
            for x in self.bbox().x_range() {
                write!(f, "{}", self.map[p2d(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Clear, char('.')),
        value(Square::Wall, char('#')),
        value(Square::Blizzard(Direction::Up), char('^')),
        value(Square::Blizzard(Direction::Down), char('v')),
        value(Square::Blizzard(Direction::Left), char('<')),
        value(Square::Blizzard(Direction::Right), char('>')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((
        i,
        Input {
            map: Array2d::from_vec(rows),
        },
    ))
}

#[derive(Clone, Debug)]
struct Blizzard {
    pos: Point2d,
    dir: Vec2d,
}

#[derive(Clone, Debug)]
struct Valley {
    bbox: BBox2d,
    inner_bbox: BBox2d,
    blizzards: HashMap<Point2d, Vec<Blizzard>>,
    start_pos: Point2d,
    goal_pos: Point2d,
}
impl Valley {
    fn new(input: &Input) -> util::Result<Valley> {
        let bbox = input.map.bbox();
        let p_min = bbox.min() + v2d(1, 1);
        let p_max = bbox.max() - v2d(1, 1);
        let inner_bbox = BBox2d::from_corners(p_min, p_max);
        let mut blizzards = HashMap::new();
        for blizzard in input.blizzards() {
            let p0 = blizzard.pos;
            let mut p = p0;
            loop {
                let entry = blizzards.entry(p).or_insert(Vec::new());
                entry.push(blizzard.clone());

                p = (p + blizzard.dir) % inner_bbox;
                if p == p0 {
                    break;
                }
            }
        }
        let start_pos = input.start_pos()?;
        let goal_pos = input.goal_pos()?;
        Ok(Valley {
            bbox,
            inner_bbox,
            blizzards,
            start_pos,
            goal_pos,
        })
    }
    fn is_allowed_pos(&self, pos: Point2d) -> bool {
        self.inner_bbox.contains(&pos)
            || pos == self.start_pos
            || pos == self.goal_pos
    }
    fn blizzard_pos_at(&self, blizzard: &Blizzard, minute: i64) -> Point2d {
        (blizzard.pos + minute * blizzard.dir) % self.inner_bbox
    }
    fn blizzards_at(&self, pos: Point2d, minute: i64) -> Vec<Blizzard> {
        if let Some(blizzards) = self.blizzards.get(&pos) {
            blizzards
                .iter()
                .filter(|b| self.blizzard_pos_at(b, minute) == pos)
                .cloned()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }
    fn is_blizzard_at(&self, pos: Point2d, minute: i64) -> bool {
        if let Some(blizzards) = self.blizzards.get(&pos) {
            blizzards
                .iter()
                .any(|b| self.blizzard_pos_at(b, minute) == pos)
        } else {
            false
        }
    }
    fn start_pos(&self) -> Point2d {
        self.start_pos
    }
    fn goal_pos(&self) -> Point2d {
        self.goal_pos
    }
}

#[derive(Clone, Debug)]
struct Node<'a> {
    valley: &'a Valley,
    pos: Point2d,
    minute: i64,
    additional_legs: i64,
}
impl<'a> Node<'a> {
    fn start(valley: &'a Valley, legs: i64) -> Node<'a> {
        let pos = valley.start_pos();
        let minute = 0;
        let additional_legs = legs - 1;
        Node {
            valley,
            pos,
            minute,
            additional_legs,
        }
    }
    fn successors(&self) -> Vec<(Node<'a>, i64)> {
        [v2d(0, 0), v2d(0, -1), v2d(0, 1), v2d(-1, 0), v2d(1, 0)]
            .iter()
            .flat_map(|v| {
                let valley = self.valley;
                let pos = self.pos + v;
                let minute = self.minute + 1;
                let additional_legs = self.additional_legs
                    - if self.pos == self.target_pos() { 1 } else { 0 };
                if valley.is_allowed_pos(pos)
                    && !valley.is_blizzard_at(pos, minute)
                {
                    Some((
                        Node {
                            valley,
                            pos,
                            minute,
                            additional_legs,
                        },
                        1,
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    fn start_pos(&self) -> Point2d {
        self.valley.start_pos()
    }
    fn goal_pos(&self) -> Point2d {
        self.valley.goal_pos()
    }
    fn target_pos(&self) -> Point2d {
        if self.additional_legs % 2 == 0 {
            self.goal_pos()
        } else {
            self.start_pos()
        }
    }
    fn full_distance(&self) -> i64 {
        self.start_pos().distance_l1(self.goal_pos())
    }
    fn heuristic(&self) -> i64 {
        self.pos.distance_l1(self.target_pos())
            + self.additional_legs * self.full_distance()
    }
    fn success(&self) -> bool {
        self.pos == self.target_pos() && self.additional_legs == 0
    }
}
impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node<'a>) -> bool {
        self.pos == other.pos
            && self.minute == other.minute
            && self.additional_legs == other.additional_legs
    }
}
impl<'a> Eq for Node<'a> {}
impl<'a> Hash for Node<'a> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.pos.hash(state);
        self.minute.hash(state);
        self.additional_legs.hash(state);
    }
}
impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.valley.bbox.y_range() {
            for x in self.valley.bbox.x_range() {
                let p = p2d(x, y);
                let c = {
                    if p == self.pos {
                        'E'
                    } else if self.valley.inner_bbox.contains(&p) {
                        let blizzards =
                            self.valley.blizzards_at(p, self.minute);
                        if blizzards.is_empty() {
                            '.'
                        } else {
                            let len = blizzards.len();
                            if len == 1 {
                                let d = Direction::try_from(blizzards[0].dir)
                                    .unwrap();
                                d.to_char()
                            } else {
                                char::from_digit(len as u32, 10).unwrap()
                            }
                        }
                    } else if p == self.valley.start_pos()
                        || p == self.valley.goal_pos()
                    {
                        '.'
                    } else {
                        '#'
                    }
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let valley = Valley::new(&input)?;

    let start_node1 = Node::start(&valley, 1);
    let search_result = astar(
        &start_node1,
        Node::successors,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) =
        search_result.ok_or_else(|| runtime_error!("no path found"))?;
    let result1 = cost;

    let start_node2 = Node::start(&valley, 3);
    let search_result = astar(
        &start_node2,
        Node::successors,
        Node::heuristic,
        Node::success,
    );
    let (_path, cost) =
        search_result.ok_or_else(|| runtime_error!("no path found"))?;
    let result2 = cost;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
