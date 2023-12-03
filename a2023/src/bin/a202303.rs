use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Digit(u32),
    Symbol(char),
}
impl Square {
    fn to_digit(self) -> Option<u32> {
        if let Square::Digit(d) = self {
            Some(d)
        } else {
            None
        }
    }
    fn is_symbol(&self) -> bool {
        matches!(self, &Square::Symbol(_))
    }
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Digit(d) => char::from_digit(d, 10).unwrap_or('�'),
            Square::Symbol(c) => c,
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Schematic {
    map: Array2d<i64, Square>,
}
impl Schematic {
    fn new(map: Array2d<i64, Square>) -> Schematic {
        Schematic { map }
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn number_bboxes(&self) -> Vec<(u32, BBox2d)> {
        let mut result = Vec::new();
        for y in self.bbox().y_range() {
            let mut n = 0;
            let mut bbox: Option<BBox2d> = None;
            for x in self.bbox().x_range() {
                let p = p2d(x, y);
                if let Some(d) = self.map[p].to_digit() {
                    n = 10 * n + d;
                    if let Some(ref mut b) = bbox {
                        b.extend_to(p);
                    } else {
                        bbox = Some(p.into())
                    }
                } else {
                    // If we were reading a number it ends here.
                    if let Some(b) = bbox {
                        result.push((n, b));
                    }
                    n = 0;
                    bbox = None;
                }
            }
            // In case there was a number at the end of the line.
            if let Some(b) = bbox {
                result.push((n, b));
            }
        }
        result
    }
}
impl fmt::Display for Schematic {
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

fn square_empty(i: &str) -> IResult<&str, Square> {
    value(Square::Empty, char('.'))(i)
}
fn square_digit(i: &str) -> IResult<&str, Square> {
    let (i, c) = one_of("0123456789")(i)?;
    Ok((i, Square::Digit(c.to_digit(10).unwrap())))
}
fn square_symbol(i: &str) -> IResult<&str, Square> {
    let (i, c) = none_of(".0123456789\n")(i)?;
    Ok((i, Square::Symbol(c)))
}
fn square(i: &str) -> IResult<&str, Square> {
    alt((square_empty, square_digit, square_symbol))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn schematic(i: &str) -> IResult<&str, Schematic> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((i, Schematic::new(Array2d::from_vec(rows))))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = schematic(&input_data).map_err(|e| e.to_owned())?;

    let schematic = result.1;
    // println!("{}", schematic);

    let map = &schematic.map;
    let bbox = map.bbox();

    let mut sum = 0;
    for y in bbox.y_range() {
        let mut n = 0;
        let mut has_symbol = false;
        for x in bbox.x_range() {
            let p = p2d(x, y);
            if let Some(d) = map[p].to_digit() {
                n = 10 * n + d;
                has_symbol |= p.neighbors_l_infty().any(|np| {
                    if let Some(square) = map.get(np) {
                        square.is_symbol()
                    } else {
                        false
                    }
                });
            } else {
                // If we were reading a number it ends here.
                if has_symbol {
                    sum += n;
                }
                n = 0;
                has_symbol = false;
            }
        }
        // In case there was a number at the end of the line.
        if has_symbol {
            sum += n;
        }
    }
    let result1 = sum;

    let number_bboxes = schematic.number_bboxes();
    let mut nbb_map = HashMap::new();
    for nbb in &number_bboxes {
        for p in nbb.1 {
            nbb_map.insert(p, nbb);
        }
    }

    let mut sum = 0;
    for p in &bbox {
        if map[p] == Square::Symbol('*') {
            let mut nbbs = HashSet::new();
            for np in p.neighbors_l_infty() {
                if let Some(nbb) = nbb_map.get(&np) {
                    nbbs.insert(nbb);
                }
            }
            if nbbs.len() == 2 {
                // We found a gear.
                sum += nbbs.iter().map(|(n, _bb)| n).product::<u32>();
            }
        }
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
