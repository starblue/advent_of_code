use core::fmt;

use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Octopus {
    energy_level: u32,
}
impl Octopus {
    fn inc(&mut self) {
        if self.energy_level <= 9 {
            self.energy_level += 1;
        }
    }
    fn reset(&mut self) {
        self.energy_level = 0;
    }
    fn flashes(&self) -> bool {
        self.energy_level > 9
    }
    fn to_char(self) -> char {
        char::from_digit(self.energy_level, 10).unwrap()
    }
}
impl fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct EnergyLevelMap {
    map: Array2d<i64, Octopus>,
    flash_count: usize,
    all_just_flashed: bool,
}
impl EnergyLevelMap {
    fn new(map: Array2d<i64, Octopus>) -> EnergyLevelMap {
        EnergyLevelMap {
            map,
            flash_count: 0,
            all_just_flashed: false,
        }
    }
    fn step(&mut self) {
        let mut new_flashers = Vec::new();
        for p in self.bbox() {
            self.map[p].inc();
            if self.map[p].flashes() {
                new_flashers.push(p);
            }
        }
        let mut handled_flashers = HashSet::new();
        while let Some(p) = new_flashers.pop() {
            if !handled_flashers.contains(&p) {
                handled_flashers.insert(p);
                for np in p.neighbors_l_infty() {
                    if let Some(octopus) = self.map.get_mut(np) {
                        octopus.inc();
                        if octopus.flashes() {
                            new_flashers.push(np);
                        }
                    }
                }
            }
        }
        self.flash_count += handled_flashers.len();
        self.all_just_flashed = handled_flashers.len() == self.bbox().volume();
        for p in handled_flashers {
            self.map[p].reset();
        }
    }
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for EnergyLevelMap {
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

fn octopus(i: &str) -> IResult<&str, Octopus> {
    let (i, c) = one_of("0123456789")(i)?;
    Ok((
        i,
        Octopus {
            energy_level: c.to_digit(10).unwrap(),
        },
    ))
}

fn line(i: &str) -> IResult<&str, Vec<Octopus>> {
    many1(octopus)(i)
}

fn input(i: &str) -> IResult<&str, EnergyLevelMap> {
    let (i, rows) = separated_list1(line_ending, line)(i)?;
    Ok((i, EnergyLevelMap::new(Array2d::from_vec(rows))))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let mut map = input.clone();
    for _ in 0..100 {
        map.step();
    }
    let result_a = map.flash_count;

    let mut map = input;
    let mut step_count = 0;
    while !map.all_just_flashed {
        map.step();
        step_count += 1;
    }
    let result_b = step_count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
