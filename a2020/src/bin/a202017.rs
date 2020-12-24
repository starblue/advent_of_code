use core::fmt;

use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::do_parse;
use nom::line_ending;
use nom::many1;
use nom::named;
use nom::value;

use gamedim::p3d;
use gamedim::p4d;
use gamedim::v3d;
use gamedim::v4d;
use gamedim::Point3d;
use gamedim::Point4d;

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

named!(cube<&str, Cube>,
    alt!(
        value!(Cube::Inactive, char!('.')) |
        value!(Cube::Active, char!('#'))
    )
);
named!(
    line<&str, Vec<Cube>>,
    many1!(cube)
);

named!(
    lines<&str, Vec<Vec<Cube>>>,
    many1!(
        do_parse!(
            line: line >>
            line_ending >> (line)
        )
    )
);

fn neighbors3d(p: Point3d<i64>) -> Vec<Point3d<i64>> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dy != 0 || dz != 0 {
                    result.push(p + v3d(dx, dy, dz))
                }
            }
        }
    }
    result
}
fn neighbors4d(p: Point4d<i64>) -> Vec<Point4d<i64>> {
    let mut result = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        result.push(p + v4d(dx, dy, dz, dw))
                    }
                }
            }
        }
    }
    result
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

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
            .flat_map(|&p| neighbors3d(p).into_iter())
            .collect::<HashSet<Point3d<i64>>>();
        let positions = active_positions
            .union(&neighbor_positions)
            .cloned()
            .collect::<HashSet<Point3d<i64>>>();
        let new_active_positions = positions
            .into_iter()
            .filter(|&p| {
                let active = active_positions.contains(&p);
                let count = neighbors3d(p)
                    .iter()
                    .filter(|np| active_positions.contains(np))
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
            .flat_map(|&p| neighbors4d(p).into_iter())
            .collect::<HashSet<Point4d<i64>>>();
        let positions = active_positions
            .union(&neighbor_positions)
            .cloned()
            .collect::<HashSet<Point4d<i64>>>();
        let new_active_positions = positions
            .into_iter()
            .filter(|&p| {
                let active = active_positions.contains(&p);
                let count = neighbors4d(p)
                    .iter()
                    .filter(|np| active_positions.contains(np))
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
