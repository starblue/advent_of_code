use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::one_of;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

use util::Permutation;

#[derive(Clone, Copy, Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}
impl DanceMove {}
impl fmt::Display for DanceMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            DanceMove::Spin(x) => write!(f, "s{}", x,),
            DanceMove::Exchange(x, y) => write!(f, "x{}/{}", x, y),
            DanceMove::Partner(x, y) => write!(f, "p{}/{}", x, y),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    len: usize,
    /// Permutation by name for the `p` dance moves.
    name_perm: Permutation,
    /// Permutation by position for the `s` and `x` dance moves.
    perm: Permutation,
}
impl State {
    fn new(len: usize) -> State {
        State {
            len,
            name_perm: Permutation::identity(len),
            perm: Permutation::identity(len),
        }
    }
    fn execute(&mut self, dance_move: &DanceMove) {
        match dance_move {
            DanceMove::Spin(x) => self.spin(*x),
            DanceMove::Exchange(x, y) => self.exchange(*x, *y),
            DanceMove::Partner(x, y) => self.partner(*x, *y),
        }
    }
    fn spin(&mut self, x: usize) {
        self.perm = Permutation::rotate_right(self.len, x) * &self.perm;
    }
    fn exchange(&mut self, x: usize, y: usize) {
        self.perm = Permutation::transpose(self.len, x, y) * &self.perm;
    }
    fn partner(&mut self, x: char, y: char) {
        let i = usize::try_from(u32::from(x) - u32::from('a')).unwrap();
        let j = usize::try_from(u32::from(y) - u32::from('a')).unwrap();
        self.name_perm = &self.name_perm * Permutation::transpose(self.len, i, j);
    }
    /// Repeat the dance moves so far a given number of times.
    fn repeat(&mut self, n: u32) {
        self.name_perm = self.name_perm.pow(n);
        self.perm = self.perm.pow(n);
    }
    fn result(&self) -> String {
        let programs = ('a'..).take(self.len).collect::<Vec<_>>();
        (&self.perm * &self.name_perm)
            .permute(&programs)
            .into_iter()
            .collect::<String>()
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn letter(i: &str) -> IResult<&str, char> {
    one_of("abcdefghijklmnopqrstuvwxyz")(i)
}

fn dance_move_spin(i: &str) -> IResult<&str, DanceMove> {
    let (i, _) = tag("s")(i)?;
    let (i, x) = uint(i)?;
    Ok((i, DanceMove::Spin(x)))
}
fn dance_move_exchange(i: &str) -> IResult<&str, DanceMove> {
    let (i, _) = tag("x")(i)?;
    let (i, a) = uint(i)?;
    let (i, _) = tag("/")(i)?;
    let (i, b) = uint(i)?;
    Ok((i, DanceMove::Exchange(a, b)))
}
fn dance_move_partner(i: &str) -> IResult<&str, DanceMove> {
    let (i, _) = tag("p")(i)?;
    let (i, a) = letter(i)?;
    let (i, _) = tag("/")(i)?;
    let (i, b) = letter(i)?;
    Ok((i, DanceMove::Partner(a, b)))
}
fn dance_move(i: &str) -> IResult<&str, DanceMove> {
    alt((dance_move_exchange, dance_move_partner, dance_move_spin))(i)
}

fn input(i: &str) -> IResult<&str, Vec<DanceMove>> {
    separated_list1(tag(","), dance_move)(i)
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
    // let mut sep = "";
    // for dance_move in &input {
    //     print!("{}{}", sep, dance_move);
    //     sep = ",";
    // }
    // println!();

    let mut state = State::new(16);
    for dance_move in &input {
        state.execute(dance_move);
    }
    let result_a = state.result();

    state.repeat(1_000_000_000);
    let result_b = state.result();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
