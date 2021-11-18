use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Empty,
    PathNS,
    PathEW,
    Curve,
    Letter(char),
}
impl Square {
    fn is_empty(&self) -> bool {
        *self == Square::Empty
    }
    fn to_char(self) -> char {
        match self {
            Square::Empty => ' ',
            Square::PathNS => '|',
            Square::PathEW => '-',
            Square::Curve => '+',
            Square::Letter(c) => c,
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn square_letter(i: &str) -> IResult<&str, Square> {
    let (i, c) = one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)?;
    Ok((i, Square::Letter(c)))
}
fn square(i: &str) -> IResult<&str, Square> {
    alt((
        value(Square::Empty, char(' ')),
        value(Square::PathNS, char('|')),
        value(Square::PathEW, char('-')),
        value(Square::Curve, char('+')),
        square_letter,
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

    let input = result.unwrap().1;

    let map: Array2d<i64, Square> = Array2d::from_vec(input);
    // for y in map.bounds().y_range() {
    //     for x in map.bounds().x_range() {
    //         print!("{}", map[p2d(x, y)]);
    //     }
    //     println!();
    // }

    let bounds = map.bounds();
    // Find the starting position in the first row.
    let mut pos = bounds
        .x_range()
        .map(|x| p2d(x, 0))
        .find(|&p| !map[p].is_empty())
        .unwrap();
    let mut dir = v2d(1, 0);
    let mut letters = String::new();
    let mut steps = 0;
    loop {
        if let Square::Letter(c) = map[pos] {
            letters.push(c);
        }
        steps += 1;

        let left = dir.rotate_left();
        let right = dir.rotate_right();
        let np = pos + dir;
        let lnp = pos + left;
        let rnp = pos + right;
        if bounds.contains(&np) && !map[np].is_empty() {
            pos = np;
        } else if bounds.contains(&lnp) && !map[lnp].is_empty() {
            pos = lnp;
            dir = left;
        } else if bounds.contains(&rnp) && !map[rnp].is_empty() {
            pos = rnp;
            dir = right;
        } else {
            // We reached the end.
            break;
        }
    }
    let result_a = letters;
    let result_b = steps;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
