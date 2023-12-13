use core::fmt;

use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Ash,
    Rocks,
}
impl Square {
    fn invert(self) -> Square {
        match self {
            Square::Ash => Square::Rocks,
            Square::Rocks => Square::Ash,
        }
    }
    fn to_char(self) -> char {
        match self {
            Square::Ash => '.',
            Square::Rocks => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LineOfReflection {
    Vertical(i64),
    Horizontal(i64),
}
impl LineOfReflection {
    fn note(&self) -> i64 {
        match self {
            LineOfReflection::Vertical(n) => *n,
            LineOfReflection::Horizontal(n) => 100 * *n,
        }
    }
}

#[derive(Clone, Debug)]
struct Pattern {
    map: Array2d<i64, Square>,
}
impl Pattern {
    fn note(&self) -> Option<i64> {
        self.lines_of_reflection().map(|lor| lor.note()).next()
    }
    fn note2(&self) -> Option<i64> {
        let lof1 = self.lines_of_reflection().next().unwrap();
        let mut pattern = self.clone();
        for p in pattern.bbox() {
            pattern.invert_square(p);
            if let Some(lof) = pattern.lines_of_reflection().find(|&lof| lof != lof1) {
                return Some(lof.note());
            }
            pattern.invert_square(p);
        }
        None
    }
    fn invert_square(&mut self, p: Point2d) {
        self.map[p] = self.map[p].invert();
    }

    fn lines_of_reflection(&self) -> impl Iterator<Item = LineOfReflection> + '_ {
        self.vertical_lines_of_reflection()
            .chain(self.horizontal_lines_of_reflection())
    }
    fn vertical_lines_of_reflection(&self) -> impl Iterator<Item = LineOfReflection> + '_ {
        self.bbox()
            .x_range()
            .filter(|&x| self.is_vlor(x))
            .map(LineOfReflection::Vertical)
    }
    fn horizontal_lines_of_reflection(&self) -> impl Iterator<Item = LineOfReflection> + '_ {
        self.bbox()
            .y_range()
            .filter(|&y| self.is_hlor(y))
            .map(LineOfReflection::Horizontal)
    }

    fn is_hlor(&self, y: i64) -> bool {
        let bbox = self.bbox();
        if y > bbox.y_min() && y <= bbox.y_max() {
            let n = (y - bbox.y_start()).min(bbox.y_end() - y);
            (0..n).all(|i| self.rows_match(y - 1 - i, y + i))
        } else {
            false
        }
    }
    fn is_vlor(&self, x: i64) -> bool {
        let bbox = self.bbox();
        if x > bbox.x_min() && x <= bbox.x_max() {
            let n = (x - bbox.x_start()).min(bbox.x_end() - x);
            (0..n).all(|i| self.cols_match(x - 1 - i, x + i))
        } else {
            false
        }
    }

    fn rows_match(&self, y0: i64, y1: i64) -> bool {
        self.bbox()
            .x_range()
            .all(|x| self.map[p2d(x, y0)] == self.map[p2d(x, y1)])
    }
    fn cols_match(&self, x0: i64, x1: i64) -> bool {
        self.bbox()
            .y_range()
            .all(|y| self.map[p2d(x0, y)] == self.map[p2d(x1, y)])
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Pattern {
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

#[derive(Clone, Debug)]
struct Input {
    patterns: Vec<Pattern>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for pattern in &self.patterns {
            write!(f, "{}{}", sep, pattern)?;
            sep = "\n";
        }
        Ok(())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Ash, char('.')),
        value(Square::Rocks, char('#')),
    ))(i)
}

fn pattern(i: &str) -> IResult<&str, Pattern> {
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Pattern { map }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, patterns) = separated_list1(tuple((line_ending, line_ending)), pattern)(i)?;
    Ok((i, Input { patterns }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input.patterns.iter().flat_map(|p| p.note()).sum::<i64>();

    let result2 = input.patterns.iter().flat_map(|p| p.note2()).sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
