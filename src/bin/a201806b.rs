use std::io;
use std::str::FromStr;

use nom::call;
use nom::char;
use nom::digit;
use nom::do_parse;
use nom::error_position;
use nom::map_res;
use nom::named;
use nom::sep;
use nom::wrap_sep;
use nom::ws;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, p1: &Point) -> i32 {
        (p1.x - self.x).abs() + (p1.y - self.y).abs()
    }
}

named!(int32<&str, i32>,
    map_res!(digit, FromStr::from_str)
);

named!(
    position<&str, Point>,
    do_parse!(x: int32 >> ws!(char!(',')) >> y: int32 >> (Point { x, y }))
);

fn main() {
    let mut positions = Vec::new();

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.is_empty() {
            break;
        }

        // parse line
        let result = position(&line);
        //println!("{:?}", result);

        let r = result.unwrap().1;
        positions.push(r);
    }
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let total_distance = positions
                .iter()
                .map(|p| p.distance(&Point { x, y }))
                .sum::<i32>();
            if total_distance < 10000 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
