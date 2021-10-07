use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::combinator::opt;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Token(String);
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
struct Replacement {
    left: Token,
    right: Vec<Token>,
}
impl Replacement {}
impl fmt::Display for Replacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} => ", self.left)?;
        for t in &self.right {
            write!(f, "{} ", t)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    replacements: Vec<Replacement>,
    molecule: Vec<Token>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for r in &self.replacements {
            writeln!(f, "{}", r)?;
        }
        writeln!(f)?;
        for t in &self.molecule {
            write!(f, "{} ", t)?;
        }
        Ok(())
    }
}

fn token(i: &str) -> IResult<&str, Token> {
    let (i, c0) = one_of("eABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)?;
    let (i, oc1) = opt(one_of("abcdefghijklmnopqrstuvwxyz"))(i)?;
    if let Some(c1) = oc1 {
        let mut s = String::new();
        s.push(c0);
        s.push(c1);
        Ok((i, Token(s)))
    } else {
        Ok((i, Token(c0.into())))
    }
}

fn molecule(i: &str) -> IResult<&str, Vec<Token>> {
    many1(token)(i)
}

fn replacement(i: &str) -> IResult<&str, Replacement> {
    let (i, left) = token(i)?;
    let (i, _) = tag(" => ")(i)?;
    let (i, right) = molecule(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Replacement { left, right }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, replacements) = many1(replacement)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, molecule) = molecule(i)?;
    Ok((
        i,
        Input {
            replacements,
            molecule,
        },
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Token2 {
    Input(Token),
    Gensym(i64),
}
impl fmt::Display for Token2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Token2::Input(token) => write!(f, "{}", token),
            Token2::Gensym(id) => write!(f, "G{}", id),
        }
    }
}

#[derive(Clone, Debug)]
struct Replacement2 {
    left: Token2,
    right0: Token2,
    right1: Token2,
}
impl Replacement2 {
    fn from(r: &Replacement, gensym_counter: &mut i64) -> Vec<Replacement2> {
        let mut result = Vec::new();
        let mut left = Token2::Input(r.left.clone());
        let mut iter = r.right.iter();
        let mut right0 = Token2::Input(iter.next().unwrap().clone());
        let mut right1 = Token2::Input(iter.next().unwrap().clone());
        for t in iter {
            let t1 = Token2::Gensym(*gensym_counter);
            *gensym_counter += 1;
            result.push(Replacement2 {
                left,
                right0,
                right1: t1.clone(),
            });
            left = t1;
            right0 = right1;
            right1 = Token2::Input(t.clone());
        }
        result.push(Replacement2 {
            left,
            right0,
            right1,
        });
        result
    }
    fn cost(&self) -> i64 {
        match &self.left {
            Token2::Input(_token) => 1,
            Token2::Gensym(_id) => 0,
        }
    }
}
impl fmt::Display for Replacement2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} => {} {}", self.left, self.right0, self.right1)
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    //println!("{}", input);

    let mut molecules = HashSet::new();
    for r in &input.replacements {
        for (i, t) in input.molecule.iter().enumerate() {
            if t == &r.left {
                let mut prefix = input.molecule[..i].to_vec();
                let mut suffix = input.molecule[(i + 1)..].to_vec();
                let mut new_molecule = Vec::new();
                new_molecule.append(&mut prefix);
                new_molecule.append(&mut r.right.clone());
                new_molecule.append(&mut suffix);
                molecules.insert(new_molecule);
            }
        }
    }
    let result_a = molecules.len();

    let mut r2s = Vec::new();
    let mut gensym_counter = 0;
    for r in &input.replacements {
        r2s.append(&mut Replacement2::from(r, &mut gensym_counter));
    }

    let mut reductions = HashMap::new();
    for r2 in &r2s {
        let entry = reductions
            .entry((&r2.right0, &r2.right1))
            .or_insert_with(HashSet::new);
        (*entry).insert((r2.left.clone(), r2.cost()));
    }

    let mut table = Vec::new();
    // dummy row for subsequences of length 0
    table.push(Vec::new());
    // row1[i]: map from tokens to costs for subsequence of length 1 starting at i
    let mut row1 = Vec::new();
    for t in &input.molecule {
        let mut token_costs = HashMap::new();
        // cost is 0 because it is part of the input
        token_costs.insert(Token2::Input(t.clone()), 0);
        row1.push(token_costs);
    }
    table.push(row1);
    for len in 2..=input.molecule.len() {
        // row for subsequences of length len
        let mut row = Vec::new();
        for i in 0..=(input.molecule.len() - len) {
            let mut token_costs = HashMap::new();
            // split subsequence starting at i into two nonempty parts and
            // search for replacement applicable in reverse
            for len0 in 1..len {
                let len1 = len - len0;
                let token_costs0 = &table[len0][i];
                let token_costs1 = &table[len1][i + len0];
                for (t0, c0) in token_costs0 {
                    for (t1, c1) in token_costs1 {
                        if let Some(set) = reductions.get(&(t0, t1)) {
                            for (left, cost) in set {
                                token_costs.insert(left.clone(), cost + c0 + c1);
                            }
                        }
                    }
                }
            }
            row.push(token_costs);
        }
        table.push(row);
    }
    let token_costs = &table[input.molecule.len()][0];
    let result_b = token_costs[&Token2::Input(Token("e".to_string()))];

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
