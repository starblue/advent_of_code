use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::alt;
use nom::do_parse;
use nom::line_ending;
use nom::many1;
use nom::named;
use nom::tag;
use nom::value;

use gamedim::p2d;
use gamedim::v2d;
use gamedim::Point2d;
use gamedim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum Dir {
    E,
    NE,
    NW,
    W,
    SW,
    SE,
}
const DIRS: [Dir; 6] = [Dir::E, Dir::NE, Dir::NW, Dir::W, Dir::SW, Dir::SE];
impl Dir {
    fn to_str(&self) -> &str {
        match self {
            Dir::E => "e",
            Dir::NE => "ne",
            Dir::NW => "nw",
            Dir::W => "w",
            Dir::SW => "sw",
            Dir::SE => "se",
        }
    }
}
impl Dir {
    /// Get the vector corresponding to a direction.
    ///
    /// One step in x direction is east, in y direction northwest,
    /// same as for Eisenstein numbers.
    fn to_v2d(&self) -> Vec2d {
        match self {
            Dir::E => v2d(1, 0),
            Dir::NE => v2d(1, 1),
            Dir::NW => v2d(0, 1),
            Dir::W => v2d(-1, 0),
            Dir::SW => v2d(-1, -1),
            Dir::SE => v2d(0, -1),
        }
    }
}
impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Clone, Debug)]
struct Tile(Vec<Dir>);
impl Tile {
    fn pos(&self) -> Point2d {
        self.0.iter().map(Dir::to_v2d).fold(p2d(0, 0), |p, v| p + v)
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

named!(dir<&str, Dir>,
    alt!(
        value!(Dir::E, tag!("e")) |
        value!(Dir::NE, tag!("ne")) |
        value!(Dir::NW, tag!("nw")) |
        value!(Dir::W, tag!("w")) |
        value!(Dir::SW, tag!("sw")) |
        value!(Dir::SE, tag!("se"))
    )
);

named!(tile<&str, Tile>,
    do_parse!(
        dirs: many1!(dir) >>
        line_ending >>
            (Tile(dirs))
    )
);

named!(input<&str, Vec<Tile>>,
    do_parse!(
        tiles: many1!(tile) >>
        line_ending >>
            (tiles)
    )
);

fn neighbors(p: Point2d) -> Vec<Point2d> {
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
            .flat_map(|&p| neighbors(p).into_iter())
            .collect::<HashSet<Point2d>>();
        let positions = black_positions
            .union(&neighbor_positions)
            .cloned()
            .collect::<HashSet<Point2d>>();
        let new_black_positions = positions
            .into_iter()
            .filter(|&p| {
                let black = black_positions.contains(&p);
                let count = neighbors(p)
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
