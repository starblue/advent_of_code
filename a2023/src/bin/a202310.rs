use core::fmt;

use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::Point2d;
use lowdim::Vec2d;

use util::runtime_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    E,
    N,
    W,
    S,
}
impl Direction {
    fn values() -> impl Iterator<Item = Direction> {
        [Direction::E, Direction::N, Direction::W, Direction::S].into_iter()
    }
    fn to_v2d(self) -> Vec2d {
        match self {
            Direction::E => v2d(1, 0),
            Direction::N => v2d(0, -1),
            Direction::W => v2d(-1, 0),
            Direction::S => v2d(0, 1),
        }
    }
    fn opposite(&self) -> Direction {
        match self {
            Direction::E => Direction::W,
            Direction::N => Direction::S,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Ground,
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
}
impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Ground => '.',
            Tile::NS => '|',
            Tile::EW => '-',
            Tile::NE => 'L',
            Tile::NW => 'J',
            Tile::SW => '7',
            Tile::SE => 'F',
            Tile::Start => 'S',
        }
    }
    /// Next direction when we arrive at a new tile with a given direction.
    ///
    /// Returns `None` if the tile is not properly connected or
    /// if it is the starting position.
    fn next_direction(&self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Tile::NS, direction) => Some(direction),
            (Tile::EW, direction) => Some(direction),
            (Tile::NE, Direction::S) => Some(Direction::E),
            (Tile::NE, Direction::W) => Some(Direction::N),
            (Tile::NW, Direction::S) => Some(Direction::W),
            (Tile::NW, Direction::E) => Some(Direction::N),
            (Tile::SW, Direction::N) => Some(Direction::W),
            (Tile::SW, Direction::E) => Some(Direction::S),
            (Tile::SE, Direction::N) => Some(Direction::E),
            (Tile::SE, Direction::W) => Some(Direction::S),
            _ => None,
        }
    }
    fn connections(self) -> HashSet<Direction> {
        match self {
            Tile::NS => HashSet::from([Direction::N, Direction::S]),
            Tile::EW => HashSet::from([Direction::E, Direction::W]),
            Tile::NE => HashSet::from([Direction::N, Direction::E]),
            Tile::NW => HashSet::from([Direction::N, Direction::W]),
            Tile::SW => HashSet::from([Direction::S, Direction::W]),
            Tile::SE => HashSet::from([Direction::S, Direction::E]),
            _ => HashSet::new(),
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Tile>,
}
impl Input {
    fn start_pos(&self) -> util::Result<Point2d> {
        self.map
            .bbox()
            .iter()
            .find(|&p| self.map[p] == Tile::Start)
            .ok_or(runtime_error!("starting position not found"))
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.map.bbox().y_range() {
            for x in self.map.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn tile(i: &str) -> IResult<&str, Tile> {
    alt((
        value(Tile::Ground, char('.')),
        value(Tile::NS, char('|')),
        value(Tile::EW, char('-')),
        value(Tile::NE, char('L')),
        value(Tile::NW, char('J')),
        value(Tile::SW, char('7')),
        value(Tile::SE, char('F')),
        value(Tile::Start, char('S')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Tile>> {
    let (i, line) = many1(tile)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, lines) = many1(line)(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let start_pos = input.start_pos()?;
    let mut loop_positions = None;
    let mut start_connections = HashSet::new();
    'outer: for start_direction in Direction::values() {
        let mut pos = start_pos;
        let mut dir = start_direction;
        let mut positions = Vec::new();
        loop {
            positions.push(pos);
            pos += dir.to_v2d();
            if let Some(&tile) = input.map.get(pos) {
                if let Some(new_dir) = tile.next_direction(dir) {
                    dir = new_dir;
                } else if tile == Tile::Start {
                    // Found the loop
                    loop_positions = Some(positions);
                    start_connections.insert(start_direction);
                    start_connections.insert(dir.opposite());
                    break 'outer;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    let loop_positions = loop_positions.ok_or(util::runtime_error!("loop not found"))?;
    let result1 = loop_positions.len() / 2;

    let loop_positions = loop_positions.into_iter().collect::<HashSet<_>>();
    let mut area = 0;
    for y in input.map.bbox().y_range() {
        let mut inside = false;
        let mut boundary_connection: Option<Direction> = None;
        for x in input.map.bbox().x_range() {
            let p = p2d(x, y);
            let tile = input.map[p];
            if loop_positions.contains(&p) {
                let connections = {
                    if tile == Tile::Start {
                        start_connections.clone()
                    } else {
                        tile.connections()
                    }
                };
                if let Some(dir) = boundary_connection {
                    if connections.contains(&dir.opposite()) {
                        // We left the boundary and crossed the loop.
                        inside = !inside;
                        boundary_connection = None;
                    } else if connections.contains(&dir) {
                        // We left the boundary but didn't cross the loop.
                        boundary_connection = None;
                    } else {
                        // We are still on the boundary, nothing changes.
                    }
                } else {
                    // We weren't on a boundary.
                    if connections.contains(&Direction::N) {
                        if connections.contains(&Direction::S) {
                            // We crossed a north-south tile of the loop.
                            inside = !inside;
                        } else {
                            // We entered the boundary,
                            // which is connected to the north.
                            boundary_connection = Some(Direction::N);
                        }
                    } else {
                        // We entered the boundary,
                        // which must be connected to the south.
                        boundary_connection = Some(Direction::S);
                    }
                }
            } else if inside {
                area += 1;
            }
        }
    }
    let result2 = area;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
