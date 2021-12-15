use std::fmt;
use std::io;
use std::io::Read;

use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    risk_level: i64,
}
impl Location {
    fn to_char(self) -> char {
        char::from_digit(u32::try_from(self.risk_level).unwrap(), 10).unwrap()
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct RiskLevelMap {
    map: Array2d<i64, Location>,
}
impl RiskLevelMap {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn risk_level(&self, p: Point2d) -> Option<i64> {
        self.map.get(p).map(|location| location.risk_level)
    }
}
impl fmt::Display for RiskLevelMap {
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

#[derive(Clone, Debug)]
struct BigMap {
    map: RiskLevelMap,
}
impl BigMap {
    fn new(map: RiskLevelMap) -> BigMap {
        BigMap { map }
    }
    fn bbox(&self) -> BBox2d {
        let inner_bbox = self.map.bbox();
        BBox2d::new(inner_bbox.min(), 5 * inner_bbox.lengths())
    }
    fn risk_level(&self, p: Point2d) -> Option<i64> {
        if self.bbox().contains(&p) {
            let x_len = self.map.bbox().x_len();
            let y_len = self.map.bbox().y_len();
            let tile_x = p.x() / x_len;
            let tile_y = p.y() / y_len;
            let inner_x = p.x() % x_len;
            let inner_y = p.y() % y_len;
            let inner_p = p2d(inner_x, inner_y);
            let inner_risk_level = self.map.risk_level(inner_p).unwrap();
            let risk_level = (inner_risk_level + tile_x + tile_y - 1) % 9 + 1;
            Some(risk_level)
        } else {
            None
        }
    }
}
impl fmt::Display for BigMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range() {
            for x in self.bbox().x_range() {
                write!(f, "{}", self.risk_level(p2d(x, y)).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn location(i: &str) -> IResult<&str, Location> {
    let (i, c) = one_of("0123456789")(i)?;
    Ok((
        i,
        Location {
            risk_level: i64::from(c.to_digit(10).unwrap()),
        },
    ))
}

fn line(i: &str) -> IResult<&str, Vec<Location>> {
    many1(location)(i)
}

fn input(i: &str) -> IResult<&str, RiskLevelMap> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((
        i,
        RiskLevelMap {
            map: Array2d::from_vec(rows),
        },
    ))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let map = result.unwrap().1;
    // println!("{}", map);

    let bbox = map.bbox();
    let start_node = bbox.min();
    let target_pos = bbox.max();
    let successors = |p: &Point2d| {
        p.neighbors_l1()
            .filter_map(|np| map.risk_level(np).map(|risk_level| (np, risk_level)))
    };
    let heuristic = |p: &Point2d| p.distance_l1(target_pos);
    let success = |p: &Point2d| *p == target_pos;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.unwrap();
    let result_a = cost;

    let map = BigMap::new(map);
    let bbox = map.bbox();
    let start_node = bbox.min();
    let target_pos = bbox.max();
    let successors = |p: &Point2d| {
        p.neighbors_l1()
            .filter_map(|np| map.risk_level(np).map(|risk_level| (np, risk_level)))
    };
    let heuristic = |p: &Point2d| p.distance_l1(target_pos);
    let success = |p: &Point2d| *p == target_pos;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.unwrap();
    let result_b = cost;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
