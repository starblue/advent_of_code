use std::collections::HashSet;
use std::fmt;
use std::io;
use std::ops::Index;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pixel {
    Light,
    Dark,
}
impl Pixel {
    fn value(&self) -> usize {
        match self {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
    fn complement(&self) -> Pixel {
        match self {
            Pixel::Light => Pixel::Dark,
            Pixel::Dark => Pixel::Light,
        }
    }
    fn to_char(self) -> char {
        match self {
            Pixel::Light => '#',
            Pixel::Dark => '.',
        }
    }
}
impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Algorithm(Vec<Pixel>);
impl Algorithm {
    fn output(&self, index: usize) -> Pixel {
        self.0[index]
    }
    fn default_output(&self, default_input: Pixel) -> Pixel {
        match default_input {
            Pixel::Light => self.output(511),
            Pixel::Dark => self.output(0),
        }
    }
}
impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pixel in &self.0 {
            write!(f, "{}", pixel)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Image {
    map: Array2d<i64, Pixel>,
}
impl fmt::Display for Image {
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

#[derive(Clone, Debug)]
struct Input {
    algorithm: Algorithm,
    image: Image,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.algorithm)?;
        writeln!(f)?;
        writeln!(f, "{}", self.image)
    }
}

fn pixel(i: &str) -> IResult<&str, Pixel> {
    alt((value(Pixel::Light, tag("#")), value(Pixel::Dark, tag("."))))(i)
}

fn row(i: &str) -> IResult<&str, Vec<Pixel>> {
    many1(pixel)(i)
}

fn algorithm(i: &str) -> IResult<&str, Algorithm> {
    let (i, row) = row(i)?;
    Ok((i, Algorithm(row)))
}

fn image(i: &str) -> IResult<&str, Image> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    Ok((
        i,
        Image {
            map: Array2d::from_vec(rows),
        },
    ))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, algorithm) = algorithm(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, image) = image(i)?;
    Ok((i, Input { algorithm, image }))
}

#[derive(Clone, Debug)]
struct InfImage {
    default: Pixel,
    other: Pixel,
    others: HashSet<Point2d>,
}
impl InfImage {
    fn new(default: Pixel) -> InfImage {
        let other = default.complement();
        let others = HashSet::new();
        InfImage {
            default,
            other,
            others,
        }
    }
    fn from_array(image: &Array2d<i64, Pixel>, default: Pixel) -> InfImage {
        let mut result = InfImage::new(default);
        for p in image.bbox() {
            result.set(p, image[p]);
        }
        result
    }
    fn set(&mut self, p: Point2d, pixel: Pixel) {
        if pixel == self.default {
            self.others.remove(&p);
        } else {
            self.others.insert(p);
        }
    }
    fn enhanced(&self, algorithm: &Algorithm) -> InfImage {
        #[rustfmt::skip]
        let deltas = &[
            v2d(-1, -1), v2d(0, -1), v2d(1, -1),
            v2d(-1,  0), v2d(0,  0), v2d(1,  0),
            v2d(-1,  1), v2d(0,  1), v2d(1,  1),
        ];
        let mut ps = self.others.clone();
        for p in &self.others {
            for np in p.neighbors_l_infty() {
                ps.insert(np);
            }
        }
        let mut result = InfImage::new(algorithm.default_output(self.default));
        for p in ps {
            let mut value = 0;
            for d in deltas {
                value = 2 * value + self[p + d].value();
            }
            result.set(p, algorithm.output(value));
        }
        result
    }
    fn count(&self, pixel: Pixel) -> Option<usize> {
        if pixel == self.other {
            Some(self.others.len())
        } else {
            None
        }
    }
}
impl Index<Point2d> for InfImage {
    type Output = Pixel;
    fn index(&self, index: Point2d) -> &Pixel {
        if self.others.contains(&index) {
            &self.other
        } else {
            &self.default
        }
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let mut image = InfImage::from_array(&input.image.map, Pixel::Dark);
    for _ in 0..2 {
        image = image.enhanced(&input.algorithm);
    }
    let result_a = image.count(Pixel::Light).unwrap();

    let mut image = InfImage::from_array(&input.image.map, Pixel::Dark);
    for _ in 0..50 {
        image = image.enhanced(&input.algorithm);
    }
    let result_b = image.count(Pixel::Light).unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
