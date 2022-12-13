use core::fmt;
use core::str::FromStr;

use std::error;
use std::io;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::IResult;

use util::runtime_error;

#[derive(Clone, Debug)]
enum Expr {
    Int(i64),
    List(Vec<Expr>),
}
impl Expr {
    fn to_list(&self) -> Expr {
        Expr::List(vec![self.clone()])
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(i) => write!(f, "{}", i),
            Expr::List(v) => {
                write!(f, "[")?;
                let mut sep = "";
                for e in v {
                    write!(f, "{}{}", sep, e)?;
                    sep = ",";
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}
impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        match (self, other) {
            (Expr::Int(n0), Expr::Int(n1)) => n0 == n1,
            (Expr::List(v0), Expr::List(v1)) => v0 == v1,
            (Expr::Int(_), Expr::List(_)) => &self.to_list() == other,
            (Expr::List(_), Expr::Int(_)) => self == &other.to_list(),
        }
    }
}
impl Eq for Expr {}
impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Expr) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Expr::Int(n0), Expr::Int(n1)) => n0.cmp(n1),
            (Expr::List(v0), Expr::List(v1)) => v0.cmp(v1),
            (Expr::Int(_), Expr::List(_)) => self.to_list().cmp(other),
            (Expr::List(_), Expr::Int(_)) => self.cmp(&other.to_list()),
        }
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn expr_int(i: &str) -> IResult<&str, Expr> {
    let (i, n) = int(i)?;
    Ok((i, Expr::Int(n)))
}
fn expr_list(i: &str) -> IResult<&str, Expr> {
    let (i, _) = char('[')(i)?;
    let (i, v) = separated_list0(char(','), expr)(i)?;
    let (i, _) = char(']')(i)?;
    Ok((i, Expr::List(v)))
}
fn expr(i: &str) -> IResult<&str, Expr> {
    alt((expr_int, expr_list))(i)
}

fn pair(i: &str) -> IResult<&str, (Expr, Expr)> {
    let (i, e0) = expr(i)?;
    let (i, _) = line_ending(i)?;
    let (i, e1) = expr(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, (e0, e1)))
}

fn input(i: &str) -> IResult<&str, Vec<(Expr, Expr)>> {
    separated_list0(line_ending, pair)(i)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for (e0, e1) in &input {
    //     println!("{}", e0);
    //     println!("{}", e1);
    //     println!();
    // }

    let mut sum = 0;
    for (i, (e0, e1)) in input.iter().enumerate() {
        let right = e0 <= e1;
        if right {
            // Add one because indices are one-based.
            sum += i + 1;
        }
    }
    let result1 = sum;

    let div0 = expr("[[2]]").map_err(|e| e.to_owned())?.1;
    let div1 = expr("[[6]]").map_err(|e| e.to_owned())?.1;
    let mut packets = vec![div0.clone(), div1.clone()];
    for (p0, p1) in input {
        packets.push(p0);
        packets.push(p1);
    }
    packets.sort();
    let pos0 = 1 + packets
        .iter()
        .position(|p| p == &div0)
        .ok_or(runtime_error!("packet not found"))?;
    let pos1 = 1 + packets
        .iter()
        .position(|p| p == &div1)
        .ok_or(runtime_error!("packet not found"))?;
    let result2 = pos0 * pos1;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
