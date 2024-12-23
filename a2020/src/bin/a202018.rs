use core::str::FromStr;

use std::fmt;
use std::io;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}
impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Op::Add => '+',
            Op::Mul => '*',
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Num(i64),
    Paren(Rc<Expr>),
    BinOps(Rc<Expr>, Vec<(Op, Rc<Expr>)>),
}
impl Expr {
    fn from(e0: Expr, oes: Vec<(Op, Expr)>) -> Expr {
        if oes.is_empty() {
            e0
        } else {
            Expr::BinOps(
                Rc::new(e0),
                oes.into_iter()
                    .map(|(op, e1)| (op, Rc::new(e1)))
                    .collect::<Vec<_>>(),
            )
        }
    }
    fn eval1(&self) -> i64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Paren(e) => e.eval1(),
            Expr::BinOps(e0, oes) => {
                let mut v = e0.eval1();
                for (op, e1) in oes {
                    v = op.apply(v, e1.eval1());
                }
                v
            }
        }
    }
    fn eval2(&self) -> i64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Paren(e) => e.eval2(),
            Expr::BinOps(e0, oes) => {
                let mut sum = e0.eval2();
                let mut prod = 1;
                for (op, e1) in oes {
                    let v1 = e1.eval2();
                    match op {
                        Op::Add => {
                            sum += v1;
                        }
                        Op::Mul => {
                            prod *= sum;
                            sum = v1;
                        }
                    }
                }
                prod * sum
            }
        }
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Paren(e) => write!(f, "({})", e),
            Expr::BinOps(e0, oes) => {
                write!(f, "{}", e0)?;
                for (op, e1) in oes {
                    write!(f, " {} {}", op.to_char(), e1)?;
                }
                Ok(())
            }
        }
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn op(i: &str) -> IResult<&str, Op> {
    alt((value(Op::Add, char('+')), value(Op::Mul, char('*'))))(i)
}

fn prim_expr_paren(i: &str) -> IResult<&str, Expr> {
    let (i, _) = char('(')(i)?;
    let (i, e) = expr(i)?;
    let (i, _) = char(')')(i)?;
    Ok((i, Expr::Paren(Rc::new(e))))
}

fn prim_expr_num(i: &str) -> IResult<&str, Expr> {
    let (i, n) = int(i)?;
    Ok((i, Expr::Num(n)))
}

fn prim_expr(i: &str) -> IResult<&str, Expr> {
    alt((prim_expr_paren, prim_expr_num))(i)
}

fn op_expr(i: &str) -> IResult<&str, (Op, Expr)> {
    let (i, _) = tag(" ")(i)?;
    let (i, op) = op(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, e) = prim_expr(i)?;
    Ok((i, (op, e)))
}

fn expr(i: &str) -> IResult<&str, Expr> {
    let (i, e) = prim_expr(i)?;
    let (i, oes) = many0(op_expr)(i)?;
    Ok((i, Expr::from(e, oes)))
}

fn line(i: &str) -> IResult<&str, Expr> {
    let (i, e) = expr(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, e))
}

fn input(i: &str) -> IResult<&str, Vec<Expr>> {
    many1(line)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let es = result.unwrap().1;
    // for e in es {
    //     println!("{}", e);
    // }

    let result_a = es.iter().map(|e| e.eval1()).sum::<i64>();
    let result_b = es.iter().map(|e| e.eval2()).sum::<i64>();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::expr;
    use crate::Expr;

    #[test]
    fn test_expr_num() {
        let e = Expr::Num(42);
        assert_eq!(e, expr("42\n").unwrap().1);
    }

    #[test]
    fn test_expr_paren() {
        let e = Expr::Paren(Rc::new(Expr::Num(23)));
        assert_eq!(e, expr("(23)\n").unwrap().1);
    }

    #[test]
    fn test_eval1_sum2() {
        let e = expr("2 + 3\n").unwrap().1;
        assert_eq!(5, e.eval1());
    }
    #[test]
    fn test_eval1_sum3() {
        let e = expr("2 + 3 + 4\n").unwrap().1;
        assert_eq!(9, e.eval1());
    }

    #[test]
    fn test_eval1_mul2() {
        let e = expr("2 * 3\n").unwrap().1;
        assert_eq!(6, e.eval1());
    }
    #[test]
    fn test_eval1_mul3() {
        let e = expr("2 * 3 * 4\n").unwrap().1;
        assert_eq!(24, e.eval1());
    }

    #[test]
    fn test_eval1_mul_add() {
        let e = expr("2 * 3 + 4\n").unwrap().1;
        assert_eq!(10, e.eval1());
    }
    #[test]
    fn test_eval1_add_mul() {
        let e = expr("2 + 3 * 4\n").unwrap().1;
        assert_eq!(20, e.eval1());
    }

    #[test]
    fn test_eval1_26() {
        let e = expr("2 * 3 + (4 * 5)\n").unwrap().1;
        assert_eq!(26, e.eval1());
    }
    #[test]
    fn test_eval1_437() {
        let e = expr("5 + (8 * 3 + 9 + 3 * 4 * 3)\n").unwrap().1;
        assert_eq!(437, e.eval1());
    }
    #[test]
    fn test_eval1_12240() {
        let e = expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n")
            .unwrap()
            .1;
        assert_eq!(12240, e.eval1());
    }
    #[test]
    fn test_eval1_13632() {
        let e = expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2\n")
            .unwrap()
            .1;
        assert_eq!(13632, e.eval1());
    }

    #[test]
    fn test_eval2_46() {
        let e = expr("2 * 3 + (4 * 5)\n").unwrap().1;
        assert_eq!(46, e.eval2());
    }
    #[test]
    fn test_eval2_1445() {
        let e = expr("5 + (8 * 3 + 9 + 3 * 4 * 3)\n").unwrap().1;
        assert_eq!(1445, e.eval2());
    }
    #[test]
    fn test_eval2_669060() {
        let e = expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))\n")
            .unwrap()
            .1;
        assert_eq!(669060, e.eval2());
    }
    #[test]
    fn test_eval2_23340() {
        let e = expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2\n")
            .unwrap()
            .1;
        assert_eq!(23340, e.eval2());
    }
}
