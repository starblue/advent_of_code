use std::io;
use std::io::Read;

use nom::alt;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::many_m_n;
use nom::named;
use nom::tag;
use nom::value;

named!(fb<&str, i64>,
   alt!(
       value!(0, tag!("F")) |
       value!(1, tag!("B"))
   )
);
named!(lr<&str, i64>,
   alt!(
       value!(0, tag!("L")) |
       value!(1, tag!("R"))
   )
);
named!(row<&str, i64>,
    do_parse!(
        fbs: many_m_n!(7, 7, fb) >> (fbs.into_iter().fold(0, |a, b| 2 * a + b))
    )
);
named!(column<&str, i64>,
    do_parse!(
        lrs: many_m_n!(3, 3, lr) >> (lrs.into_iter().fold(0, |a, b| 2 * a + b))
    )
);
named!(seat<&str, i64>,
    do_parse!(
        row: row >>
        column: column >>
        line_ending >> (row * 8 + column)
    )
);
named!(
    input<&str, Vec<i64>>,
    many1!(seat)
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

    let mut seats = result.unwrap().1;
    seats.sort();

    let &max_seat = seats.iter().max().unwrap();

    let result_a = max_seat;

    let mut seat = 0;
    for i in 1..max_seat {
        if seats.contains(&(i - 1)) && !seats.contains(&i) && seats.contains(&(i + 1)) {
            seat = i;
            break;
        }
    }
    let result_b = seat;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
