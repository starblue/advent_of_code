use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::ops;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::map_opt;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::Array2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    // Point of interest
    Poi(u32),
    Wall,
    Open,
}
impl Square {
    fn is_open(&self) -> bool {
        match self {
            Square::Poi(_d) => true,
            Square::Wall => false,
            Square::Open => true,
        }
    }
    fn to_char(self) -> char {
        match self {
            Square::Poi(d) => char::from_digit(d, 10).unwrap(),
            Square::Wall => '#',
            Square::Open => '.',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Row(Vec<Square>);
impl Row {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
}
impl ops::Index<i64> for Row {
    type Output = Square;

    fn index(&self, index: i64) -> &Self::Output {
        if (0..self.len()).contains(&index) {
            &self.0[index as usize]
        } else {
            &Square::Wall
        }
    }
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for square in &self.0 {
            write!(f, "{}", square)?;
        }
        writeln!(f)
    }
}

fn digit(i: &str) -> IResult<&str, u32> {
    map_opt(one_of("0123456789"), |c| char::to_digit(c, 10))(i)
}

fn square_poi(i: &str) -> IResult<&str, Square> {
    let (i, d) = digit(i)?;
    Ok((i, Square::Poi(d)))
}
fn square(i: &str) -> IResult<&str, Square> {
    alt((
        square_poi,
        value(Square::Wall, char('#')),
        value(Square::Open, char('.')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    let (i, line) = many1(square)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn input(i: &str) -> IResult<&str, Vec<Vec<Square>>> {
    many1(line)(i)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Point2d,
    pois: Vec<Point2d>,
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

    let input = result.unwrap().1;
    // for line in &input {
    //     for c in line {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let map = Array2d::from_vec(input);
    let pois = map
        .bbox()
        .iter()
        .flat_map(|p| {
            if let Square::Poi(d) = map[p] {
                Some((d, p))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let start_pos = pois[&0];

    let start_node = Node {
        pos: start_pos,
        pois: pois
            .values()
            .filter(|&&poi| poi != start_pos)
            .cloned()
            .collect::<Vec<_>>(),
    };
    let successors = |n: &Node| {
        n.pos
            .neighbors_l1()
            .iter()
            .filter_map(|&new_pos| {
                if let Some(sq) = map.get(new_pos) {
                    if sq.is_open() {
                        let new_pois = n
                            .pois
                            .iter()
                            .filter(|&&poi| poi != new_pos)
                            .cloned()
                            .collect::<Vec<_>>();
                        Some((
                            Node {
                                pos: new_pos,
                                pois: new_pois,
                            },
                            1,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let heuristic_a = |n: &Node| {
        if let Some(d) = n.pois.iter().map(|&poi| n.pos.distance_l1(poi)).max() {
            d
        } else {
            0
        }
    };
    let success_a = |n: &Node| n.pois.is_empty();
    let search_result = astar(&start_node, successors, heuristic_a, success_a);
    let (_path, cost) = search_result.unwrap();
    let result_a = cost;

    let heuristic_b = |n: &Node| {
        if let Some(d) = n
            .pois
            .iter()
            .map(|&poi| n.pos.distance_l1(poi) + poi.distance_l1(start_pos))
            .max()
        {
            d
        } else {
            n.pos.distance_l1(start_pos)
        }
    };
    let success_b = |n: &Node| n.pos == start_pos && n.pois.is_empty();
    let search_result = astar(&start_node, successors, heuristic_b, success_b);
    let (_path, cost) = search_result.unwrap();
    let result_b = cost;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
