use core::fmt;

use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::many_m_n;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Filled,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Filled => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char('.')),
        value(Square::Filled, char('#')),
    ))(i)
}

#[derive(Clone, Debug)]
struct Schematic {
    map: Array2d<i64, Square>,
}
impl Schematic {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Schematic {
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

fn schematic(i: &str) -> IResult<&str, Schematic> {
    let (i, lines) = separated_list1(line_ending, many1(square))(i)?;
    let map = Array2d::from_vec(lines);
    Ok((i, Schematic { map }))
}

#[derive(Clone, Debug)]
struct Input {
    schematics: Vec<Schematic>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for schematic in &self.schematics {
            writeln!(f, "{}", schematic)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, schematics) = separated_list1(many_m_n(2, 2, line_ending), schematic)(i)?;
    Ok((i, Input { schematics }))
}

#[derive(Clone, Debug)]
struct Lock {
    heights: Vec<i64>,
}
impl TryFrom<&Schematic> for Lock {
    type Error = Error;
    fn try_from(schematic: &Schematic) -> Result<Self> {
        let mut heights = Vec::new();
        for x in schematic.bbox().x_range() {
            let mut height = None;
            let mut i = -1;
            for y in schematic.bbox().y_range() {
                let p = p2d(x, y);
                if height.is_none() {
                    if schematic.map[p] == Square::Empty {
                        // First empty square, we found the height.
                        height = Some(i);
                    }
                } else {
                    if schematic.map[p] == Square::Filled {
                        // Filled after empty is not allowed.
                        return Err("not a lock schematic".into());
                    }
                }
                i += 1;
            }
            if let Some(h) = height {
                if h >= 0 {
                    heights.push(h);
                } else {
                    return Err("not a lock schematic: top row empty".into());
                }
            } else {
                return Err("not a lock schematic: bottom row filled".into());
            }
        }
        Ok(Lock { heights })
    }
}

#[derive(Clone, Debug)]
struct Key {
    heights: Vec<i64>,
}
impl Key {
    fn fits(&self, lock: &Lock) -> bool {
        assert_eq!(self.heights.len(), lock.heights.len());
        self.heights
            .iter()
            .zip(lock.heights.iter())
            .all(|(hk, hl)| hk + hl <= 5)
    }
}
impl TryFrom<&Schematic> for Key {
    type Error = Error;
    fn try_from(schematic: &Schematic) -> Result<Self> {
        let mut heights = Vec::new();
        for x in schematic.bbox().x_range() {
            let mut height = None;
            let mut i = -1;
            for y in schematic.bbox().y_range().rev() {
                let p = p2d(x, y);
                if height.is_none() {
                    if schematic.map[p] == Square::Empty {
                        // First empty square, we found the height.
                        height = Some(i);
                    }
                } else {
                    if schematic.map[p] == Square::Filled {
                        // Filled after empty is not allowed.
                        return Err("not a key schematic".into());
                    }
                }
                i += 1;
            }
            if let Some(h) = height {
                if h >= 0 {
                    heights.push(h);
                } else {
                    return Err("not a key schematic: bottom row empty".into());
                }
            } else {
                return Err("not a key schematic: top row filled".into());
            }
        }
        Ok(Key { heights })
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let locks = input
        .schematics
        .iter()
        .filter_map(|s| Lock::try_from(s).ok())
        .collect::<Vec<_>>();
    let keys = input
        .schematics
        .iter()
        .filter_map(|s| Key::try_from(s).ok())
        .collect::<Vec<_>>();
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if key.fits(lock) {
                count += 1;
            }
        }
    }
    let result1 = count;

    println!("Part 1: {}", result1);

    Ok(())
}
