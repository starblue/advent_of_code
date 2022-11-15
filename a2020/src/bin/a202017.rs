use core::fmt;

use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p3d;
use lowdim::p4d;
use lowdim::Point3d;
use lowdim::Point4d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cube {
    Inactive,
    Active,
}
impl Cube {
    fn to_char(&self) -> char {
        match self {
            Cube::Inactive => '.',
            Cube::Active => '#',
        }
    }
}
impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn cube(i: &str) -> IResult<&str, Cube> {
    alt((
        value(Cube::Inactive, char('.')),
        value(Cube::Active, char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Cube>> {
    let (i, line) = many1(cube)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn lines(i: &str) -> IResult<&str, Vec<Vec<Cube>>> {
    many1(line)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = lines(&input_data);
    //println!("{:?}", result);

    let map = result.unwrap().1;
    // for v in &map {
    //     for c in v {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let mut active_positions = HashSet::new();
    for (v, y) in map.iter().zip(0..) {
        for (&c, x) in v.iter().zip(0..) {
            if c == Cube::Active {
                active_positions.insert(p3d(x, y, 0_i64));
            }
        }
    }

    for _ in 0..6 {
        let neighbor_positions = active_positions
            .iter()
            .flat_map(|&p| p.neighbors_l_infty().into_iter())
            .collect::<HashSet<Point3d>>();
        let positions = active_positions
            .union(&neighbor_positions)
            .cloned()
            .collect::<HashSet<Point3d>>();
        let new_active_positions = positions
            .into_iter()
            .filter(|&p| {
                let active = active_positions.contains(&p);
                let count = p
                    .neighbors_l_infty()
                    .filter(|np| active_positions.contains(&np))
                    .count();
                if active {
                    count == 2 || count == 3
                } else {
                    count == 3
                }
            })
            .collect::<HashSet<_>>();

        active_positions = new_active_positions;
    }
    let result_a = active_positions.len();

    let mut active_positions = HashSet::new();
    for (v, y) in map.iter().zip(0..) {
        for (&c, x) in v.iter().zip(0..) {
            if c == Cube::Active {
                active_positions.insert(p4d(x, y, 0_i64, 0_i64));
            }
        }
    }

    for _ in 0..6 {
        let neighbor_positions = active_positions
            .iter()
            .flat_map(|&p| p.neighbors_l_infty().into_iter())
            .collect::<HashSet<Point4d>>();
        let positions = active_positions
            .union(&neighbor_positions)
            .cloned()
            .collect::<HashSet<Point4d>>();
        let new_active_positions = positions
            .into_iter()
            .filter(|&p| {
                let active = active_positions.contains(&p);
                let count = p
                    .neighbors_l_infty()
                    .filter(|np| active_positions.contains(&np))
                    .count();
                if active {
                    count == 2 || count == 3
                } else {
                    count == 3
                }
            })
            .collect::<HashSet<_>>();

        active_positions = new_active_positions;
    }
    let result_b = active_positions.len();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
