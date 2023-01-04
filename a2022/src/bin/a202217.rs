use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::multi::many0;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum Jet {
    Left,
    Right,
}
impl Jet {
    fn push(&self) -> Vec2d {
        match self {
            Jet::Left => v2d(-1, 0),
            Jet::Right => v2d(1, 0),
        }
    }
    fn to_char(self) -> char {
        match self {
            Jet::Left => '<',
            Jet::Right => '>',
        }
    }
}
impl fmt::Display for Jet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Shape {
    /// The positions occupied by rock relative to the lower left corner.
    rock: HashSet<Vec2d>,
}
impl Shape {
    fn shapes() -> Vec<Shape> {
        [
            vec![v2d(0, 0), v2d(1, 0), v2d(2, 0), v2d(3, 0)],
            vec![v2d(1, 0), v2d(0, 1), v2d(1, 1), v2d(2, 1), v2d(1, 2)],
            vec![v2d(0, 0), v2d(1, 0), v2d(2, 0), v2d(2, 1), v2d(2, 2)],
            vec![v2d(0, 0), v2d(0, 1), v2d(0, 2), v2d(0, 3)],
            vec![v2d(0, 0), v2d(0, 1), v2d(1, 0), v2d(1, 1)],
        ]
        .into_iter()
        .map(|a| Shape {
            rock: a.into_iter().collect::<HashSet<_>>(),
        })
        .collect::<Vec<_>>()
    }
}

fn jet(i: &str) -> IResult<&str, Jet> {
    alt((value(Jet::Left, char('<')), value(Jet::Right, char('>'))))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Jet>> {
    many0(jet)(i)
}

const TOP_LEN: usize = 64;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    jet_index: usize,
    shape_index: usize,
    chamber_top: [u8; TOP_LEN],
}

#[derive(Clone, Debug)]
struct Chamber {
    rocks: Vec<u8>,
}
impl Chamber {
    fn new() -> Chamber {
        Chamber { rocks: Vec::new() }
    }
    fn allows_at(&self, shape: &Shape, p: Point2d) -> bool {
        shape.rock.iter().all(|&v| self.empty_at(p + v))
    }
    fn empty_at(&self, p: Point2d) -> bool {
        if (0..7).contains(&p.x()) && p.y() >= 0 {
            let y = usize::try_from(p.y()).unwrap();
            y >= self.rocks.len() || self.rocks[y] & (1 << p.x()) == 0
        } else {
            false
        }
    }
    fn rock_height(&self) -> i64 {
        i64::try_from(self.rocks.len()).unwrap()
    }
    fn add_rock(&mut self, shape: &Shape, p: Point2d) {
        for v in &shape.rock {
            let p1 = p + v;
            let y1 = usize::try_from(p1.y()).unwrap();
            let len = self.rocks.len().max(y1 + 1);
            self.rocks.resize(len, 0);
            self.rocks[y1] |= 1 << p1.x();
        }
    }
    fn top(&self) -> [u8; TOP_LEN] {
        let len = self.rocks.len();
        <[u8; TOP_LEN]>::try_from(&self.rocks[(len - TOP_LEN)..len]).unwrap()
    }
}
impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut y = self.rock_height() + 3;
        while y >= 0 {
            write!(f, "|")?;
            for x in 0..7 {
                let p = p2d(x, y);
                write!(f, "{}", if self.empty_at(p) { '.' } else { '#' })?;
            }
            writeln!(f, "|")?;

            y -= 1;
        }
        writeln!(f, "+-------+")
    }
}

fn height(jets: &[Jet], n: i64) -> i64 {
    let mut jet_index = 0;
    let shapes = Shape::shapes();
    let mut shape_index = 0;

    let mut state_heights = HashMap::new();
    let mut iterations_height = 0;

    let mut chamber = Chamber::new();
    let mut count = 0;
    while count < n {
        let shape = &shapes[shape_index];
        shape_index = (shape_index + 1) % shapes.len();
        let mut p = p2d(2, chamber.rock_height() + 3);
        loop {
            // Sideways movement
            let jet = &jets[jet_index];
            jet_index = (jet_index + 1) % jets.len();
            let new_p = p + jet.push();
            if chamber.allows_at(shape, new_p) {
                p = new_p;
            }

            // Downward movement
            let new_p = p - v2d(0, 1);
            if chamber.allows_at(shape, new_p) {
                p = new_p;
            } else {
                // The rock stopped falling
                chamber.add_rock(shape, p);
                break;
            }
        }

        count += 1;

        if usize::try_from(chamber.rock_height()).unwrap() >= TOP_LEN
            && iterations_height == 0
        {
            // Look for a repeated state.

            let chamber_top = chamber.top();
            let state = State {
                jet_index,
                shape_index,
                chamber_top,
            };
            if let Some((count0, height0)) = state_heights.get(&state) {
                // We found a repetition, jump forward to near the end.
                let height = chamber.rock_height();

                // Count and height per iteration:
                let d_count = count - count0;
                let d_height = height - height0;

                // The number of remaining full iterations from now on:
                let iterations = (n - count) / d_count;

                // Jump over the remaining full iterations to near the end.
                count += iterations * d_count;
                iterations_height = iterations * d_height;
            } else {
                // New state, insert it.
                state_heights.insert(state, (count, chamber.rock_height()));
            }
        }
    }
    chamber.rock_height() + iterations_height
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for jet in &input {
    //     print!("{}", jet);
    // }
    // println!();

    let result1 = height(&input, 2022);
    let result2 = height(&input, 1_000_000_000_000);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
