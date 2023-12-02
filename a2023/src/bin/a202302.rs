use core::fmt;
use core::str::FromStr;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}
impl Color {
    fn values() -> impl Iterator<Item = Color> {
        [Color::Red, Color::Green, Color::Blue].into_iter()
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
        }
    }
}

#[derive(Clone, Debug)]
struct Reveal(Vec<(Color, usize)>);
impl Reveal {
    fn possible(&self, max_reveal: &Reveal) -> bool {
        Color::values().all(|c| self.count(c) <= max_reveal.count(c))
    }
    fn count(&self, color: Color) -> usize {
        self.0
            .iter()
            .filter(|(c, _n)| *c == color)
            .map(|(_c, n)| n)
            .sum::<usize>()
    }
}
impl fmt::Display for Reveal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut sep = "";
        for (c, n) in &self.0 {
            write!(f, "{sep}{n} {c}")?;
            sep = ", ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    moves: Vec<Reveal>,
}
impl Game {
    fn possible(&self, max_reveal: &Reveal) -> bool {
        self.moves.iter().all(|r| r.possible(max_reveal))
    }
    fn needed(&self, color: Color) -> usize {
        self.moves.iter().map(|r| r.count(color)).max().unwrap_or(0)
    }
    fn power(&self) -> usize {
        Color::values().map(|c| self.needed(c)).product::<usize>()
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Game {}: ", self.id)?;
        let mut sep = "";
        for r in &self.moves {
            write!(f, "{sep}{r}")?;
            sep = "; ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    games: Vec<Game>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for g in &self.games {
            writeln!(f, "{}", g)?;
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn color(i: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Red, tag("red")),
        value(Color::Green, tag("green")),
        value(Color::Blue, tag("blue")),
    ))(i)
}

fn color_count(i: &str) -> IResult<&str, (Color, usize)> {
    let (i, n) = uint(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, c) = color(i)?;
    Ok((i, (c, n)))
}

fn reveal(i: &str) -> IResult<&str, Reveal> {
    let (i, ccs) = separated_list1(tag(", "), color_count)(i)?;
    Ok((i, Reveal(ccs)))
}

fn game(i: &str) -> IResult<&str, Game> {
    let (i, _) = tag("Game ")(i)?;
    let (i, id) = uint(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, moves) = separated_list1(tag("; "), reveal)(i)?;
    Ok((i, Game { id, moves }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, games) = separated_list1(line_ending, game)(i)?;
    Ok((i, Input { games }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let max_reveal = Reveal(vec![
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]);
    let result1 = input
        .games
        .iter()
        .filter(|g| g.possible(&max_reveal))
        .map(|g| g.id)
        .sum::<usize>();

    let result2 = input.games.iter().map(|g| g.power()).sum::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
