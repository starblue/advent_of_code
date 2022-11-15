use std::fmt;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Herd {
    South,
    East,
}
impl Herd {
    fn dir(&self) -> Vec2d {
        match self {
            Herd::East => v2d(1, 0),
            Herd::South => v2d(0, 1),
        }
    }
    fn to_char(self) -> char {
        match self {
            Herd::East => '>',
            Herd::South => 'v',
        }
    }
}
impl fmt::Display for Herd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Cucumber(Herd),
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Cucumber(herd) => herd.to_char(),
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Region {
    map: Array2d<i64, Square>,
}
impl Region {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn get(&self, p: Point2d) -> Option<&Square> {
        self.map.get(p)
    }
    fn is_empty(&self, p: Point2d) -> bool {
        self.get(p) == Some(&Square::Empty)
    }
    fn is_cucumber(&self, p: Point2d, herd: Herd) -> bool {
        self.get(p) == Some(&Square::Cucumber(herd))
    }
    fn move_dest(&self, p: Point2d, herd: Herd) -> Point2d {
        (p + herd.dir()) % self.bbox()
    }
    fn step_herd(&mut self, herd: Herd) -> bool {
        let mut moves = Vec::new();
        for p in self.bbox() {
            let new_p = self.move_dest(p, herd);
            if self.is_cucumber(p, herd) && self.is_empty(new_p) {
                moves.push((p, new_p));
            }
        }
        let moved = !moves.is_empty();
        for (p, new_p) in moves {
            self.map[p] = Square::Empty;
            self.map[new_p] = Square::Cucumber(herd);
        }
        moved
    }
    fn step(&mut self) -> bool {
        let east_moved = self.step_herd(Herd::East);
        let south_moved = self.step_herd(Herd::South);
        east_moved || south_moved
    }
}
impl fmt::Display for Region {
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

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, tag(".")),
        value(Square::Cucumber(Herd::East), tag(">")),
        value(Square::Cucumber(Herd::South), tag("v")),
    ))(i)
}

fn row(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn input(i: &str) -> IResult<&str, Region> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    Ok((
        i,
        Region {
            map: Array2d::from_vec(rows),
        },
    ))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let mut region = input;
    let mut step_count = 0;
    while region.step() {
        step_count += 1;
    }
    let result_a = step_count + 1;

    println!("a: {}", result_a);
}
