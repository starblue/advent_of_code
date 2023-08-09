use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

use util::runtime_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    None,
    Open,
    Wall,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::None => ' ',
            Square::Open => '.',
            Square::Wall => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Map {
    map: Array2d<i64, Square>,
}
impl Map {
    fn new(map: Array2d<i64, Square>) -> Map {
        Map { map }
    }
    fn start_position(&self) -> util::Result<Point2d> {
        let bbox = self.bbox();
        let y = bbox.y_min();
        for x in bbox.x_range() {
            let p = p2d(x, y);
            if self.map[p] == Square::Open {
                return Ok(p);
            }
        }
        Err(runtime_error!("no start position found"))
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Map {
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

#[derive(Clone, Debug)]
enum Instruction {
    Forward(i64),
    Left,
    Right,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Forward(n) => write!(f, "{}", n),
            Instruction::Left => write!(f, "L"),
            Instruction::Right => write!(f, "R"),
        }
    }
}

#[derive(Clone, Debug)]
struct Path {
    instructions: Vec<Instruction>,
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for instruction in &self.instructions {
            write!(f, "{}", instruction)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    map: Map,
    path: Path,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.map)?;
        writeln!(f, "{}", self.path)?;
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::None, char(' ')),
        value(Square::Open, char('.')),
        value(Square::Wall, char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn map(i: &str) -> IResult<&str, Map> {
    let (i, mut rows) = separated_list1(line_ending, line)(i)?;
    // Pad all rows to the same length.
    let len = rows.iter().map(|r| r.len()).max().unwrap();
    for row in &mut rows {
        while row.len() < len {
            row.push(Square::None);
        }
    }
    Ok((i, Map::new(Array2d::from_vec(rows))))
}

fn instruction_forward(i: &str) -> IResult<&str, Instruction> {
    let (i, n) = int(i)?;
    Ok((i, Instruction::Forward(n)))
}
fn instruction(i: &str) -> IResult<&str, Instruction> {
    alt((
        instruction_forward,
        value(Instruction::Left, char('L')),
        value(Instruction::Right, char('R')),
    ))(i)
}

fn path(i: &str) -> IResult<&str, Path> {
    let (i, instructions) = many1(instruction)(i)?;
    Ok((i, Path { instructions }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, map) = map(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, path) = path(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Input { map, path }))
}

fn facing(v: Vec2d) -> util::Result<i64> {
    if v == v2d(1, 0) {
        Ok(0)
    } else if v == v2d(0, 1) {
        Ok(1)
    } else if v == v2d(-1, 0) {
        Ok(2)
    } else if v == v2d(0, -1) {
        Ok(3)
    } else {
        Err(runtime_error!("not a valid direction"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: Point2d,
    dir: Vec2d,
}
impl State {
    fn start(map: &Map) -> util::Result<State> {
        let pos = map.start_position()?;
        let dir = v2d(1, 0);
        Ok(State { pos, dir })
    }
    fn rotate_left(&mut self) {
        // Left and right are swapped, because the y coordinate increases
        // downwards.
        self.dir = self.dir.rotate_right();
    }
    fn rotate_right(&mut self) {
        self.dir = self.dir.rotate_left();
    }
    fn password(&self) -> util::Result<i64> {
        Ok(1000 * (self.pos.y() + 1) + 4 * (self.pos.x() + 1) + facing(self.dir)?)
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut state = State::start(&input.map)?;
    let map = &input.map.map;
    for instruction in &input.path.instructions {
        match instruction {
            Instruction::Forward(n) => {
                let mut next_pos = state.pos;
                let mut steps = 0;
                while steps < *n {
                    next_pos = (next_pos + state.dir) % map.bbox();
                    match map[next_pos] {
                        Square::None => {
                            // Move over this square, it doesn't exist.
                        }
                        Square::Open => {
                            // We made a step.
                            state.pos = next_pos;
                            steps += 1;
                        }
                        Square::Wall => {
                            // We hit a wall, stay where we are and finish.
                            break;
                        }
                    }
                }
            }
            Instruction::Left => {
                state.rotate_left();
            }
            Instruction::Right => {
                state.rotate_right();
            }
        }
    }
    let result1 = state.password()?;

    // Shape of the cube surface:
    //
    //   0 1 2 3
    // 0   aCbDc
    //     A#|#E
    // 1   d-eFf
    //     B#F
    // 2 gBh-i
    //   A#|#E
    // 3 j-kGl
    //   C#G
    // 4 mDn

    let mut state = State::start(&input.map)?;
    let mut jumps = HashMap::new();
    // The origin.
    let po = p2d(0_i64, 0_i64);
    // Corner points, need to be scaled up.
    let (pa, pb, pc) = (p2d(1, 0), p2d(2, 0), p2d(3, 0));
    let (pd, pe, pf) = (p2d(1, 1), p2d(2, 1), p2d(3, 1));
    let (pg, ph, pi) = (p2d(0, 2), p2d(1, 2), p2d(2, 2));
    let (pj, pk, pl) = (p2d(0, 3), p2d(1, 3), p2d(2, 3));
    let (pm, pn) = (p2d(0, 4), p2d(1, 4));
    // Directions
    let dir_e = v2d(1, 0);
    let dir_n = v2d(0, -1);
    let dir_w = v2d(-1, 0);
    let dir_s = v2d(0, 1);

    // p00 is glued to p10, and p01 to p11.
    // The direction is the direction across the edge.
    for ((p00, p01, dir0), (p10, p11, dir1)) in vec![
        ((pa, pd, dir_w), (pj, pg, dir_w)), // edge A
        ((pd, ph, dir_w), (pg, ph, dir_n)), // edge B
        ((pa, pb, dir_n), (pj, pm, dir_w)), // edge C
        ((pb, pc, dir_n), (pm, pn, dir_s)), // edge D
        ((pc, pf, dir_e), (pl, pi, dir_e)), // edge E
        ((pe, pf, dir_s), (pe, pi, dir_e)), // edge F
        ((pk, pl, dir_s), (pk, pn, dir_e)), // edge G
    ] {
        // Compute start and step for side 0 of the edge.

        // Scale up the first point.
        let mut p0 = po + 50 * (p00 - po);
        if dir0.x() > 0 || dir0.y() > 0 {
            // Correct right and bottom edges.
            // Move back to a real square as the jump-off point.
            p0 -= dir0;
        }
        let v0 = p01 - p00;
        if v0.x() < 0 || v0.y() < 0 {
            // Correct for reversed direction of an edge.
            // Go one step along the edge so that offset 49 maps to zero
            // and vice-versa.
            p0 += v0;
        }

        // Compute start and step for side 1.
        let mut p1 = po + 50 * (p10 - po);
        if dir1.x() > 0 || dir1.y() > 0 {
            p1 -= dir1;
        }
        let v1 = p11 - p10;
        if v1.x() < 0 || v1.y() < 0 {
            p1 += v1;
        }

        for i in 0..50 {
            // Jump from side 0 to side 1.
            let state_in = State {
                pos: p0 + i * v0,
                dir: dir0,
            };
            let state_out = State {
                pos: p1 + i * v1,
                dir: -dir1,
            };
            jumps.insert(state_in, state_out);

            // Jump from side 1 to side 0.
            let state_in = State {
                pos: p1 + i * v1,
                dir: dir1,
            };
            let state_out = State {
                pos: p0 + i * v0,
                dir: -dir0,
            };
            jumps.insert(state_in, state_out);
        }
    }

    for instruction in &input.path.instructions {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    let next_state;
                    if let Some(&new_state) = jumps.get(&state) {
                        next_state = new_state;
                    } else {
                        next_state = State {
                            pos: state.pos + state.dir,
                            dir: state.dir,
                        };
                    }
                    match map[next_state.pos] {
                        Square::None => {
                            // This must not happen.
                            return Err(runtime_error!("we fell off the edge"));
                        }
                        Square::Open => {
                            // We made a step.
                            state = next_state;
                        }
                        Square::Wall => {
                            // We hit a wall, stay where we are.
                            break;
                        }
                    }
                }
            }
            Instruction::Left => {
                state.rotate_left();
            }
            Instruction::Right => {
                state.rotate_right();
            }
        }
    }
    let result2 = state.password()?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use lowdim::v2d;

    use util::Result;

    use crate::facing;

    #[test]
    fn test_facing() -> util::Result<()> {
        assert_eq!(0, facing(v2d(1, 0))?);
        assert_eq!(1, facing(v2d(0, 1))?);
        assert_eq!(2, facing(v2d(-1, 0))?);
        assert_eq!(3, facing(v2d(0, -1))?);
        Ok(())
    }
}
