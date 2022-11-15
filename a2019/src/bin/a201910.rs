use core::cmp;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use num::integer::gcd;

use lowdim::v2d;
use lowdim::Array2d;
use lowdim::Vec2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Direction(Vec2d);
impl Direction {
    fn quadrant_normalized(&self) -> (usize, Vec2d) {
        if self.0.y() < 0 && self.0.x() >= 0 {
            (0, self.0.rotate_left())
        } else if self.0.x() > 0 && self.0.y() >= 0 {
            (1, self.0)
        } else if self.0.y() > 0 && self.0.x() <= 0 {
            (2, self.0.rotate_right())
        } else if self.0.x() < 0 && self.0.x() <= 0 {
            (3, -self.0)
        } else {
            panic!("direction 0/0 is undefined");
        }
    }
}
impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Direction) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Direction {
    /// Returns the ordering of directions so that up is smallest and
    /// directions increase clockwise until just befor reaching up again.
    ///
    /// This corresponds to one rotation of the giant laser.
    fn cmp(&self, other: &Direction) -> cmp::Ordering {
        let (sq, sv) = self.quadrant_normalized();
        let (oq, ov) = other.quadrant_normalized();
        sq.cmp(&oq).then((sv.y() * ov.x()).cmp(&(sv.x() * ov.y())))
    }
}

fn cell(i: &str) -> IResult<&str, bool> {
    alt((value(true, char('#')), value(false, char('.'))))(i)
}

fn line(i: &str) -> IResult<&str, Vec<bool>> {
    let (i, line) = many1(cell)(i)?;
    Ok((i, line))
}

fn input(i: &str) -> IResult<&str, Vec<Vec<bool>>> {
    separated_list1(line_ending, line)(i)
}

/// Converts a vector into integer polar coordinates.
///
/// The direction is given as a vector without common factors,
/// and the distance as the quotient of the input by this vector,
/// i.e. the common factor.
fn polar(v: Vec2d) -> (i64, Direction) {
    let d = gcd(v.x(), v.y());
    (d, Direction(v2d(v.x() / d, v.y() / d)))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;

    let map = Array2d::from_vec(input);
    // use lowdim::p2d;
    // for y in map.bbox().y_range() {
    //     for x in map.bbox().x_range() {
    //         print!("{}", if map[p2d(x, y)] { '#' } else { '.' });
    //     }
    //     println!();
    // }

    let mut asteroids = Vec::new();
    for p in map.bbox().iter() {
        if map[p] {
            asteroids.push(p);
        }
    }
    let mut max_count = 0;
    let mut max_p = None;
    for &p0 in &asteroids {
        // In each direction one asteriod is visible, so count those.
        let mut directions = HashSet::new();
        for &p1 in &asteroids {
            if p1 != p0 {
                let (_, dir) = polar(p1 - p0);
                directions.insert(dir);
            }
        }
        let count = directions.len();
        if count > max_count {
            max_count = count;
            max_p = Some(p0);
        }
    }
    let result_a = max_count;

    // The location of the laser.
    let p0 = max_p.unwrap();

    // Collect asteroids by direction from the laser.
    // The value for each direction is sorted by distance.
    let mut directions = BTreeMap::new();
    for &p in &asteroids {
        if p != p0 {
            let (dist, dir) = polar(p - p0);
            let e = directions.entry(dir).or_insert_with(Vec::new);
            e.push((dist, p));
            e.sort_by_key(|&(dist, _)| dist);
        }
    }

    // The directions for one rotation of the laser.
    let rotation = directions.keys().cloned().collect::<Vec<_>>();
    let mut count = 0;
    let mut p;
    'outer: loop {
        for &dir in &rotation {
            let e = directions.entry(dir).or_insert_with(Vec::new);
            if !e.is_empty() {
                // Vaporize asteroid!
                p = e.remove(0).1;
                count += 1;
                if count == 200 {
                    break 'outer;
                }
            }
        }
    }
    let result_b = p.x() * 100 + p.y();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use lowdim::v2d;

    use crate::Direction;

    #[test]
    fn test_quadrant_normalize() {
        assert_eq!((0, v2d(1, 0)), Direction(v2d(0, -1)).quadrant_normalized());
        assert_eq!((0, v2d(1, 1)), Direction(v2d(1, -1)).quadrant_normalized());
        assert_eq!((1, v2d(1, 0)), Direction(v2d(1, 0)).quadrant_normalized());
        assert_eq!((1, v2d(1, 1)), Direction(v2d(1, 1)).quadrant_normalized());
        assert_eq!((2, v2d(1, 0)), Direction(v2d(0, 1)).quadrant_normalized());
        assert_eq!((2, v2d(1, 1)), Direction(v2d(-1, 1)).quadrant_normalized());
        assert_eq!((3, v2d(1, 0)), Direction(v2d(-1, 0)).quadrant_normalized());
        assert_eq!((3, v2d(1, 1)), Direction(v2d(-1, -1)).quadrant_normalized());
    }

    #[test]
    fn test_ord() {
        assert!(Direction(v2d(0, -1)) < Direction(v2d(1, -2)));
        assert!(Direction(v2d(1, -2)) < Direction(v2d(1, -1)));
        assert!(Direction(v2d(1, -1)) < Direction(v2d(2, -1)));
        assert!(Direction(v2d(2, -1)) < Direction(v2d(1, 0)));
        assert!(Direction(v2d(1, 0)) < Direction(v2d(1, 1)));
        assert!(Direction(v2d(1, 1)) < Direction(v2d(0, 1)));
        assert!(Direction(v2d(0, 1)) < Direction(v2d(-1, 1)));
        assert!(Direction(v2d(-1, 1)) < Direction(v2d(-1, 0)));
        assert!(Direction(v2d(-1, 0)) < Direction(v2d(-1, -1)));
        assert!(Direction(v2d(-1, -1)) > Direction(v2d(0, -1)));
    }
}
