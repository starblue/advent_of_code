use std::fmt;
use std::io;
use std::io::Read;
use std::ops;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}
impl Tile {
    fn is_safe(&self) -> bool {
        self == &Tile::Safe
    }
    fn to_char(self) -> char {
        match self {
            Tile::Safe => '.',
            Tile::Trap => '^',
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Row(Vec<Tile>);
impl Row {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
    fn safe_count(&self) -> usize {
        self.0.iter().filter(|tile| tile.is_safe()).count()
    }
}
impl ops::Index<i64> for Row {
    type Output = Tile;

    fn index(&self, index: i64) -> &Self::Output {
        if (0..self.len()).contains(&index) {
            &self.0[index as usize]
        } else {
            &Tile::Safe
        }
    }
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for tile in &self.0 {
            write!(f, "{}", tile)?;
        }
        writeln!(f)
    }
}

fn tile(i: &str) -> IResult<&str, Tile> {
    alt((value(Tile::Safe, char('.')), value(Tile::Trap, char('^'))))(i)
}

fn input(i: &str) -> IResult<&str, Row> {
    let (i, line) = many1(tile)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Row(line)))
}

fn floor_safe_count(input: &Row, row_count: usize) -> usize {
    let mut row = input.clone();
    let mut count = 0;
    for _ in 0..row_count {
        count += row.safe_count();

        let mut new_tiles = Vec::new();
        for i in 0..row.len() {
            let left = row[i - 1];
            let right = row[i + 1];
            let new_tile = match (left, right) {
                (Tile::Trap, Tile::Safe) => Tile::Trap,
                (Tile::Safe, Tile::Trap) => Tile::Trap,
                _ => Tile::Safe,
            };
            new_tiles.push(new_tile);
        }
        row = Row(new_tiles);
    }
    count
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
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let result_a = floor_safe_count(&input, 40);
    let result_b = floor_safe_count(&input, 400_000);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
