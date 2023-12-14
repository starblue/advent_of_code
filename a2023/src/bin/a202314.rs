use core::fmt;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    RoundedRock,
    CubeShapedRock,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::RoundedRock => 'O',
            Square::CubeShapedRock => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn total_load(&self) -> i64 {
        self.bbox()
            .iter()
            .filter(|&p| self.map[p] == Square::RoundedRock)
            .map(|p| self.bbox().y_end() - p.y())
            .sum::<i64>()
    }
    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }
    fn roll_north(&mut self) {
        self.roll(v2d(0, -1), p2d(0, 0), v2d(1, 0));
    }
    fn roll_west(&mut self) {
        self.roll(v2d(-1, 0), p2d(0, 0), v2d(0, 1));
    }
    fn roll_south(&mut self) {
        self.roll(v2d(0, 1), p2d(0, self.bbox().y_max()), v2d(1, 0));
    }
    fn roll_east(&mut self) {
        self.roll(v2d(1, 0), p2d(self.bbox().x_max(), 0), v2d(0, 1));
    }
    fn roll(&mut self, dir: Vec2d, p_start: Point2d, dir_row: Vec2d) {
        let mut p_row = p_start;
        let mut p0 = p_row;
        while self.bbox().contains(&p0) {
            if self.map[p0] == Square::RoundedRock {
                let mut p1 = p0;
                while self.map.get(p1 + dir) == Some(&Square::Empty) {
                    p1 += dir;
                }
                self.map[p0] = Square::Empty;
                self.map[p1] = Square::RoundedRock;
            }
            p0 += dir_row;
            if !self.bbox().contains(&p0) {
                // We completed the row, go to the next one.
                p_row -= dir;
                p0 = p_row;
            }
        }
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range() {
            for x in self.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char('.')),
        value(Square::RoundedRock, char('O')),
        value(Square::CubeShapedRock, char('#')),
    ))(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut input1 = input.clone();
    input1.roll_north();
    let result1 = input1.total_load();

    let mut input2 = input.clone();
    let mut seen = HashMap::new();
    let mut count = 0;
    let mut loop_begin_count = 0;
    while count < 1_000_000_000 {
        seen.insert(input2.clone(), count);

        input2.cycle();
        count += 1;

        if let Some(&c) = seen.get(&input2) {
            loop_begin_count = c;
            break;
        }
    }
    let loop_count = count - loop_begin_count;
    let remaining_count = (1_000_000_000 - count) % loop_count;
    for _ in 0..remaining_count {
        input2.cycle();
    }
    let result2 = input2.total_load();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
