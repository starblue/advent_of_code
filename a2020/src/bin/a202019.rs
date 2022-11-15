use core::str::FromStr;

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::none_of;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Expr {
    Lit(char),
    Rule(usize),
    Seq(Rc<Expr>, Rc<Expr>),
    Alt(Rc<Expr>, Rc<Expr>),
}
impl Expr {
    fn from_sequence(seq: Vec<Expr>) -> Expr {
        assert!(!seq.is_empty());
        let mut iter = seq.into_iter();
        let mut e0 = iter.next().unwrap();
        for e1 in iter {
            e0 = Expr::Seq(Rc::new(e0), Rc::new(e1));
        }
        e0
    }
    fn from_alternatives(alts: Vec<Expr>) -> Expr {
        assert!(!alts.is_empty());
        let mut iter = alts.into_iter();
        let mut e0 = iter.next().unwrap();
        for e1 in iter {
            e0 = Expr::Alt(Rc::new(e0), Rc::new(e1));
        }
        e0
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Lit(c) => write!(f, "\"{}\"", c),
            Expr::Rule(n) => write!(f, "{}", n),
            Expr::Seq(e0, e1) => write!(f, "{} {}", e0, e1),
            Expr::Alt(e0, e1) => write!(f, "{} | {}", e0, e1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rule {
    number: usize,
    body: Expr,
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.number, self.body)
    }
}

#[derive(Clone, Debug)]
struct Input {
    rules: Vec<Rule>,
    messages: Vec<String>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in &self.rules {
            writeln!(f, "{}", r)?;
        }
        writeln!(f)?;
        for m in &self.messages {
            writeln!(f, "{}", m)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Parser(HashMap<usize, Rc<Expr>>);
impl Parser {
    fn from(rules: Vec<Rule>) -> Parser {
        Parser(
            rules
                .into_iter()
                .map(|r| (r.number, Rc::new(r.body)))
                .collect::<HashMap<_, _>>(),
        )
    }
    fn matches(&self, rule_number: usize, msg: &str) -> bool {
        let mut table = HashMap::new();
        let msg = msg.chars().collect::<Vec<_>>();
        let start = 0;
        let end = msg.len();
        self.parses_as_rule(&mut table, &msg, rule_number, start, end)
    }
    fn parses_as_rule(
        &self,
        table: &mut HashMap<(Rc<Expr>, usize, usize), bool>,
        msg: &[char],
        rule_number: usize,
        start: usize,
        end: usize,
    ) -> bool {
        let expr = self.get_rule(rule_number);
        self.parses_as_expr(table, msg, expr, start, end)
    }
    fn parses_as_expr(
        &self,
        table: &mut HashMap<(Rc<Expr>, usize, usize), bool>,
        msg: &[char],
        expr: Rc<Expr>,
        start: usize,
        end: usize,
    ) -> bool {
        if let Some(&result) = table.get(&(expr.clone(), start, end)) {
            result
        } else {
            let result = match &*expr {
                Expr::Lit(c) => end - start == 1 && msg[start] == *c,
                Expr::Rule(n) => self.parses_as_rule(table, msg, *n, start, end),
                Expr::Seq(e0, e1) => ((start + 1)..end).any(|mid| {
                    self.parses_as_expr(table, msg, e0.clone(), start, mid)
                        && self.parses_as_expr(table, msg, e1.clone(), mid, end)
                }),
                Expr::Alt(e0, e1) => {
                    self.parses_as_expr(table, msg, e0.clone(), start, end)
                        || self.parses_as_expr(table, msg, e1.clone(), start, end)
                }
            };
            table.insert((expr, start, end), result);
            result
        }
    }
    fn get_rule(&self, rule_number: usize) -> Rc<Expr> {
        self.0[&rule_number].clone()
    }
}

fn int(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}

fn literal(i: &str) -> IResult<&str, Expr> {
    let (i, _) = char('\"')(i)?;
    let (i, c) = none_of("\"")(i)?;
    let (i, _) = char('\"')(i)?;
    Ok((i, Expr::Lit(c)))
}

fn sequence(i: &str) -> IResult<&str, Expr> {
    let (i, seq) = separated_list1(tag(" "), int)(i)?;
    Ok((
        i,
        Expr::from_sequence(seq.into_iter().map(|n| Expr::Rule(n)).collect::<Vec<_>>()),
    ))
}

fn alternatives(i: &str) -> IResult<&str, Expr> {
    let (i, alts) = separated_list1(tag(" | "), sequence)(i)?;
    Ok((i, Expr::from_alternatives(alts)))
}

fn body(i: &str) -> IResult<&str, Expr> {
    alt((literal, alternatives))(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    let (i, number) = int(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, body) = body(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Rule { number, body }))
}

fn message(i: &str) -> IResult<&str, String> {
    let (i, msg) = alpha1(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, String::from(msg)))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, rules) = many1(rule)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, messages) = many1(message)(i)?;
    Ok((i, Input { rules, messages }))
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

    let parser_a = Parser::from(input.rules.clone());
    let result_a = input
        .messages
        .iter()
        .filter(|msg| parser_a.matches(0, msg))
        .count();

    let parser_b = Parser::from(
        input
            .rules
            .into_iter()
            .map(|r| {
                if r.number == 8 {
                    rule("8: 42 | 42 8\n").unwrap().1
                } else if r.number == 11 {
                    rule("11: 42 31 | 42 11 31\n").unwrap().1
                } else {
                    r
                }
            })
            .collect::<Vec<_>>(),
    );
    let result_b = input
        .messages
        .iter()
        .filter(|msg| parser_b.matches(0, msg))
        .count();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
