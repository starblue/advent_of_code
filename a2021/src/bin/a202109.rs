use std::fmt;
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

use util::DisjointSets;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    height: u32,
}
impl Location {
    fn to_char(self) -> char {
        char::from_digit(self.height, 10).unwrap()
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct HeightMap {
    map: Array2d<i64, Location>,
}
impl HeightMap {
    fn is_low_point(&self, p: Point2d) -> bool {
        if let Some(loc) = self.map.get(p) {
            p.neighbors_l1().all(|np| {
                if let Some(nloc) = self.map.get(np) {
                    nloc.height > loc.height
                } else {
                    true
                }
            })
        } else {
            false
        }
    }
    fn risk_level(&self, p: Point2d) -> Option<u32> {
        if self.is_low_point(p) {
            Some(1 + self.map[p].height)
        } else {
            None
        }
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
    fn height(&self, p: Point2d) -> u32 {
        self.map[p].height
    }
}
impl fmt::Display for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.map.bbox().y_range() {
            for x in self.map.bbox().x_range() {
                write!(f, "{}", self.map[p2d(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn location(i: &str) -> IResult<&str, Location> {
    let (i, c) = one_of("0123456789")(i)?;
    Ok((
        i,
        Location {
            height: c.to_digit(10).unwrap(),
        },
    ))
}

fn line(i: &str) -> IResult<&str, Vec<Location>> {
    many1(location)(i)
}

fn input(i: &str) -> IResult<&str, HeightMap> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((
        i,
        HeightMap {
            map: Array2d::from_vec(rows),
        },
    ))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let map = result.unwrap().1;
    // println!("{}", map);

    let bbox = map.bbox();

    let result_a = bbox.iter().filter_map(|p| map.risk_level(p)).sum::<u32>();

    // Basins are represented as disjoint sets.
    let mut basins = DisjointSets::new();
    for p in bbox.iter() {
        basins.add(p);
    }
    for p in bbox.iter() {
        let h = map.height(p);
        for np in p.neighbors_l1() {
            if bbox.contains(&np) {
                let nh = map.height(np);
                if h < 9 && nh < 9 {
                    basins.union(&p, &np);
                }
            }
        }
    }
    let mut basin_sizes = basins
        .set_reprs()
        .iter()
        .map(|&p| basins.set_size(p))
        .collect::<Vec<_>>();
    basin_sizes.sort();
    let size0 = basin_sizes.pop().unwrap();
    let size1 = basin_sizes.pop().unwrap();
    let size2 = basin_sizes.pop().unwrap();
    let result_b = size0 * size1 * size2;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
