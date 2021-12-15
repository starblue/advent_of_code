use core::fmt;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug)]
struct Rule {
    left0: char,
    left1: char,
    right: char,
}
impl Rule {}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{} -> {}", self.left0, self.left1, self.right)
    }
}

#[derive(Clone, Debug)]
struct Input {
    start: Vec<char>,
    rules: Vec<Rule>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for c in &self.start {
            write!(f, "{}", c)?;
        }
        writeln!(f)?;
        writeln!(f)?;
        for i in &self.rules {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

fn element(i: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

fn elements(i: &str) -> IResult<&str, Vec<char>> {
    many1(element)(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (i, left0) = element(i)?;
    let (i, left1) = element(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, right) = element(i)?;
    Ok((
        i,
        Rule {
            left0,
            left1,
            right,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, start) = elements(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, rules) = separated_list1(line_ending, rule)(i)?;
    Ok((i, Input { start, rules }))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let rules = input
        .rules
        .iter()
        .map(|rule| ((rule.left0, rule.left1), rule.right))
        .collect::<HashMap<_, _>>();

    let mut template = input.start.clone();
    for _ in 0..10 {
        let mut new_template = vec![template[0]];
        for w in template.windows(2) {
            if let &[e0, e1] = w {
                if let Some(&e) = rules.get(&(e0, e1)) {
                    new_template.push(e);
                }
                new_template.push(e1);
            }
        }
        template = new_template;
    }
    let mut counts = HashMap::new();
    for e in template {
        let entry = counts.entry(e).or_insert(0);
        *entry += 1;
    }
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    let result_a = max - min;

    // The second part becomes very large, so compute the counts directly.

    // Each element is counted twice, once for its predecessor
    // and once for its successor.
    // If it can react with the other element it is counted in reactive pairs.
    // Otherwise it is counted as inert w.r.t that neighbor.
    // Similarly, the first and last element are counted once as inert
    // for being at the start and the end.
    // In particular, an element that can't react on either side
    // is counted twice as inert.
    let mut reactive_pair_counts = HashMap::new();
    let mut inert_counts = HashMap::new();

    // Initialize counts

    // Count first element as inert.
    let e1 = input.start[0];
    let entry = inert_counts.entry(e1).or_insert(0_usize);
    *entry += 1;

    // Count elements in pairs
    for w in input.start.windows(2) {
        if let &[e0, e1] = w {
            if rules.contains_key(&(e0, e1)) {
                let entry = reactive_pair_counts.entry((e0, e1)).or_insert(0);
                *entry += 1;
            } else {
                let entry = inert_counts.entry(e0).or_insert(0);
                *entry += 1;
                let entry = inert_counts.entry(e1).or_insert(0);
                *entry += 1;
            }
        }
    }
    // Count last element as inert.
    let e0 = input.start[input.start.len() - 1];
    let entry = inert_counts.entry(e0).or_insert(0);
    *entry += 1;

    // Run reactions
    for _ in 0..40 {
        let mut new_reactive_pair_counts = HashMap::new();
        let mut new_inert_counts = inert_counts.clone();

        for (&(e0, e1), count) in &reactive_pair_counts {
            let e = rules[&(e0, e1)];

            if rules.contains_key(&(e0, e)) {
                let entry = new_reactive_pair_counts.entry((e0, e)).or_insert(0);
                *entry += count;
            } else {
                let entry = new_inert_counts.entry(e0).or_insert(0);
                *entry += count;
                let entry = new_inert_counts.entry(e).or_insert(0);
                *entry += count;
            }

            if rules.contains_key(&(e, e1)) {
                let entry = new_reactive_pair_counts.entry((e, e1)).or_insert(0);
                *entry += count;
            } else {
                let entry = new_inert_counts.entry(e).or_insert(0);
                *entry += count;
                let entry = new_inert_counts.entry(e1).or_insert(0);
                *entry += count;
            }
        }
        reactive_pair_counts = new_reactive_pair_counts;
        inert_counts = new_inert_counts;
    }

    // Sum up all counts for the final result.
    let mut double_counts = inert_counts;
    for (&(e0, e1), count) in &reactive_pair_counts {
        let entry = double_counts.entry(e0).or_insert(0);
        *entry += count;
        let entry = double_counts.entry(e1).or_insert(0);
        *entry += count;
    }

    let max = double_counts.values().max().unwrap() / 2;
    let min = double_counts.values().min().unwrap() / 2;
    let result_b = max - min;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
