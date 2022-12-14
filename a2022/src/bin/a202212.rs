use core::fmt;

use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

use util::runtime_error;

/// Return the Unicode scalar value of a `char`.
fn usv(c: char) -> i64 {
    i64::from(u32::from(c))
}

fn height(c: char) -> i64 {
    usv(c) - usv('a')
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Height(char),
    Start,
    End,
}
impl Square {
    fn height(&self) -> i64 {
        match self {
            Square::Height(c) => height(*c),
            Square::Start => height('a'),
            Square::End => height('z'),
        }
    }
    fn to_char(self) -> char {
        match self {
            Square::Height(c) => c,
            Square::Start => 'S',
            Square::End => 'E',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct HeightMap {
    map: Array2d<i64, Square>,
}
impl HeightMap {
    fn new(map: Array2d<i64, Square>) -> HeightMap {
        HeightMap { map }
    }
    fn height(&self, p: Point2d) -> Option<i64> {
        self.map.get(p).map(Square::height)
    }
    fn start_pos(&self) -> Option<Point2d> {
        self.bbox().iter().find(|&p| self.map[p] == Square::Start)
    }
    fn end_pos(&self) -> Option<Point2d> {
        self.bbox().iter().find(|&p| self.map[p] == Square::End)
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for HeightMap {
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

fn square_height(i: &str) -> IResult<&str, Square> {
    let (i, c) = satisfy(|c| c.is_ascii_lowercase())(i)?;
    Ok((i, Square::Height(c)))
}
fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Start, char('S')),
        value(Square::End, char('E')),
        square_height,
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn input(i: &str) -> IResult<&str, HeightMap> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((i, HeightMap::new(Array2d::from_vec(rows))))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let map = input.clone();
    let start_node = map
        .start_pos()
        .ok_or_else(|| runtime_error!("start position not found"))?;
    let target_pos = map
        .end_pos()
        .ok_or_else(|| runtime_error!("end position not found"))?;
    let successors = |&p: &Point2d| {
        let map = &map;
        let height = map.height(p).unwrap();
        p.neighbors_l1().filter_map(move |np: Point2d| {
            if let Some(new_height) = map.height(np) {
                if new_height <= height + 1 {
                    Some((np, 1))
                } else {
                    None
                }
            } else {
                None
            }
        })
    };
    let heuristic = |p: &Point2d| p.distance_l1(target_pos);
    let success = |p: &Point2d| *p == target_pos;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.ok_or_else(|| runtime_error!("no path found"))?;
    let result1 = cost;

    let map = input;
    let start_node = map
        .end_pos()
        .ok_or_else(|| runtime_error!("end position not found"))?;
    let successors = |&p: &Point2d| {
        let map = &map;
        let height = map.height(p).unwrap();
        p.neighbors_l1().filter_map(move |np: Point2d| {
            if let Some(new_height) = map.height(np) {
                if new_height + 1 >= height {
                    Some((np, 1))
                } else {
                    None
                }
            } else {
                None
            }
        })
    };
    let heuristic = |_p: &Point2d| 0;
    let success = |&p: &Point2d| map.height(p).unwrap() == 0;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.ok_or_else(|| runtime_error!("no path found"))?;
    let result2 = cost;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
