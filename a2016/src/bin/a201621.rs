use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnPositionOfLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}
impl Operation {}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Operation::SwapPosition(x, y) => write!(f, "swap position {} with position {}", x, y),
            Operation::SwapLetter(x, y) => write!(f, "swap letter {} with letter {}", x, y),
            Operation::RotateLeft(x) => {
                write!(
                    f,
                    "rotate left {} step{}",
                    x,
                    if *x != 1 { "s" } else { "" }
                )
            }
            Operation::RotateRight(x) => {
                write!(
                    f,
                    "rotate right {} step{}",
                    x,
                    if *x != 1 { "s" } else { "" }
                )
            }
            Operation::RotateBasedOnPositionOfLetter(x) => {
                write!(f, "rotate based on position of letter {}", x)
            }
            Operation::Reverse(x, y) => write!(f, "reverse positions {} through {}", x, y),
            Operation::Move(x, y) => write!(f, "move position {} to position {}", x, y),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    password: Vec<char>,
}
impl State {
    fn new(input: &str) -> State {
        let password = input.chars().collect::<Vec<_>>();
        State { password }
    }
    fn execute(&mut self, operation: &Operation) {
        match operation {
            Operation::SwapPosition(x, y) => self.swap_position(*x, *y),
            Operation::SwapLetter(x, y) => self.swap_letter(*x, *y),
            Operation::RotateLeft(x) => self.rotate_left(*x),
            Operation::RotateRight(x) => self.rotate_right(*x),
            Operation::RotateBasedOnPositionOfLetter(x) => {
                self.rotate_based_on_position_of_letter(*x)
            }
            Operation::Reverse(x, y) => self.reverse(*x, *y),
            Operation::Move(x, y) => self.move_(*x, *y),
        }
    }
    fn reverse_execute(&mut self, operation: &Operation) {
        match operation {
            Operation::SwapPosition(x, y) => self.swap_position(*x, *y),
            Operation::SwapLetter(x, y) => self.swap_letter(*x, *y),
            Operation::RotateLeft(x) => self.rotate_right(*x),
            Operation::RotateRight(x) => self.rotate_left(*x),
            Operation::RotateBasedOnPositionOfLetter(x) => {
                self.reverse_rotate_based_on_position_of_letter(*x)
            }
            Operation::Reverse(x, y) => self.reverse(*x, *y),
            Operation::Move(x, y) => self.move_(*y, *x),
        }
    }
    fn swap_position(&mut self, x: usize, y: usize) {
        self.password.swap(x, y);
    }
    fn swap_letter(&mut self, x: char, y: char) {
        if let (Some(i), Some(j)) = (
            self.password.iter().position(|&c| c == x),
            self.password.iter().position(|&c| c == y),
        ) {
            self.password.swap(i, j);
        }
    }
    fn rotate_left(&mut self, x: usize) {
        let x = x % self.password.len();
        let mut new_password = self.password.split_off(x);
        new_password.append(&mut self.password);
        self.password = new_password;
    }
    fn rotate_right(&mut self, x: usize) {
        let len = self.password.len();
        self.rotate_left(len - x % len);
    }
    fn rotate_based_on_position_of_letter(&mut self, x: char) {
        if let Some(i) = self.password.iter().position(|&c| c == x) {
            let j = i + 1 + if i >= 4 { 1 } else { 0 };
            self.rotate_right(j);
        }
    }
    fn reverse_rotate_based_on_position_of_letter(&mut self, x: char) {
        if let Some(i) = self.password.iter().position(|&c| c == x) {
            // Derive the rotation from the position of x after the operation.
            // This is only valid for passwords of length 8!
            assert_eq!(8, self.password.len());

            // pos x before op  0 1 2 3 4 5 6 7
            // right rotation   1 2 3 4 6 7 8 9
            // pos x after op   1 3 5 7 2 4 6 0
            let j = if i % 2 != 0 {
                (i + 1) / 2
            } else if i == 0 {
                1
            } else {
                (i / 2) + 5
            };

            // Reverse the rotation.
            self.rotate_left(j);
        }
    }
    fn reverse(&mut self, x: usize, y: usize) {
        self.permute(|i| if x <= i && i <= y { x + y - i } else { i });
    }
    fn move_(&mut self, x: usize, y: usize) {
        self.permute(|i| {
            match x.cmp(&y) {
                Ordering::Less => {
                    if i < x {
                        i
                    } else if i < y {
                        // shift down elements, omitting element at x
                        i + 1
                    } else if i == y {
                        x
                    } else {
                        i
                    }
                }
                Ordering::Greater => {
                    if i < y {
                        i
                    } else if i == y {
                        x
                    } else if i <= x {
                        // shift up elements, omitting element at x
                        i - 1
                    } else {
                        i
                    }
                }
                Ordering::Equal => {
                    // x == y, delete and insert in same position means no change
                    i
                }
            }
        });
    }
    fn permute<F>(&mut self, f: F)
    where
        F: Fn(usize) -> usize,
    {
        let new_password = (0..self.password.len())
            .map(|i| self.password[f(i)])
            .collect::<Vec<_>>();
        self.password = new_password;
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for c in &self.password {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn letter(i: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(i)
}

fn operation_swap_position(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("swap position ")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag(" with position ")(i)?;
    let (i, y) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::SwapPosition(x, y)))
}
fn operation_swap_letter(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("swap letter ")(i)?;
    let (i, x) = letter(i)?;
    let (i, _) = tag(" with letter ")(i)?;
    let (i, y) = letter(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::SwapLetter(x, y)))
}
fn operation_rotate_left(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("rotate left ")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag(" step")(i)?;
    let (i, _) = opt(tag("s"))(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::RotateLeft(x)))
}
fn operation_rotate_right(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("rotate right ")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag(" step")(i)?;
    let (i, _) = opt(tag("s"))(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::RotateRight(x)))
}
fn operation_rotate_based_on_position_of_letter(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("rotate based on position of letter ")(i)?;
    let (i, x) = letter(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::RotateBasedOnPositionOfLetter(x)))
}
fn operation_reverse(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("reverse positions ")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag(" through ")(i)?;
    let (i, y) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::Reverse(x, y)))
}
fn operation_move(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("move position ")(i)?;
    let (i, x) = uint(i)?;
    let (i, _) = tag(" to position ")(i)?;
    let (i, y) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Operation::Move(x, y)))
}
fn operation(i: &str) -> IResult<&str, Operation> {
    alt((
        operation_swap_position,
        operation_swap_letter,
        operation_rotate_left,
        operation_rotate_right,
        operation_rotate_based_on_position_of_letter,
        operation_reverse,
        operation_move,
    ))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Operation>> {
    many1(operation)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for operation in &input {
    //     println!("{}", operation);
    // }

    let mut state = State::new("abcdefgh");
    for operation in &input {
        state.execute(operation);
    }
    let result_a = state;

    let mut state = State::new("fbgdceah");
    for operation in input.iter().rev() {
        state.reverse_execute(operation);
    }
    let result_b = state;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
