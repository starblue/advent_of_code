use core::fmt;

use std::error;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}
impl Outcome {
    fn score(&self) -> i64 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
    fn to_char(self) -> char {
        match self {
            Outcome::Loss => 'X',
            Outcome::Draw => 'Y',
            Outcome::Win => 'Z',
        }
    }
}
impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    fn score(&self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
    fn outcome(&self, other: Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Loss,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Loss,
            (Move::Scissors, Move::Rock) => Outcome::Loss,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }
    fn round_score(&self, other: Move) -> i64 {
        self.score() + self.outcome(other).score()
    }
    fn to_char(self) -> char {
        match self {
            Move::Rock => 'A',
            Move::Paper => 'B',
            Move::Scissors => 'C',
        }
    }
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
struct Round {
    elf_move: Move,
    outcome: Outcome,
}
impl Round {
    fn score1(&self) -> i64 {
        let player_move = match self.outcome {
            Outcome::Loss => Move::Rock,
            Outcome::Draw => Move::Paper,
            Outcome::Win => Move::Scissors,
        };
        player_move.round_score(self.elf_move)
    }
    fn score2(&self) -> i64 {
        let player_move = match (self.elf_move, self.outcome) {
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
        };
        player_move.round_score(self.elf_move)
    }
}
impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", &self.elf_move, self.outcome)
    }
}

fn elf_move(i: &str) -> IResult<&str, Move> {
    alt((
        value(Move::Rock, char('A')),
        value(Move::Paper, char('B')),
        value(Move::Scissors, char('C')),
    ))(i)
}

fn outcome(i: &str) -> IResult<&str, Outcome> {
    alt((
        value(Outcome::Loss, char('X')),
        value(Outcome::Draw, char('Y')),
        value(Outcome::Win, char('Z')),
    ))(i)
}

fn round(i: &str) -> IResult<&str, Round> {
    let (i, elf_move) = elf_move(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, outcome) = outcome(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Round { elf_move, outcome }))
}

fn input(i: &str) -> IResult<&str, Vec<Round>> {
    many1(round)(i)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for round in &input {
    //     println!("{}", round);
    // }

    let result1 = input.iter().map(|r| r.score1()).sum::<i64>();

    let result2 = input.iter().map(|r| r.score2()).sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
