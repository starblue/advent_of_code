use core::str::FromStr;

use std::fmt;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
struct Rule(i64, i64);
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}|{}", self.0, self.1)
    }
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (i, n0) = uint(i)?;
    let (i, _) = tag("|")(i)?;
    let (i, n1) = uint(i)?;
    Ok((i, Rule(n0, n1)))
}

#[derive(Clone, Debug)]
struct Update(Vec<i64>);
impl Update {
    fn misordered_indices1(&self, rule: &Rule) -> Option<(usize, usize)> {
        let mut right_index = None;
        for (i, n) in self.0.iter().enumerate() {
            if n == &rule.0 {
                if let Some(i1) = right_index {
                    return Some((i, i1));
                }
            }
            if n == &rule.1 {
                right_index = Some(i);
            }
        }
        None
    }
    fn misordered_indices(&self, rules: &[Rule]) -> Option<(usize, usize)> {
        rules
            .iter()
            .filter_map(|r| self.misordered_indices1(r))
            .next()
    }
    fn sort(&mut self, rules: &[Rule]) {
        while let Some((i0, i1)) = self.misordered_indices(rules) {
            self.0.swap(i0, i1);
        }
    }
    fn satisfies_rules(&self, rules: &[Rule]) -> bool {
        self.misordered_indices(rules).is_none()
    }
    fn middle_page_number(&self) -> i64 {
        let len = self.0.len();
        let i = len / 2;
        self.0[i]
    }
}
impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";
        for n in &self.0 {
            write!(f, "{}{}", sep, n)?;
            sep = ",";
        }
        Ok(())
    }
}

fn update(i: &str) -> IResult<&str, Update> {
    let (i, pages) = separated_list1(tag(","), uint)(i)?;
    Ok((i, Update(pages)))
}

#[derive(Clone, Debug)]
struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rule in &self.rules {
            writeln!(f, "{}", rule)?;
        }
        writeln!(f)?;
        for update in &self.updates {
            writeln!(f, "{}", update)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, rules) = separated_list1(line_ending, rule)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, updates) = separated_list1(line_ending, update)(i)?;
    Ok((i, Input { rules, updates }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    //println!("{}", input);

    let result1 = input
        .updates
        .iter()
        .filter(|update| update.satisfies_rules(&input.rules))
        .map(|update| update.middle_page_number())
        .sum::<i64>();

    let mut sum = 0;
    for update in &input.updates {
        if !update.satisfies_rules(&input.rules) {
            let mut sorted_update = update.clone();
            sorted_update.sort(&input.rules);
            sum += sorted_update.middle_page_number();
        }
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
