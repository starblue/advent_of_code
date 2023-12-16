use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}
impl Spring {
    fn to_char(self) -> char {
        match self {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        }
    }
}
impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct Row {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for spring in &self.springs {
            write!(f, "{}", spring)?;
        }
        write!(f, " ")?;
        let mut sep = "";
        for count in &self.counts {
            write!(f, "{}{}", sep, count)?;
            sep = ",";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    rows: Vec<Row>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.rows {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

fn spring(i: &str) -> IResult<&str, Spring> {
    alt((
        value(Spring::Operational, char('.')),
        value(Spring::Damaged, char('#')),
        value(Spring::Unknown, char('?')),
    ))(i)
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn numbers(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), uint)(i)
}

fn row(i: &str) -> IResult<&str, Row> {
    let (i, springs) = many1(spring)(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, counts) = numbers(i)?;
    Ok((i, Row { springs, counts }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    Ok((i, Input { rows }))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

fn compute_count(table: &mut HashMap<State, usize>, state: State) -> usize {
    if let Some(&c) = table.get(&state) {
        c
    } else if let Some(&spring) = state.springs.last() {
        let mut result = 0;

        let len = state.springs.len();
        if spring == Spring::Operational || spring == Spring::Unknown {
            // Assume last spring is operational.
            let springs = state.springs[..state.springs.len() - 1].to_vec();
            let counts = state.counts.clone();
            let new_state = State { springs, counts };
            result += compute_count(table, new_state);
        }
        if spring == Spring::Damaged || spring == Spring::Unknown {
            // Assume last spring is damaged.
            if let Some(&count) = state.counts.last() {
                if len >= count {
                    let damaged_start = len - count;
                    let (boundary, boundary_ok) = {
                        if damaged_start > 0 {
                            let boundary = damaged_start - 1;
                            (
                                boundary,
                                state.springs[boundary] == Spring::Operational
                                    || state.springs[boundary] == Spring::Unknown,
                            )
                        } else {
                            (0, true)
                        }
                    };
                    if boundary_ok
                        && state.springs[damaged_start..]
                            .iter()
                            .all(|&s| s == Spring::Damaged || s == Spring::Unknown)
                    {
                        let springs = state.springs[..boundary].to_vec();
                        let counts = state.counts[..(state.counts.len() - 1)].to_vec();
                        let new_state = State { springs, counts };
                        result += compute_count(table, new_state);
                    }
                }
            }
        }
        table.insert(state, result);
        result
    } else if state.counts.is_empty() {
        1
    } else {
        0
    }
}

fn unfold(row: &Row) -> Row {
    let mut springs = Vec::new();
    let mut sep = Vec::new();
    for _ in 0..5 {
        springs.extend(sep);
        springs.extend(&row.springs);
        sep = vec![Spring::Unknown];
    }
    let counts = row
        .counts
        .iter()
        .cycle()
        .take(5 * row.counts.len())
        .copied()
        .collect::<Vec<_>>();
    Row { springs, counts }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let mut sum = 0;
    for row in &input.rows {
        let state = State {
            springs: row.springs.clone(),
            counts: row.counts.clone(),
        };
        let mut table = HashMap::new();
        sum += compute_count(&mut table, state);
    }
    let result1 = sum;

    let mut sum = 0;
    for row in input.rows.into_iter().map(|r| unfold(&r)) {
        let state = State {
            springs: row.springs,
            counts: row.counts,
        };
        let mut table = HashMap::new();
        sum += compute_count(&mut table, state);
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
