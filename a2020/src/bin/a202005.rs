use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::many_m_n;
use nom::IResult;

fn fb(i: &str) -> IResult<&str, i64> {
    alt((value(0, tag("F")), value(1, tag("B"))))(i)
}

fn lr(i: &str) -> IResult<&str, i64> {
    alt((value(0, tag("L")), value(1, tag("R"))))(i)
}

fn row(i: &str) -> IResult<&str, i64> {
    let (i, fbs) = many_m_n(7, 7, fb)(i)?;
    Ok((i, fbs.into_iter().fold(0, |a, b| 2 * a + b)))
}

fn column(i: &str) -> IResult<&str, i64> {
    let (i, lrs) = many_m_n(3, 3, lr)(i)?;
    Ok((i, lrs.into_iter().fold(0, |a, b| 2 * a + b)))
}

fn seat(i: &str) -> IResult<&str, i64> {
    let (i, row) = row(i)?;
    let (i, column) = column(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, row * 8 + column))
}

fn input(i: &str) -> IResult<&str, Vec<i64>> {
    many1(seat)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

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
