use core::fmt;

use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::bb2d;
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
    Box,
    Robot,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Wall => '#',
            Square::Box => 'O',
            Square::Robot => '@',
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
        value(Square::Box, char('O')),
        value(Square::Robot, char('@')),
    ))(i)
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
impl Move {
    fn delta(&self) -> Vec2d {
        match self {
            Move::Up => v2d(0, -1),
            Move::Down => v2d(0, 1),
            Move::Left => v2d(-1, 0),
            Move::Right => v2d(1, 0),
        }
    }
    fn to_char(self) -> char {
        match self {
            Move::Up => '^',
            Move::Down => 'v',
            Move::Left => '<',
            Move::Right => '>',
        }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn move_(i: &str) -> IResult<&str, Move> {
    alt((
        value(Move::Up, char('^')),
        value(Move::Down, char('v')),
        value(Move::Left, char('<')),
        value(Move::Right, char('>')),
    ))(i)
}

#[derive(Clone, Debug)]
struct Map {
    map: Array2d<i64, Square>,
}
impl Map {
    fn robot_pos(&self) -> Result<Point2d> {
        Ok(self
            .map
            .bbox()
            .iter()
            .find(|&p| self.map[p] == Square::Robot)
            .ok_or("robot not found")?)
    }
    fn can_move(&self, pos: &Point2d, m: Move) -> bool {
        let v = m.delta();
        let mut p1 = *pos + v;
        loop {
            if self.map[p1] == Square::Empty {
                return true;
            }
            if self.map[p1] == Square::Wall {
                return false;
            }
            p1 += v;
        }
    }
    fn execute_move(&mut self, pos: &mut Point2d, m: Move) {
        if self.can_move(pos, m) {
            let v = m.delta();
            let mut p1 = *pos + v;

            // Find the first empty square.
            while self.map[p1] != Square::Empty {
                p1 += v;
            }
            // Move any boxes.
            while p1 != *pos {
                self.map[p1] = self.map[p1 - v];
                p1 -= v;
            }

            // Move the robot.
            self.map[*pos] = Square::Empty;
            *pos += v;
            self.map[*pos] = Square::Robot;
        }
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Map {
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

fn map(i: &str) -> IResult<&str, Map> {
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Map { map }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WideMapSquare {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}
impl WideMapSquare {
    fn to_char(self) -> char {
        match self {
            WideMapSquare::Empty => '.',
            WideMapSquare::Wall => '#',
            WideMapSquare::BoxLeft => '[',
            WideMapSquare::BoxRight => ']',
            WideMapSquare::Robot => '@',
        }
    }
}
impl fmt::Display for WideMapSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct WideMap {
    map: Array2d<i64, WideMapSquare>,
}
impl WideMap {
    fn robot_pos(&self) -> Result<Point2d> {
        Ok(self
            .map
            .bbox()
            .iter()
            .find(|&p| self.map[p] == WideMapSquare::Robot)
            .ok_or("robot not found")?)
    }
    fn can_move(&self, pos: &Point2d, m: Move) -> bool {
        let v = m.delta();
        let mut stack = vec![*pos + v];
        let mut visited = HashSet::new();
        while let Some(p) = stack.pop() {
            if !visited.contains(&p) {
                visited.insert(p);
                match (self.map[p], m) {
                    (WideMapSquare::Wall, _) => return false,
                    (WideMapSquare::BoxLeft, Move::Up) => {
                        stack.push(p + v);
                        stack.push(p + v + v2d(1, 0));
                    }
                    (WideMapSquare::BoxLeft, Move::Down) => {
                        stack.push(p + v);
                        stack.push(p + v + v2d(1, 0));
                    }
                    (WideMapSquare::BoxLeft, Move::Left) => stack.push(p + v),
                    (WideMapSquare::BoxLeft, Move::Right) => stack.push(p + v),
                    (WideMapSquare::BoxRight, Move::Up) => {
                        stack.push(p + v);
                        stack.push(p + v - v2d(1, 0));
                    }
                    (WideMapSquare::BoxRight, Move::Down) => {
                        stack.push(p + v);
                        stack.push(p + v - v2d(1, 0));
                    }
                    (WideMapSquare::BoxRight, Move::Left) => stack.push(p + v),
                    (WideMapSquare::BoxRight, Move::Right) => stack.push(p + v),
                    (WideMapSquare::Empty, _) => (),
                    (WideMapSquare::Robot, _) => panic!("unexpected robot"),
                }
            }
        }
        true
    }
    fn execute_move(&mut self, pos: &mut Point2d, m: Move) {
        if self.can_move(pos, m) {
            let v = m.delta();

            // Find all boxes to move by depth-first search.
            let mut moved_boxes = HashSet::new();
            let mut stack = vec![*pos + v];
            let mut visited = HashSet::new();
            while let Some(p) = stack.pop() {
                if !visited.contains(&p) {
                    visited.insert(p);
                    match self.map[p] {
                        WideMapSquare::Wall => panic!("unexpected wall"),
                        WideMapSquare::BoxLeft => {
                            match m {
                                Move::Up => {
                                    stack.push(p + v);
                                    stack.push(p + v + v2d(1, 0));
                                }
                                Move::Down => {
                                    stack.push(p + v);
                                    stack.push(p + v + v2d(1, 0));
                                }
                                Move::Left => {
                                    stack.push(p + v);
                                }
                                Move::Right => {
                                    stack.push(p + v);
                                }
                            }
                            moved_boxes.insert(p);
                        }
                        WideMapSquare::BoxRight => {
                            match m {
                                Move::Up => {
                                    stack.push(p + v);
                                    stack.push(p + v - v2d(1, 0));
                                }
                                Move::Down => {
                                    stack.push(p + v);
                                    stack.push(p + v - v2d(1, 0));
                                }
                                Move::Left => stack.push(p + v),
                                Move::Right => stack.push(p + v),
                            }
                            moved_boxes.insert(p - v2d(1, 0));
                        }
                        WideMapSquare::Empty => (),
                        WideMapSquare::Robot => panic!("unexpected robot"),
                    }
                }
            }
            // Move the boxes.
            for &p in &moved_boxes {
                self.map[p] = WideMapSquare::Empty;
                self.map[p + v2d(1, 0)] = WideMapSquare::Empty;
            }
            for &p in &moved_boxes {
                self.map[p + v] = WideMapSquare::BoxLeft;
                self.map[p + v2d(1, 0) + v] = WideMapSquare::BoxRight;
            }

            // Move the robot.
            self.map[*pos] = WideMapSquare::Empty;
            *pos += v;
            self.map[*pos] = WideMapSquare::Robot;
        }
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl From<&Map> for WideMap {
    fn from(map: &Map) -> Self {
        let x_end = 2 * map.bbox().x_end();
        let bbox = bb2d(0..x_end, map.bbox().y_range());
        let map = Array2d::with(bbox, |p| {
            let p0 = p2d(p.x() / 2, p.y());
            let left = p.x() % 2 == 0;
            match map.map[p0] {
                Square::Empty => WideMapSquare::Empty,
                Square::Wall => WideMapSquare::Wall,
                Square::Box => {
                    if left {
                        WideMapSquare::BoxLeft
                    } else {
                        WideMapSquare::BoxRight
                    }
                }
                Square::Robot => {
                    if left {
                        WideMapSquare::Robot
                    } else {
                        WideMapSquare::Empty
                    }
                }
            }
        });
        WideMap { map }
    }
}
impl fmt::Display for WideMap {
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

#[derive(Clone, Debug)]
struct Input {
    map: Map,
    moves: Vec<Move>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.map)?;
        writeln!(f)?;
        for (i, m) in self.moves.iter().enumerate() {
            write!(f, "{}", m)?;
            if i % 1000 == 999 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn move_opt_line_ending(i: &str) -> IResult<&str, Move> {
    let (i, m) = move_(i)?;
    let (i, _) = opt(line_ending)(i)?;
    Ok((i, m))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, map) = map(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, moves) = many1(move_opt_line_ending)(i)?;
    Ok((i, Input { map, moves }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let bbox = input.map.bbox();

    let mut map = input.map.clone();
    let mut pos = map.robot_pos()?;

    // println!("Initial state:");
    // println!("{}", map);
    for &m in &input.moves {
        map.execute_move(&mut pos, m);

        // println!("Move {}:", m);
        // println!("{}", map);
    }
    let result1 = bbox
        .iter()
        .map(|p| {
            if map.map[p] == Square::Box {
                p.x() + 100 * p.y()
            } else {
                0
            }
        })
        .sum::<i64>();

    let mut map = WideMap::from(&input.map);
    let mut pos = map.robot_pos()?;

    // println!("Initial state:");
    // println!("{}", map);
    for &m in &input.moves {
        map.execute_move(&mut pos, m);

        // println!("Move {}:", m);
        // println!("{}", map);
    }
    let result2 = map
        .bbox()
        .iter()
        .map(|p| {
            if map.map[p] == WideMapSquare::BoxLeft {
                p.x() + 100 * p.y()
            } else {
                0
            }
        })
        .sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
