use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Compounds(HashMap<String, i64>);
impl Compounds {
    fn ordering_1(_compound: &str) -> Ordering {
        Ordering::Equal
    }
    fn ordering_2(compound: &str) -> Ordering {
        if compound == "cats" || compound == "trees" {
            Ordering::Less
        } else if compound == "pomeranians" || compound == "goldfish" {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    fn matches<F>(&self, analysis: &Compounds, ordering: F) -> bool
    where
        F: Fn(&str) -> Ordering,
    {
        self.0
            .iter()
            .all(|(k, v)| analysis.0[k].cmp(v) == ordering(k))
    }
    fn matches_1(&self, analysis: &Compounds) -> bool {
        self.matches(analysis, Self::ordering_1)
    }
    fn matches_2(&self, analysis: &Compounds) -> bool {
        self.matches(analysis, Self::ordering_2)
    }
}
impl fmt::Display for Compounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut sep = "";
        for (k, v) in &self.0 {
            write!(f, "{}{}: {}", sep, k, v)?;
            sep = ", ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Sue {
    id: i64,
    compounds: Compounds,
}
impl fmt::Display for Sue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Sue {}: {}", self.id, self.compounds)
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn compound(i: &str) -> IResult<&str, (String, i64)> {
    let (i, name) = string(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, quantity) = uint(i)?;
    Ok((i, (name, quantity)))
}

fn compounds(i: &str) -> IResult<&str, Compounds> {
    let (i, v) = separated_list1(tag(", "), compound)(i)?;
    Ok((i, Compounds(v.into_iter().collect::<HashMap<_, _>>())))
}

fn sue(i: &str) -> IResult<&str, Sue> {
    let (i, _) = tag("Sue ")(i)?;
    let (i, id) = uint(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, compounds) = compounds(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Sue { id, compounds }))
}

fn input(i: &str) -> IResult<&str, Vec<Sue>> {
    many1(sue)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    //println!("{:#?}", input);
    // for sue in &input {
    //     println!("{}", sue);
    // }

    let compounds = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];
    let analysis = Compounds(
        compounds
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, i64>>(),
    );

    let mut sue_id = None;
    for sue in &input {
        if sue.compounds.matches_1(&analysis) {
            sue_id = Some(sue.id);
        }
    }
    let result_a = sue_id.expect("no sue found");

    let mut sue_id = None;
    for sue in &input {
        if sue.compounds.matches_2(&analysis) {
            sue_id = Some(sue.id);
        }
    }
    let result_b = sue_id.expect("no sue found");

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
