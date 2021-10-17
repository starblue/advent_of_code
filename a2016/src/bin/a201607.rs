use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;
use std::iter::once;

use nom::bytes::complete::tag;
use nom::character::complete::alpha0;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

fn contains_abba(s: &str) -> bool {
    let cs = s.chars().collect::<Vec<_>>();
    cs.windows(4).any(|w| {
        if let [c0, c1, c2, c3] = w {
            c0 != c1 && c0 == c3 && c1 == c2
        } else {
            false
        }
    })
}

fn insert_aba_matches(matches: &mut HashSet<(char, char)>, s: &str) {
    let cs = s.chars().collect::<Vec<_>>();
    cs.windows(3)
        .filter_map(|w| {
            if let &[c0, c1, c2] = w {
                if c0 != c1 && c0 == c2 {
                    Some((c0, c1))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .for_each(|p| {
            matches.insert(p);
        })
}

#[derive(Clone, Debug)]
struct Address {
    supernet: Vec<String>,
    hypernet: Vec<String>,
}
impl Address {
    fn supports_tls(&self) -> bool {
        self.supernet.iter().any(|s| contains_abba(s))
            && !self.hypernet.iter().any(|s| contains_abba(s))
    }
    fn supports_ssl(&self) -> bool {
        let mut s_abas = HashSet::new();
        for s in &self.supernet {
            insert_aba_matches(&mut s_abas, s);
        }
        let mut h_abas = HashSet::new();
        for h in &self.hypernet {
            insert_aba_matches(&mut h_abas, h);
        }
        s_abas.iter().any(|&(a, b)| h_abas.contains(&(b, a)))
    }
}
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut si = self.supernet.iter();
        let mut hi = self.hypernet.iter();
        loop {
            if let Some(b) = si.next() {
                write!(f, "{}", b)?;
            } else {
                break;
            }
            if let Some(h) = hi.next() {
                write!(f, "[{}]", h)?;
            } else {
                break;
            }
        }
        Ok(())
    }
}

fn string(i: &str) -> IResult<&str, String> {
    map(recognize(alpha0), String::from)(i)
}

fn line(i: &str) -> IResult<&str, Address> {
    let (i, v) = many1(tuple((string, tag("["), string, tag("]"))))(i)?;
    let (i, s) = string(i)?;
    let (i, _) = line_ending(i)?;
    let base = v
        .iter()
        .map(|(s, _, _h, _)| s)
        .cloned()
        .chain(once(s))
        .collect::<Vec<_>>();
    let hypernet = v.into_iter().map(|(_s, _, h, _)| h).collect::<Vec<_>>();
    Ok((
        i,
        Address {
            supernet: base,
            hypernet,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Address>> {
    many1(line)(i)
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
    // for address in &input {
    //     println!("{}", address);
    // }

    let result_a = input.iter().filter(|a| a.supports_tls()).count();

    let result_b = input.iter().filter(|a| a.supports_ssl()).count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
