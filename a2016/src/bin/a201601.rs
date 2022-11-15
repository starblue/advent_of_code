use std::collections::HashSet;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list0;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum Turn {
    L,
    R,
}
impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Turn::L => write!(f, "L"),
            Turn::R => write!(f, "R"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    turn: Turn,
    distance: i64,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.turn, self.distance)
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn turn(i: &str) -> IResult<&str, Turn> {
    alt((value(Turn::L, tag("L")), value(Turn::R, tag("R"))))(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    let (i, turn) = turn(i)?;
    let (i, distance) = uint(i)?;
    Ok((i, Instruction { turn, distance }))
}

fn input(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(tag(", "), instruction)(i)
}

#[derive(Clone, Debug)]
struct State {
    position: Point2d,
    direction: Vec2d,
    been: HashSet<Point2d>,
    hq_position: Option<Point2d>,
}
impl State {
    fn start() -> State {
        let position = p2d(0, 0);
        let direction = v2d(0, 1);
        let mut been = HashSet::new();
        been.insert(position);
        let hq_position = None;
        State {
            position,
            direction,
            been,
            hq_position,
        }
    }
    fn execute(&mut self, i: &Instruction) {
        match i.turn {
            Turn::L => self.direction = self.direction.rotate_left(),
            Turn::R => self.direction = self.direction.rotate_right(),
        }
        for _ in 0..i.distance {
            self.position += self.direction;
            if self.hq_position == None && self.been.contains(&self.position) {
                self.hq_position = Some(self.position);
            }
            self.been.insert(self.position);
        }
    }
    fn shortest_path_len_a(&self) -> i64 {
        self.position.distance_l1(p2d(0, 0))
    }
    fn shortest_path_len_b(&self) -> i64 {
        let hq_position = self.hq_position.unwrap();
        hq_position.distance_l1(p2d(0, 0))
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // let mut sep = "";
    // for i in &input {
    //     print!("{}{}", sep, i);
    //     sep = ", ";
    // }
    // println!();

    let mut state = State::start();
    for i in &input {
        state.execute(i);
    }
    let result_a = state.shortest_path_len_a();

    let mut state = State::start();
    for i in &input {
        state.execute(i);
    }
    let result_b = state.shortest_path_len_b();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
