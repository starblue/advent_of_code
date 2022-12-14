use core::fmt;
use core::str::FromStr;

use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::character::complete::space0;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use util::runtime_error;

#[derive(Clone, Copy, Debug)]
struct Crate {
    name: char,
}
impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.name)
    }
}

#[derive(Clone, Copy, Debug)]
struct Step {
    quantity: usize,
    from: usize,
    to: usize,
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.quantity, self.from, self.to
        )
    }
}

#[derive(Clone, Debug)]
struct Stacks {
    stacks: Vec<Vec<Crate>>,
}
impl Stacks {
    fn from_rows(rows: Vec<Vec<Option<Crate>>>) -> util::Result<Stacks> {
        let len = rows
            .iter()
            .map(|r| r.len())
            .max()
            .ok_or_else(|| runtime_error!("diagram is empty"))?;
        let mut stacks = (0..len).map(|_| Vec::new()).collect::<Vec<_>>();
        for row in rows {
            for (i, opt_crate) in row.iter().enumerate() {
                if let Some(c) = opt_crate {
                    stacks[i].push(*c);
                }
            }
        }
        for stack in stacks.iter_mut() {
            stack.reverse();
        }
        Ok(Stacks { stacks })
    }
    fn by_rows(&self) -> util::Result<Vec<Vec<Option<Crate>>>> {
        let mut rows = Vec::new();
        let mut i = self.max_height()? - 1;
        loop {
            rows.push(self.stacks.iter().map(|s| s.get(i).cloned()).collect());
            if i == 0 {
                break;
            }
            i -= 1;
        }
        Ok(rows)
    }
    fn max_height(&self) -> util::Result<usize> {
        self.stacks
            .iter()
            .map(|s| s.len())
            .max()
            .ok_or_else(|| runtime_error!("diagram is empty"))
    }
    fn move_crate(&mut self, from: usize, to: usize) -> util::Result<()> {
        if let Some(c) = self.stacks[from - 1].pop() {
            self.stacks[to - 1].push(c);
        } else {
            return Err(runtime_error!("stack is empty"));
        }
        Ok(())
    }
    fn execute1(&mut self, step: &Step) -> util::Result<()> {
        let Step { quantity, from, to } = step;
        for _ in 0..*quantity {
            self.move_crate(*from, *to)?;
        }
        Ok(())
    }
    fn execute2(&mut self, step: &Step) -> util::Result<()> {
        let Step { quantity, from, to } = step;
        let stack0 = &mut self.stacks[from - 1];
        let mut moved_crates = stack0.split_off(stack0.len() - quantity);
        self.stacks[to - 1].append(&mut moved_crates);
        Ok(())
    }
    fn tops(&self) -> Vec<Crate> {
        self.stacks.iter().flat_map(|s| s.last()).copied().collect()
    }
}
impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows = self.by_rows().map_err(|_| fmt::Error)?;
        for row in rows {
            let mut sep = "";
            for opt in row {
                if let Some(c) = opt {
                    write!(f, "{}{}", sep, c)?;
                } else {
                    write!(f, "{}   ", sep)?;
                }
                sep = " ";
            }
            writeln!(f)?;
        }
        let mut sep = "";
        for i in 1..=self.stacks.len() {
            write!(f, "{} {} ", sep, i)?;
            sep = " ";
        }
        writeln!(f)
    }
}

#[derive(Clone, Debug)]
struct Input {
    stacks: Stacks,
    steps: Vec<Step>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.stacks)?;
        for step in &self.steps {
            writeln!(f, "{}", step)?;
        }
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}

fn opt_crate_some(i: &str) -> IResult<&str, Option<Crate>> {
    let (i, _) = char('[')(i)?;
    let (i, c) = satisfy(char::is_alphabetic)(i)?;
    let (i, _) = char(']')(i)?;
    Ok((i, Some(Crate { name: c })))
}

fn opt_crate(i: &str) -> IResult<&str, Option<Crate>> {
    alt((opt_crate_some, value(None, tag("   "))))(i)
}

fn row(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(char(' '), opt_crate)(i)
}

fn stack_name(i: &str) -> IResult<&str, usize> {
    let (i, _) = space0(i)?;
    let (i, n) = int(i)?;
    Ok((i, n))
}

fn stack_names(i: &str) -> IResult<&str, Vec<usize>> {
    many1(stack_name)(i)
}

fn stacks_drawing(i: &str) -> IResult<&str, Stacks> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = stack_names(i)?;
    let (i, _) = space0(i)?;
    let (i, _) = line_ending(i)?;
    let stacks = Stacks::from_rows(rows).unwrap();
    Ok((i, stacks))
}

fn step(i: &str) -> IResult<&str, Step> {
    let (i, _) = tag("move ")(i)?;
    let (i, quantity) = int(i)?;
    let (i, _) = tag(" from ")(i)?;
    let (i, from) = int(i)?;
    let (i, _) = tag(" to ")(i)?;
    let (i, to) = int(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Step { quantity, from, to }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, stacks) = stacks_drawing(i)?;
    let (i, _) = line_ending(i)?;
    let (i, steps) = many0(step)(i)?;
    Ok((i, Input { stacks, steps }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    //println!("{}", input);

    let mut stacks = input.stacks.clone();
    for step in &input.steps {
        stacks.execute1(step)?;
    }
    let result1 = stacks.tops().iter().map(|c| c.name).collect::<String>();

    let mut stacks = input.stacks.clone();
    for step in &input.steps {
        stacks.execute2(step)?;
    }
    let result2 = stacks.tops().iter().map(|c| c.name).collect::<String>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
