use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pixel {
    On,
    Off,
}
impl Pixel {
    fn to_char(self) -> char {
        match self {
            Pixel::On => '#',
            Pixel::Off => '.',
        }
    }
}
impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Pattern {
    array: Array2d<i64, Pixel>,
}
impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bounds = self.array.bbox();
        let mut sep = "";
        for y in bounds.y_range() {
            write!(f, "{}", sep)?;
            sep = "/";
            for x in bounds.x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.array[p])?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Rule {
    left: Pattern,
    right: Pattern,
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.left, self.right)
    }
}

fn pixel(i: &str) -> IResult<&str, Pixel> {
    alt((value(Pixel::On, tag("#")), value(Pixel::Off, tag("."))))(i)
}

fn row(i: &str) -> IResult<&str, Vec<Pixel>> {
    many1(pixel)(i)
}

fn pattern(i: &str) -> IResult<&str, Pattern> {
    let (i, rows) = separated_list1(tag("/"), row)(i)?;
    Ok((
        i,
        Pattern {
            array: Array2d::from_vec(rows),
        },
    ))
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (i, left) = pattern(i)?;
    let (i, _) = tag(" => ")(i)?;
    let (i, right) = pattern(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Rule { left, right }))
}

fn input(i: &str) -> IResult<&str, Vec<Rule>> {
    many1(rule)(i)
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

    let mut input = result.unwrap().1;
    // for rule in &input {
    //     println!("{}", rule);
    // }

    let result_a = 0;

    let result_b = 0;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
