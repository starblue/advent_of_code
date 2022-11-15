use std::collections::HashSet;
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

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum HexDir {
    E,
    NE,
    NW,
    W,
    SW,
    SE,
}
const DIRS: [HexDir; 6] = [
    HexDir::E,
    HexDir::NE,
    HexDir::NW,
    HexDir::W,
    HexDir::SW,
    HexDir::SE,
];
impl HexDir {
    fn to_str(&self) -> &str {
        match self {
            HexDir::E => "e",
            HexDir::NE => "ne",
            HexDir::NW => "nw",
            HexDir::W => "w",
            HexDir::SW => "sw",
            HexDir::SE => "se",
        }
    }
}
impl HexDir {
    /// Get the vector corresponding to a direction.
    ///
    /// One step in x direction is east, in y direction northwest,
    /// same as for Eisenstein numbers.
    fn to_v2d(&self) -> Vec2d {
        match self {
            HexDir::E => v2d(1, 0),
            HexDir::NE => v2d(1, 1),
            HexDir::NW => v2d(0, 1),
            HexDir::W => v2d(-1, 0),
            HexDir::SW => v2d(-1, -1),
            HexDir::SE => v2d(0, -1),
        }
    }
}
impl fmt::Display for HexDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Clone, Debug)]
struct Tile(Vec<HexDir>);
impl Tile {
    fn pos(&self) -> Point2d {
        self.0
            .iter()
            .map(HexDir::to_v2d)
            .fold(p2d(0, 0), |p, v| p + v)
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for dir in &self.0 {
            write!(f, "{}", dir)?;
        }
        Ok(())
    }
}

fn dir(i: &str) -> IResult<&str, HexDir> {
    alt((
        value(HexDir::E, tag("e")),
        value(HexDir::NE, tag("ne")),
        value(HexDir::NW, tag("nw")),
        value(HexDir::W, tag("w")),
        value(HexDir::SW, tag("sw")),
        value(HexDir::SE, tag("se")),
    ))(i)
}

fn tile(i: &str) -> IResult<&str, Tile> {
    let (i, dirs) = many1(dir)(i)?;
    Ok((i, Tile(dirs)))
}

fn input(i: &str) -> IResult<&str, Vec<Tile>> {
    separated_list1(line_ending, tile)(i)
}

fn hex_neighbors(p: Point2d) -> Vec<Point2d> {
    DIRS.iter().map(|d| p + d.to_v2d()).collect::<Vec<_>>()
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let tiles = result.unwrap().1;
    // for t in &tiles {
    //     println!("{}", t);
    // }

    let mut black_positions = HashSet::new();
    for t in &tiles {
        let p = t.pos();
        if black_positions.contains(&p) {
            black_positions.remove(&p);
        } else {
            black_positions.insert(p);
        }
    }
    let result_a = black_positions.len();

    for _ in 0..100 {
        let neighbor_positions = black_positions
            .iter()
            .flat_map(|&p| hex_neighbors(p).into_iter())
            .collect::<HashSet<Point2d>>();
        let positions = black_positions
            .union(&neighbor_positions)
            .cloned()
            .collect::<HashSet<Point2d>>();
        let new_black_positions = positions
            .into_iter()
            .filter(|&p| {
                let black = black_positions.contains(&p);
                let count = hex_neighbors(p)
                    .iter()
                    .filter(|np| black_positions.contains(np))
                    .count();
                if black {
                    count == 1 || count == 2
                } else {
                    count == 2
                }
            })
            .collect::<HashSet<_>>();

        black_positions = new_black_positions;
    }

    let result_b = black_positions.len();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
