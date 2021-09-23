use core::str::FromStr;

use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Condition {
    min: i64,
    max: i64,
}
impl Condition {
    fn accepts(&self, n: i64) -> bool {
        self.min <= n && n <= self.max
    }
}
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    field: String,
    alt0: Condition,
    alt1: Condition,
}
impl Rule {
    fn may_have_value(&self, n: i64) -> bool {
        self.alt0.accepts(n) || self.alt1.accepts(n)
    }
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} or {}", self.field, self.alt0, self.alt1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ticket(Vec<i64>);
impl Ticket {
    fn completely_invalid(&self, rules: &[Rule]) -> bool {
        self.0
            .iter()
            .any(|&n| !rules.iter().any(|r| r.may_have_value(n)))
    }
    fn error_rate(&self, rules: &[Rule]) -> i64 {
        self.0
            .iter()
            .filter(|&&n| !rules.iter().any(|r| r.may_have_value(n)))
            .sum::<i64>()
    }
}
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for n in &self.0 {
            write!(f, "{}{}", sep, n)?;
            sep = ",";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    rules: Vec<Rule>,
    own_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}
impl Input {
    fn ticket_len(&self) -> usize {
        self.own_ticket.0.len()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in &self.rules {
            writeln!(f, "{}", r)?;
        }
        writeln!(f)?;
        writeln!(f, "your ticket:")?;
        writeln!(f, "{}", self.own_ticket)?;
        writeln!(f)?;
        writeln!(f, "nearby tickets:")?;
        for nt in &self.nearby_tickets {
            writeln!(f, "{}", nt)?;
        }
        Ok(())
    }
}

fn field(i: &str) -> IResult<&str, String> {
    map(recognize(many1(none_of(":"))), String::from)(i)
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn condition(i: &str) -> IResult<&str, Condition> {
    let (i, min) = int(i)?;
    let (i, _) = char('-')(i)?;
    let (i, max) = int(i)?;
    Ok((i, Condition { min, max }))
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (i, field) = field(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, alt0) = condition(i)?;
    let (i, _) = tag(" or ")(i)?;
    let (i, alt1) = condition(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Rule { field, alt0, alt1 }))
}

fn ticket(i: &str) -> IResult<&str, Ticket> {
    let (i, ns) = separated_list1(char(','), int)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Ticket(ns)))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, rules) = many1(rule)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("your ticket:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, own_ticket) = ticket(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("nearby tickets:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, nearby_tickets) = many1(ticket)(i)?;
    Ok((
        i,
        Input {
            rules,
            own_ticket,
            nearby_tickets,
        },
    ))
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

    let result_a = input
        .nearby_tickets
        .iter()
        .map(|t| t.error_rate(&input.rules))
        .sum::<i64>();

    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|t| !t.completely_invalid(&input.rules))
        .collect::<Vec<_>>();

    let mut possible_fields = (0..input.ticket_len())
        .map(|_| HashSet::new())
        .collect::<Vec<_>>();
    for rule in &input.rules {
        for i in 0..input.ticket_len() {
            if valid_tickets.iter().all(|t| rule.may_have_value(t.0[i])) {
                possible_fields[i].insert(&rule.field[..]);
            }
        }
    }

    let mut fields = (0..input.ticket_len()).map(|_| None).collect::<Vec<_>>();
    loop {
        let mut find = None;
        for i in 0..possible_fields.len() {
            if possible_fields[i].len() == 1 {
                // found unique possible field for index
                let f = possible_fields[i].iter().copied().next().unwrap();
                find = Some((i, f));
            }
        }
        if let Some((i, f)) = find {
            fields[i] = Some(f);
            for pf in &mut possible_fields {
                pf.remove(&f);
            }
        } else {
            break;
        }
    }
    let result_b = fields
        .iter()
        .enumerate()
        .filter_map(|(i, opt_f)| {
            if let Some(f) = opt_f {
                if f.starts_with("departure") {
                    Some(input.own_ticket.0[i])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .product::<i64>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::condition;
    use crate::rule;
    use crate::Condition;
    use crate::Rule;

    #[test]
    fn test_condition() {
        let c = Condition { min: 25, max: 80 };
        assert_eq!(c, condition("25-80 ").unwrap().1);
    }

    #[test]
    fn test_rule() {
        let r = Rule {
            field: "departure location".to_string(),
            alt0: Condition { min: 25, max: 80 },
            alt1: Condition { min: 90, max: 961 },
        };
        assert_eq!(r, rule("departure location: 25-80 or 90-961\n").unwrap().1);
    }
}
