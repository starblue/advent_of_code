use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::BBox2d;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}
impl Node {
    fn to_char(self) -> char {
        match self {
            Node::Clean => '.',
            Node::Weakened => 'W',
            Node::Infected => '#',
            Node::Flagged => 'F',
        }
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn node(i: &str) -> IResult<&str, Node> {
    alt((
        value(Node::Clean, char('.')),
        value(Node::Infected, char('#')),
        value(Node::Weakened, char('W')),
        value(Node::Flagged, char('F')),
    ))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Node>> {
    let (i, line) = many1(node)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn lines(i: &str) -> IResult<&str, Vec<Vec<Node>>> {
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

    let row_count = input.len();
    let mut bounds = BBox2d::from_point(p2d(0, 0));
    let mut initial_map = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, &node) in row.iter().enumerate() {
            let x = i64::try_from(j).unwrap();
            let y = i64::try_from(row_count - i - 1).unwrap();
            let p = p2d(x, y);
            bounds = bounds.extend_to(p);
            initial_map.insert(p, node);
        }
    }

    let mut map = initial_map.clone();
    let mut pos = bounds.center();
    let mut dir = v2d(0, 1);
    let mut infection_count = 0;
    for _ in 0..10_000 {
        let entry = map.entry(pos).or_insert(Node::Clean);
        match *entry {
            Node::Infected => {
                dir = dir.rotate_right();
                *entry = Node::Clean;
            }
            Node::Clean => {
                dir = dir.rotate_left();
                *entry = Node::Infected;
                infection_count += 1;
            }
            _ => {
                panic!("unexpected node value");
            }
        }
        pos += dir;
    }
    let result_a = infection_count;

    let mut map = initial_map;
    let mut pos = bounds.center();
    let mut dir = v2d(0, 1);
    let mut infection_count = 0;
    for _ in 0..10_000_000 {
        let entry = map.entry(pos).or_insert(Node::Clean);
        match *entry {
            Node::Clean => {
                dir = dir.rotate_left();
                *entry = Node::Weakened;
            }
            Node::Weakened => {
                // Don't change direction.
                *entry = Node::Infected;
                infection_count += 1;
            }
            Node::Infected => {
                dir = dir.rotate_right();
                *entry = Node::Flagged;
            }
            Node::Flagged => {
                dir = -dir;
                *entry = Node::Clean;
            }
        }
        pos += dir;
    }
    let result_b = infection_count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
