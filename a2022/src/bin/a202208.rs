use core::fmt;

use std::error;
use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;
use lowdim::Vector;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Tree {
    height: u32,
}
impl Tree {
    fn to_char(self) -> char {
        char::from_digit(self.height, 10).unwrap()
    }
}
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct HeightMap {
    map: Array2d<i64, Tree>,
}
impl HeightMap {
    fn new(map: Array2d<i64, Tree>) -> HeightMap {
        HeightMap { map }
    }
    fn visible_count(&self) -> usize {
        self.map.bbox().iter().filter(|&p| self.visible(p)).count()
    }
    fn visible(&self, p: Point2d) -> bool {
        Vec2d::unit_vecs_l1().any(|v| self.visible_from(p, v))
    }
    fn visible_from(&self, p: Point2d, dir: Vec2d) -> bool {
        let h = self.map[p].height;
        let mut p1 = p + dir;
        while self.bbox().contains(&p1) {
            if self.map[p1].height >= h {
                // This view is obstructed.
                return false;
            }
            p1 += dir;
        }
        // We found an unobstructed view to the outside.
        true
    }
    fn scenic_score(&self, p: Point2d) -> usize {
        Vec2d::unit_vecs_l1()
            .map(|v| self.viewing_distance(p, v))
            .product::<usize>()
    }
    fn viewing_distance(&self, p: Point2d, dir: Vec2d) -> usize {
        let h = self.map[p].height;
        let mut p1 = p + dir;
        let mut result = 0;
        while self.bbox().contains(&p1) {
            result += 1;
            if self.map[p1].height >= h {
                break;
            }
            p1 += dir;
        }
        result
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range() {
            for x in self.bbox().x_range() {
                write!(f, "{}", self.map[p2d(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn tree(i: &str) -> IResult<&str, Tree> {
    let (i, c) = one_of("0123456789")(i)?;
    Ok((
        i,
        Tree {
            height: c.to_digit(10).unwrap(),
        },
    ))
}

fn line(i: &str) -> IResult<&str, Vec<Tree>> {
    many1(tree)(i)
}

fn input(i: &str) -> IResult<&str, HeightMap> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((i, HeightMap::new(Array2d::from_vec(rows))))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let result1 = input.visible_count();

    let result2 = input
        .bbox()
        .iter()
        .map(|p| input.scenic_score(p))
        .max()
        .unwrap();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
