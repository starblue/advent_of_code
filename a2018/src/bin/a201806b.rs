use std::io;
use std::str::FromStr;

use nom::char;
use nom::digit;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::ws;

use gamedim::p2d;
use gamedim::Point2d;

named!(int<&str, i64>,
    map_res!(digit, FromStr::from_str)
);

named!(
    position<&str, Point2d>,
    do_parse!(x: int >> ws!(char!(',')) >> y: int >> (p2d(x, y)))
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
                .map(|p| p.distance_l1(p2d(x, y)))
                .sum::<i64>();
            if total_distance < 10000 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
