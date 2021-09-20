use std::io;
use std::io::Read;
use std::str::FromStr;

use lowdim::Array2d;
use lowdim::BBox2d;
use nom::alt;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map_res;
use nom::named;
use nom::tag;
use nom::value;

use lowdim::p2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
impl Action {
    fn apply_1(&self, b: &mut bool) {
        match self {
            Action::TurnOn => {
                *b = true;
            }
            Action::TurnOff => {
                *b = false;
            }
            Action::Toggle => {
                *b = !*b;
            }
        }
    }
    fn apply_2(&self, b: &mut i64) {
        match self {
            Action::TurnOn => {
                *b += 1;
            }
            Action::TurnOff => {
                *b = (*b - 1).max(0);
            }
            Action::Toggle => {
                *b += 2;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    action: Action,
    bbox: BBox2d,
}

named!(uint<&str, i64>,
    map_res!(digit1, FromStr::from_str)
);

named!(action<&str, Action>,
    alt!(
        value!(Action::TurnOn, tag!("turn on")) |
        value!(Action::TurnOff, tag!("turn off")) |
        value!(Action::Toggle, tag!("toggle"))
    )
);

named!(point<&str, Point2d>,
    do_parse!(
        x: uint >>
        tag!(",") >>
        y: uint >> (p2d(x, y))
    )
);

named!(bbox<&str, BBox2d>,
    do_parse!(
        p0: point >>
        tag!(" through ") >>
        p1: point >> (BBox2d::from_points(p0, p1))
    )
);

named!(instruction<&str, Instruction>,
    do_parse!(
        action: action >>
        tag!(" ") >>
        bbox: bbox >>
        line_ending >> (Instruction { action, bbox })
    )
);

named!(input<&str, Vec<Instruction>>,
    many1!(instruction)
);

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

    let bbox = BBox2d::from_points(p2d(0, 0), p2d(999, 999));
    let mut lights = Array2d::new(bbox, false);
    for &instruction in &input {
        for p in instruction.bbox.iter() {
            instruction.action.apply_1(&mut lights[p]);
        }
    }
    let result_a = lights.bounds().iter().filter(|&p| lights[p]).count();

    let bbox = BBox2d::from_points(p2d(0, 0), p2d(999, 999));
    let mut lights = Array2d::new(bbox, 0);
    for &instruction in &input {
        for p in instruction.bbox.iter() {
            instruction.action.apply_2(&mut lights[p]);
        }
    }
    let result_b = lights.bounds().iter().map(|p| lights[p]).sum::<i64>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
