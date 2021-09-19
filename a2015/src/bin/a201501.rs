use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::many1;
use nom::named;
use nom::value;

named!(action<&str, i64>,
    alt!(
        value!(1, char!('(')) |
        value!(-1, char!(')'))
    )
);

named!(input<&str, Vec<i64>>,
    many1!(action)
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
    //println!("{:?}", result);

    let input = result.unwrap().1;

    let result_a = input.iter().sum::<i64>();

    let mut floor = 0;
    let mut position = None;
    for (i, delta) in input.iter().enumerate() {
        floor += delta;
        if floor == -1 {
            position = Some(i + 1);
            break;
        }
    }
    let result_b = position.unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
