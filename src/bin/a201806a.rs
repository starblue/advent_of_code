use std::io;
use std::iter::repeat;
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

    fn is_east_of(&self, p1: &Point) -> bool {
        p1.x - self.x > (p1.y - self.y).abs()
    }
    fn is_north_of(&self, p1: &Point) -> bool {
        p1.y - self.y > (p1.x - self.x).abs()
    }
    fn is_west_of(&self, p1: &Point) -> bool {
        self.x - p1.x > (p1.y - self.y).abs()
    }
    fn is_south_of(&self, p1: &Point) -> bool {
        self.y - p1.y > (p1.x - self.x).abs()
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
    let mut counts = repeat(0).take(positions.len()).collect::<Vec<_>>();
    for x in 0..1000 {
        for y in 0..1000 {
            let mut min_d = std::i32::MAX;
            let mut min_i = None;
            for (i, p) in positions.iter().enumerate() {
                let d = p.distance(&Point { x, y });
                if d < min_d {
                    min_d = d;
                    min_i = Some(i);
                } else if d == min_d {
                    // two or more with the same distance
                    min_i = None;
                } else {
                    // do nothing
                }
            }
            if let Some(i) = min_i {
                counts[i] += 1;
            }
        }
    }

    let mut count_max = 0;
    for (p, &c) in positions.iter().zip(counts.iter()) {
        let e_finite = positions.iter().any(|p1| p1.is_east_of(p));
        let n_finite = positions.iter().any(|p1| p1.is_north_of(p));
        let w_finite = positions.iter().any(|p1| p1.is_west_of(p));
        let s_finite = positions.iter().any(|p1| p1.is_south_of(p));
        if e_finite && n_finite && w_finite && s_finite {
            // area is finite
            if c > count_max {
                count_max = c;
            }
        }
    }

    println!("{}", count_max);
}
