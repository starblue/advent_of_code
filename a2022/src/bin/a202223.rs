use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
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
use lowdim::BBox;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

use util::runtime_error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Elf,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => '.',
            Square::Elf => '#',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Square>,
}
impl Input {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Input {
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

fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char('.')),
        value(Square::Elf, char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    many1(square)(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((
        i,
        Input {
            map: Array2d::from_vec(rows),
        },
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Elf {
    pos: Point2d,
}
impl Elf {
    fn is_alone(&self, elves: &HashSet<Elf>) -> bool {
        self.pos
            .neighbors_l_infty()
            .all(|p| !elves.contains(&Elf { pos: p }))
    }
    fn square_is_empty(&self, v: Vec2d, elves: &HashSet<Elf>) -> bool {
        let elf = Elf { pos: self.pos + v };
        !elves.contains(&elf)
    }
    fn can_move(&self, dir: Vec2d, elves: &HashSet<Elf>) -> bool {
        let forward = dir;
        let forward_left = forward + dir.rotate_right();
        let forward_right = forward + dir.rotate_left();

        self.square_is_empty(forward, elves)
            && self.square_is_empty(forward_left, elves)
            && self.square_is_empty(forward_right, elves)
    }
    fn tried_move(&self, dirs: &[Vec2d], elves: &HashSet<Elf>) -> Point2d {
        if self.is_alone(elves) {
            // We don't need to move.
            self.pos
        } else {
            // Try to move.
            for &dir in dirs {
                if self.can_move(dir, elves) {
                    return self.pos + dir;
                }
            }

            // We can't move, so we stay in the same position.
            self.pos
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    elves: HashSet<Elf>,
    dirs: Vec<Vec2d>,
    round: usize,
}
impl State {
    fn new(map: &Array2d<i64, Square>) -> State {
        let bbox = map.bbox();
        let elves = bbox
            .iter()
            .flat_map(|p| {
                if map[p] == Square::Elf {
                    Some(Elf { pos: p })
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        let dirs = vec![v2d(0, -1), v2d(0, 1), v2d(-1, 0), v2d(1, 0)];
        let round = 0;
        State { elves, dirs, round }
    }
    fn step(&mut self) {
        let tried_moves = self
            .elves
            .iter()
            .map(|e| (e, e.tried_move(&self.dirs, &self.elves)))
            .collect::<HashMap<_, _>>();

        // Count elves which want to move into each position,
        // to determine collisions.
        let mut counts = HashMap::new();
        for (_, p) in &tried_moves {
            let entry = counts.entry(p).or_insert(0);
            *entry += 1;
        }

        // Execute the moves that don't collide.
        let mut new_elves = HashSet::new();
        for elf in self.elves.iter() {
            let p = tried_moves[elf];
            let new_elf = Elf {
                pos: {
                    if counts[&p] == 1 {
                        // Move to the new position.
                        p
                    } else {
                        // Stay at the old position.
                        elf.pos
                    }
                },
            };
            new_elves.insert(new_elf);
        }
        self.elves = new_elves;

        // Move the first direction to the end.
        let dir = self.dirs.remove(0);
        self.dirs.push(dir);

        self.round += 1;
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut state = State::new(&input.map);
    for _ in 0..10 {
        state.step();
    }
    let bbox = BBox::enclosing(state.elves.iter().map(|e| &e.pos))
        .ok_or(runtime_error!("no elves"))?;
    let result1 = bbox.volume() - state.elves.len();

    let mut previous_elves = HashSet::new();
    while state.elves != previous_elves {
        previous_elves = state.elves.clone();
        state.step();
    }
    let result2 = state.round;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
