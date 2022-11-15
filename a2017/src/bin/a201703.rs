use std::collections::HashMap;
use std::io;
use std::str::FromStr;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;

use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::IResult;

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, i64> {
    let (i, n) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, n))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let a = input;

    let r = ((a - 1) as f64).sqrt() as i64;
    // side length of smallest square containing a
    let s = ((r + 1) / 2) * 2 + 1;
    // side length of largest square not containing a
    let s0 = s - 2;

    // maximal absolute coordinate value
    let d = (s - 1) / 2;
    // start value of border turn
    let b = s0 * s0;
    let da = if s > 1 { (a - b) % (s - 1) - d } else { 0 };
    let result_a = d + da.abs();

    let mut grid: HashMap<Point2d, i64> = HashMap::new();

    let p0 = p2d(0, 0);
    grid.insert(p0, 1);

    // start point and direction
    let mut p = p0;
    let mut d = v2d(1, 0);

    let mut value;
    loop {
        // go one step
        p += d;

        // direction left
        let d_left = d.rotate_left();
        let p_left = p + d_left;
        // if square left is empty then turn left
        if !grid.contains_key(&p_left) {
            d = d_left;
        }

        // compute sum of neighbors
        value = p.neighbors_l_infty().flat_map(|np| grid.get(&np)).sum();
        grid.insert(p, value);

        if value > input {
            break;
        }
    }
    let result_b = value;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
