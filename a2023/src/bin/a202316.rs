use core::fmt;

use std::collections::HashSet;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    NorthEastMirror,
    SouthEastMirror,
    HorizontalSplitter,
    VerticalSplitter,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::NorthEastMirror => '/',
            Square::SouthEastMirror => '\\',
            Square::HorizontalSplitter => '-',
            Square::VerticalSplitter => '|',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
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
    alt((
        value(Square::Empty, char('.')),
        value(Square::NorthEastMirror, char('/')),
        value(Square::SouthEastMirror, char('\\')),
        value(Square::HorizontalSplitter, char('-')),
        value(Square::VerticalSplitter, char('|')),
    ))(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    p: Point2d,
    v: Vec2d,
}
impl State {
    fn next_velocities(&self, square: Square) -> Vec<Vec2d> {
        let mut result = Vec::new();
        match square {
            Square::Empty => result.push(self.v),
            Square::NorthEastMirror => {
                result.push({
                    if self.v == v2d(1, 0) {
                        v2d(0, -1)
                    } else if self.v == v2d(0, -1) {
                        v2d(1, 0)
                    } else if self.v == v2d(-1, 0) {
                        v2d(0, 1)
                    } else {
                        // self.v == v2d(0, 1)
                        v2d(-1, 0)
                    }
                })
            }
            Square::SouthEastMirror => {
                result.push({
                    if self.v == v2d(1, 0) {
                        v2d(0, 1)
                    } else if self.v == v2d(0, -1) {
                        v2d(-1, 0)
                    } else if self.v == v2d(-1, 0) {
                        v2d(0, -1)
                    } else {
                        // self.v == v2d(0, 1)
                        v2d(1, 0)
                    }
                })
            }
            Square::HorizontalSplitter => {
                if self.v == v2d(0, -1) || self.v == v2d(0, 1) {
                    result.push(v2d(1, 0));
                    result.push(v2d(-1, 0));
                } else {
                    result.push(self.v);
                }
            }
            Square::VerticalSplitter => {
                if self.v == v2d(-1, 0) || self.v == v2d(1, 0) {
                    result.push(v2d(0, 1));
                    result.push(v2d(0, -1));
                } else {
                    result.push(self.v);
                }
            }
        }
        result
    }
    fn next_states(&self, input: &Input) -> Vec<State> {
        let mut result = Vec::new();
        if let Some(&square) = input.map.get(self.p) {
            for v in self.next_velocities(square) {
                let p = self.p + v;
                if input.bbox().contains(&p) {
                    result.push(State { p, v });
                }
            }
        }
        result
    }
}

fn energized_positions(input: &Input, initial_state: State) -> HashSet<Point2d> {
    let mut stack = vec![initial_state];
    let mut seen = HashSet::new();
    while let Some(state) = stack.pop() {
        if !seen.contains(&state) {
            seen.insert(state);
            let mut next_states = state.next_states(input);
            stack.append(&mut next_states);
        }
    }
    seen.iter().map(|s| s.p).collect::<HashSet<_>>()
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    // for y in input.bbox().y_range() {
    //     for x in input.bbox().x_range() {
    //         let p = p2d(x, y);
    //         if input.map[p] == Square::Empty {
    //             if seen_positions.contains(&p) {
    //                 print!("#");
    //             } else {
    //                 print!(".");
    //             }
    //         } else {
    //             print!("{}", input.map[p]);
    //         }
    //     }
    //     println!();
    // }

    let p = p2d(0, 0);
    let v = v2d(1, 0);
    let initial_state = State { p, v };
    let result1 = energized_positions(&input, initial_state).len();

    let initial_states = {
        input
            .bbox()
            .y_range()
            .map(|y| {
                let p = p2d(input.bbox().x_min(), y);
                let v = v2d(1, 0);
                State { p, v }
            })
            .chain(input.bbox().x_range().map(|x| {
                let p = p2d(x, input.bbox().y_max());
                let v = v2d(0, -1);
                State { p, v }
            }))
            .chain(input.bbox().y_range().map(|y| {
                let p = p2d(input.bbox().x_max(), y);
                let v = v2d(-1, 0);
                State { p, v }
            }))
            .chain(input.bbox().x_range().map(|x| {
                let p = p2d(x, input.bbox().y_min());
                let v = v2d(0, 1);
                State { p, v }
            }))
    };
    let result2 = initial_states
        .map(|initial_state| energized_positions(&input, initial_state).len())
        .max()
        .ok_or("internal error")?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
