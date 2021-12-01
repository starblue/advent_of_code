use core::fmt;

use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::Array2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Inactive,
    Active,
}
impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Inactive => '.',
            Cell::Active => '#',
        }
    }
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn cell(i: &str) -> IResult<&str, Cell> {
    alt((
        value(Cell::Inactive, char('.')),
        value(Cell::Active, char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Cell>> {
    let (i, line) = many1(cell)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn lines(i: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    many1(line)(i)
}

fn is_corner(map: &Array2d<i32, Cell>, p: Point2d<i32>) -> bool {
    let bounds = map.bbox();
    (p.x() == bounds.x_min() || p.x() == bounds.x_max())
        && (p.y() == bounds.y_min() || p.y() == bounds.y_max())
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

    let input = result.unwrap().1;
    // for v in &map {
    //     for c in v {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let steps = 100;

    let mut map = Array2d::<i32, _>::from_vec(input.clone());
    for _ in 0..steps {
        let new_map = Array2d::with(map.bbox(), |p| {
            let count = p
                .neighbors_l_infty()
                .iter()
                .filter(|&&p| map.get(p) == Some(&Cell::Active))
                .count();
            if (count == 2 && map[p] == Cell::Active) || count == 3 {
                Cell::Active
            } else {
                Cell::Inactive
            }
        });
        map = new_map;
    }
    let result_a = map.iter().filter(|&&c| c == Cell::Active).count();

    let mut map = Array2d::<i32, _>::from_vec(input);
    for _ in 0..steps {
        let new_map = Array2d::with(map.bbox(), |p| {
            let count = p
                .neighbors_l_infty()
                .iter()
                .filter(|&&p| map.get(p) == Some(&Cell::Active))
                .count();
            if (count == 2 && map[p] == Cell::Active) || count == 3 || is_corner(&map, p) {
                Cell::Active
            } else {
                Cell::Inactive
            }
        });
        map = new_map;
    }
    let result_b = map.iter().filter(|&&c| c == Cell::Active).count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
