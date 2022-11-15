use std::fmt;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}
impl Ingredient {}
impl fmt::Display for Ingredient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
            self.name, self.capacity, self.durability, self.flavor, self.texture, self.calories,
        )
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn ingredient(i: &str) -> IResult<&str, Ingredient> {
    let (i, name) = string(i)?;
    let (i, _) = tag(": capacity ")(i)?;
    let (i, capacity) = int(i)?;
    let (i, _) = tag(", durability ")(i)?;
    let (i, durability) = int(i)?;
    let (i, _) = tag(", flavor ")(i)?;
    let (i, flavor) = int(i)?;
    let (i, _) = tag(", texture ")(i)?;
    let (i, texture) = int(i)?;
    let (i, _) = tag(", calories ")(i)?;
    let (i, calories) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Ingredient>> {
    many1(ingredient)(i)
}

fn next_partition(p: &[i64]) -> Option<Vec<i64>> {
    let mut v = p.to_vec();
    let mut i = v.len();
    while i > 0 && v[i - 1] == 0 {
        i -= 1;
    }
    if i <= 1 {
        None
    } else {
        // move one to the previous part and the rest to the last part
        v[i - 2] += 1;
        let rest = v[i - 1] - 1;
        v[i - 1] = 0;
        let j = v.len() - 1;
        v[j] = rest;
        Some(v)
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    //println!("{:#?}", input);
    // for ingredient in &input {
    //     println!("{}", ingredient);
    // }

    let mut max_score = 0;
    let mut partition = vec![0, 0, 0, 100];
    while let Some(p) = next_partition(&partition) {
        partition = p;

        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        for (n, i) in partition.iter().zip(input.iter()) {
            capacity += n * i.capacity;
            durability += n * i.durability;
            flavor += n * i.flavor;
            texture += n * i.texture;
        }
        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);

        let score = capacity * durability * flavor * texture;
        max_score = max_score.max(score);
    }
    let result_a = max_score;

    let mut max_score = 0;
    let mut partition = vec![0, 0, 0, 100];
    while let Some(p) = next_partition(&partition) {
        partition = p;

        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;
        for (n, i) in partition.iter().zip(input.iter()) {
            capacity += n * i.capacity;
            durability += n * i.durability;
            flavor += n * i.flavor;
            texture += n * i.texture;
            calories += n * i.calories;
        }
        capacity = capacity.max(0);
        durability = durability.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);

        if calories == 500 {
            let score = capacity * durability * flavor * texture;
            max_score = max_score.max(score);
        }
    }
    let result_b = max_score;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
