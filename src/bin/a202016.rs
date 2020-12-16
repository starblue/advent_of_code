use core::str::FromStr;

use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Read;

use nom::char;
use nom::digit;
use nom::do_parse;
use nom::line_ending;
use nom::many1;
use nom::map;
use nom::map_res;
use nom::named;
use nom::none_of;
use nom::recognize;
use nom::separated_nonempty_list;
use nom::tag;

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

named!(field<&str, String>,
    map!(recognize!(many1!(none_of!(":"))), String::from)
);
named!(int<&str, i64>,
    map_res!(digit, FromStr::from_str)
);
named!(condition<&str, Condition>,
    do_parse!(
        min: int >>
        char!('-') >>
        max: int >>
            (Condition { min, max })
    )
);
named!(rule<&str, Rule>,
    do_parse!(
        field: field >>
        tag!(": ") >>
        alt0: condition >>
        tag!(" or ") >>
        alt1: condition >>
        line_ending >>
            (Rule { field, alt0, alt1 })
    )
);
named!(ticket<&str, Ticket>,
    do_parse!(
        ns: separated_nonempty_list!(char!(','), int) >>
        line_ending >>
            (Ticket(ns))
    )
);
named!(input<&str, Input>,
    do_parse!(
        rules: many1!(rule) >>
        line_ending >>
        tag!("your ticket:") >>
        line_ending >>
        own_ticket: ticket >>
        line_ending >>
        tag!("nearby tickets:") >>
        line_ending >>
        nearby_tickets: many1!(ticket) >>
            (Input { rules, own_ticket, nearby_tickets })
    )
);

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
