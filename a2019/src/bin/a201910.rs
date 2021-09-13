use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::named;
use nom::value;

use num::integer::gcd;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Vec2d;

named!(cell<&str, bool>,
    alt!(
        value!(true, char!('#')) |
        value!(false, char!('.'))
    )
);

named!(
    line<&str, Vec<bool>>,
    many1!(cell)
);

named!(
    input<&str, Vec<Vec<bool>>>,
    many1!(
        do_parse!(
            line: line >>
            line_ending >> (line)
        )
    )
);

fn polar(v: Vec2d) -> (i64, Vec2d) {
    let d = gcd(v.x(), v.y());
    (d, v2d(v.x() / d, v.y() / d))
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
    //println!("{:?}", result);

    let map = result.unwrap().1;

    let mut asteroids = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] {
                asteroids.push(p2d(x as i64, y as i64));
            }
        }
    }
    let mut max_count = 0;
    let mut max_p = None;
    for p0 in &asteroids {
        let mut count = 0;
        for p1 in &asteroids {
            if p1 != p0 {
                let mut visible = true;
                for p2 in &asteroids {
                    if p2 != p0 && p2 != p1 {
                        let v1 = p1 - p0;
                        let v2 = p2 - p0;

                        let (d1, dir1) = polar(v1);
                        let (d2, dir2) = polar(v2);
                        if dir1 == dir2 && d2 < d1 {
                            visible = false;
                        }
                    }
                }
                if visible {
                    count += 1;
                }
            }
        }
        if count > max_count {
            max_count = count;
            max_p = Some(p0);
        }
    }

    println!("{:?}", max_p);

    let result_a = max_count;
    let result_b = 0;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
