use core::fmt;
use core::str::FromStr;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct MapItem {
    dst: usize,
    src: usize,
    len: usize,
}
impl MapItem {
    fn map(&self, n: usize) -> Option<usize> {
        if self.src <= n && n < self.src + self.len {
            Some(n - self.src + self.dst)
        } else {
            None
        }
    }
}
impl fmt::Display for MapItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {} {}", self.dst, self.src, self.len)
    }
}

#[derive(Clone, Debug)]
struct Map {
    name: String,
    items: Vec<MapItem>,
}
impl Map {
    fn map(&self, n: usize) -> usize {
        for item in &self.items {
            if let Some(result) = item.map(n) {
                return result;
            }
        }
        n
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "{} map:", self.name)?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}
impl Input {
    fn map(&self, n: usize) -> usize {
        let mut n = n;
        for map in &self.maps {
            n = map.map(n);
        }
        n
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "seeds: ")?;
        let mut sep = "";
        for seed in &self.seeds {
            write!(f, "{}{}", sep, seed)?;
            sep = " ";
        }
        writeln!(f)?;
        writeln!(f)?;
        let mut sep = "";
        for map in &self.maps {
            write!(f, "{}{}", sep, map)?;
            sep = "\n";
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn seeds(i: &str) -> IResult<&str, Vec<usize>> {
    let (i, _) = tag("seeds: ")(i)?;
    separated_list1(space1, uint)(i)
}

fn map_item(i: &str) -> IResult<&str, MapItem> {
    let (i, dst) = uint(i)?;
    let (i, _) = space1(i)?;
    let (i, src) = uint(i)?;
    let (i, _) = space1(i)?;
    let (i, len) = uint(i)?;
    Ok((i, MapItem { dst, src, len }))
}

fn name(i: &str) -> IResult<&str, String> {
    let (i, cs) = many1(one_of("abcdefghijklmnopqrstuvwxyz-"))(i)?;
    Ok((i, cs.into_iter().collect::<String>()))
}

fn map(i: &str) -> IResult<&str, Map> {
    let (i, name) = name(i)?;
    let (i, _) = tag(" map:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, items) = separated_list1(line_ending, map_item)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Map { name, items }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, seeds) = seeds(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, maps) = separated_list1(line_ending, map)(i)?;
    Ok((i, Input { seeds, maps }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input
        .seeds
        .iter()
        .map(|&seed| input.map(seed))
        .min()
        .ok_or("input is empty")?;

    let result2 = input
        .seeds
        .chunks(2)
        .flat_map(|ch| ch[0]..(ch[0] + ch[1]))
        .map(|seed| input.map(seed))
        .min()
        .ok_or("input is empty")?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
