use std::collections::HashSet;
use std::fmt;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    Obstacle,
    Start,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Obstacle => '#',
            Square::Start => '^',
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
        value(Square::Obstacle, char('#')),
        value(Square::Start, char('^')),
    ))(i)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Point2d,
    dir: Vec2d,
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
            .ok_or("starting position not found")?)
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn loops(&self, start_state: State) -> bool {
        let mut states_visited = HashSet::new();
        let State { mut pos, mut dir } = start_state;
        while self.bbox().contains(&pos) {
            let state = State { pos, dir };
            if states_visited.contains(&state) {
                return true;
            }
            states_visited.insert(state);
            if self.map.get(pos + dir) == Some(&Square::Obstacle) {
                dir = dir.rotate_right();
            } else {
                pos += dir;
            }
        }
        false
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range().rev() {
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
    let (i, mut lines) = separated_list1(line_ending, many1(square))(i)?;

    // The y coordinate increases from the bottom, i.e. here from the end.
    lines.reverse();

    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let bbox = input.bbox();

    let mut pos = input.start_pos()?;
    let mut dir = v2d(0, 1);
    let mut visited = HashSet::new();
    while bbox.contains(&pos) {
        visited.insert(pos);
        let pos1 = pos + dir;
        if input.map.get(pos1) == Some(&Square::Obstacle) {
            dir = dir.rotate_right();
        } else {
            // No obstacle ahead, walk one step.
            pos += dir;
        }
    }
    let result1 = visited.len();

    let mut input = input;
    let mut loop_positions = HashSet::new();
    let mut pos = input.start_pos()?;
    let mut dir = v2d(0, 1);
    let mut visited = HashSet::new();
    while bbox.contains(&pos) {
        visited.insert(pos);
        let pos1 = pos + dir;
        if input.map.get(pos1) == Some(&Square::Obstacle) {
            dir = dir.rotate_right();
        } else {
            if input.map.get(pos1) == Some(&Square::Empty) && !visited.contains(&pos1) {
                // Check if adding an obstacle at `pos1` leads to a loop.
                input.map[pos1] = Square::Obstacle;
                if input.loops(State { pos, dir }) {
                    loop_positions.insert(pos1);
                }
                input.map[pos1] = Square::Empty;
            }

            // No obstacle ahead, walk one step.
            pos += dir;
        }
    }
    let result2 = loop_positions.len();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
