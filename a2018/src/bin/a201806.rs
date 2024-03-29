use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::combinator::map_res;
use nom::IResult;

use lowdim::p2d;
use lowdim::Point2d;

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn position(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = int(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace1(i)?;
    let (i, y) = int(i)?;
    Ok((i, p2d(x, y)))
}

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
            let mut min_d = std::i64::MAX;
            let mut min_i = None;
            for (i, p) in positions.iter().enumerate() {
                let d = p.distance_l1(p2d(x, y));
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
        let pos_x_finite = positions.iter().any(|p1| (p1 - p).is_towards_pos_x());
        let neg_x_finite = positions.iter().any(|p1| (p1 - p).is_towards_neg_x());
        let pos_y_finite = positions.iter().any(|p1| (p1 - p).is_towards_pos_y());
        let neg_y_finite = positions.iter().any(|p1| (p1 - p).is_towards_neg_y());
        if pos_x_finite && neg_x_finite && pos_y_finite && neg_y_finite {
            // area is finite
            if c > count_max {
                count_max = c;
            }
        }
    }
    println!("a: {}", count_max);

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

    println!("b: {}", count);
}
