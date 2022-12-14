use core::fmt;
use core::str::FromStr;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vector;

use util::runtime_error;

#[derive(Clone, Debug)]
struct Path {
    points: Vec<Point2d>,
}
impl Path {
    fn draw_on(&self, map: &mut Array2d<i64, Square>) -> util::Result<()> {
        let mut iter = self.points.iter();
        let mut pos = *iter.next().ok_or_else(|| "empty rock formation")?;
        map[pos] = Square::Rock;
        for &next_pos in iter {
            let delta = (next_pos - pos).signum();
            // TODO Check that we will hit the next point.
            while pos != next_pos {
                pos += delta;
                map[pos] = Square::Rock;
            }
        }
        Ok(())
    }
}
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for p in &self.points {
            write!(f, "{}{},{}", sep, p.x(), p.y())?;
            sep = " -> ";
        }
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn point(i: &str) -> IResult<&str, Point2d> {
    let (i, x) = int(i)?;
    let (i, _) = char(',')(i)?;
    let (i, y) = int(i)?;
    Ok((i, p2d(x, y)))
}

fn path(i: &str) -> IResult<&str, Path> {
    let (i, points) = separated_list1(tag(" -> "), point)(i)?;
    Ok((i, Path { points }))
}

fn input(i: &str) -> IResult<&str, Vec<Path>> {
    separated_list0(line_ending, path)(i)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Rock,
    Sand,
}
impl Square {
    fn to_char(self) -> char {
        match self {
            Square::Empty => ',',
            Square::Rock => '#',
            Square::Sand => 'o',
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn simulate_sand(map: &mut Array2d<i64, Square>, source: Point2d) -> usize {
    let bbox = map.bbox();
    let mut sand_count = 0;
    let down = v2d(0, 1);
    let down_left = v2d(-1, 1);
    let down_right = v2d(1, 1);
    let steps = vec![down, down_left, down_right];
    loop {
        let mut pos = source;
        loop {
            let mut at_rest = true;
            for &step in &steps {
                let new_pos = pos + step;
                if bbox.contains(&new_pos) {
                    if map[new_pos] == Square::Empty {
                        pos = new_pos;
                        at_rest = false;
                        break;
                    }
                } else {
                    // The sand falls into the bottomless void.
                    return sand_count;
                }
            }
            if at_rest {
                // The sand comes to rest here.
                map[pos] = Square::Sand;
                sand_count += 1;
                if pos == source {
                    // The sand blocked the source.
                    return sand_count;
                }
                break;
            }
        }
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for path in &input {
    //     println!("{}", path);
    // }

    // Construct map
    let source = p2d(500, 0);

    let mut bbox = BBox2d::enclosing(input.iter().flat_map(|path| path.points.iter()))
        .ok_or(runtime_error!("packet not found"))?;
    bbox = bbox.extend_to(source);

    let mut map1 = Array2d::new(bbox, Square::Empty);
    for path in &input {
        path.draw_on(&mut map1)?;
    }
    let result1 = simulate_sand(&mut map1, source);

    let y_max = bbox.max().y() + 2;
    let x_min = source.x() - y_max;
    let x_max = source.x() + y_max;
    bbox = bbox.extend_to(p2d(x_min, y_max));
    bbox = bbox.extend_to(p2d(x_max, y_max));
    let mut map2 = Array2d::new(bbox, Square::Empty);
    for x in x_min..=x_max {
        map2[p2d(x, y_max)] = Square::Rock;
    }
    for path in &input {
        path.draw_on(&mut map2)?;
    }
    let result2 = simulate_sand(&mut map2, source);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
