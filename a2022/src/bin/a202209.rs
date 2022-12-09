use core::fmt;
use core::str::FromStr;
use std::collections::HashSet;

use std::error;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many0;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;
use lowdim::Vector;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn to_vec2d(self) -> Vec2d {
        match self {
            Direction::Up => v2d(0, 1),
            Direction::Down => v2d(0, -1),
            Direction::Left => v2d(-1, 0),
            Direction::Right => v2d(1, 0),
        }
    }
    fn to_char(self) -> char {
        match self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
struct Motion {
    direction: Direction,
    distance: i64,
}
impl fmt::Display for Motion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.distance,)
    }
}

#[derive(Clone, Debug)]
struct Input {
    motions: Vec<Motion>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for motion in &self.motions {
            writeln!(f, "{}", motion)?;
        }
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(i)
}

fn motion(i: &str) -> IResult<&str, Motion> {
    let (i, direction) = direction(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, distance) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Motion {
            direction,
            distance,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, motions) = many0(motion)(i)?;
    Ok((i, Input { motions }))
}

#[derive(Clone, Debug)]
struct Rope {
    knots: Vec<Point2d>,
}
impl Rope {
    fn new(len: usize) -> Rope {
        let knots = (0..len).map(|_| p2d(0, 0)).collect::<Vec<_>>();
        Rope { knots }
    }
    fn execute_step(&mut self, d: Direction) {
        self.knots[0] += d.to_vec2d();
        self.update_tail();
    }
    fn update_tail(&mut self) {
        for i in 1..self.knots.len() {
            if self.knots[i].distance_l_infty(self.knots[i - 1]) > 1 {
                let delta = self.knots[i - 1] - self.knots[i];
                self.knots[i] += delta.signum();
            }
        }
    }
    fn tail(&self) -> Point2d {
        *self.knots.last().unwrap()
    }
}
impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for knot in &self.knots {
            write!(f, "{}{:?}", sep, knot)?;
            sep = "-";
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut visited = HashSet::new();
    let mut rope = Rope::new(2);
    visited.insert(rope.tail());
    for motion in &input.motions {
        for _ in 0..motion.distance {
            rope.execute_step(motion.direction);
            visited.insert(rope.tail());
        }
    }
    let result1 = visited.len();

    let mut visited = HashSet::new();
    let mut rope = Rope::new(10);
    visited.insert(rope.tail());
    for motion in &input.motions {
        for _ in 0..motion.distance {
            rope.execute_step(motion.direction);
            visited.insert(rope.tail());
        }
    }
    let result2 = visited.len();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
