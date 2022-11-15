use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::many_m_n;
use nom::IResult;

fn uint_len(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        let mut rest = n;
        let mut result = 0;
        while rest > 0 {
            rest /= 10;
            result += 1;
        }
        result
    }
}

#[derive(Clone, Debug)]
enum Chunk {
    Plain(String),
    Repeat(CompressedData, usize),
}
impl Chunk {
    fn len_uncompressed_v1(&self) -> usize {
        match self {
            Chunk::Plain(s) => s.len(),
            Chunk::Repeat(cd, n) => n * cd.len_compressed(),
        }
    }
    fn len_uncompressed_v2(&self) -> usize {
        match self {
            Chunk::Plain(s) => s.len(),
            Chunk::Repeat(cd, n) => n * cd.len_uncompressed_v2(),
        }
    }
    fn len_compressed(&self) -> usize {
        match self {
            Chunk::Plain(s) => s.len(),
            Chunk::Repeat(cd, n) => {
                let len_cd = cd.len_compressed();
                let len_n = uint_len(*n);
                let len_len = uint_len(len_cd);
                3 + len_len + len_n + len_cd
            }
        }
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Chunk::Plain(s) => write!(f, "{}", s),
            Chunk::Repeat(cd, n) => write!(f, "({}x{}){}", cd.len_compressed(), n, cd),
        }
    }
}

#[derive(Clone, Debug)]
struct CompressedData(Vec<Chunk>);
impl CompressedData {
    fn len_uncompressed_v1(&self) -> usize {
        self.0.iter().map(|c| c.len_uncompressed_v1()).sum()
    }
    fn len_uncompressed_v2(&self) -> usize {
        self.0.iter().map(|c| c.len_uncompressed_v2()).sum()
    }
    fn len_compressed(&self) -> usize {
        self.0.iter().map(|c| c.len_compressed()).sum()
    }
}
impl fmt::Display for CompressedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for chunk in &self.0 {
            write!(f, "{}", chunk)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(many1(none_of("( \t\r\n"))), String::from)(i)
}

fn string_n(n: usize, i: &str) -> IResult<&str, String> {
    map(recognize(many_m_n(n, n, none_of(" \t\r\n"))), String::from)(i)
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn chunk_plain(i: &str) -> IResult<&str, Chunk> {
    let (i, s) = string(i)?;
    Ok((i, Chunk::Plain(s)))
}

fn chunk_repeat(i: &str) -> IResult<&str, Chunk> {
    let (i, _) = tag("(")(i)?;
    let (i, len) = uint(i)?;
    let (i, _) = tag("x")(i)?;
    let (i, n) = uint(i)?;
    let (i, _) = tag(")")(i)?;
    let (i, s) = string_n(len, i)?;
    // recursively parse s as compressed data
    let (_, cd) = compressed_data(&s).unwrap();
    Ok((i, Chunk::Repeat(cd, n)))
}

fn chunk(i: &str) -> IResult<&str, Chunk> {
    let (i, _) = multispace0(i)?;
    let (i, c) = alt((chunk_plain, chunk_repeat))(i)?;
    Ok((i, c))
}

fn compressed_data(i: &str) -> IResult<&str, CompressedData> {
    let (i, cs) = many1(chunk)(i)?;
    Ok((i, CompressedData(cs)))
}

fn input(i: &str) -> IResult<&str, CompressedData> {
    let (i, cd) = compressed_data(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, cd))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let result_a = input.len_uncompressed_v1();

    let result_b = input.len_uncompressed_v2();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
