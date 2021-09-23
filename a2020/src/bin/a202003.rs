use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Tree,
}
impl Square {
    fn to_char(&self) -> char {
        match self {
            Square::Empty => '.',
            Square::Tree => '#',
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
        value(Square::Tree, char('#')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    let (i, line) = many1(square)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn lines(i: &str) -> IResult<&str, Vec<Vec<Square>>> {
    many1(line)(i)
}

fn count_trees(map: &[Vec<Square>], (dx, dy): (usize, usize)) -> usize {
    let len_y = map.len();
    let len_x = map[0].len();

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < len_y {
        if map[y][x] == Square::Tree {
            count += 1;
        }

        x = (x + dx) % len_x;
        y += dy;
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
    let result = lines(&input_data);
    //println!("{:?}", result);

    let map = result.unwrap().1;

    let result_a = count_trees(&map, (3, 1));
    let result_b = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|d| count_trees(&map, d))
        .product::<usize>();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
