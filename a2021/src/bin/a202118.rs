use core::cell::RefCell;
use core::fmt;

use std::io;
use std::io::Read;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq)]
enum InnerNumber {
    Regular(i64),
    Pair(Number, Number),
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Number(Rc<RefCell<InnerNumber>>);
impl Number {
    fn regular(n: i64) -> Number {
        Number(Rc::new(RefCell::new(InnerNumber::Regular(n))))
    }
    fn pair(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(InnerNumber::Pair(self, other))))
    }
    fn deep_clone(&self) -> Number {
        match &*self.0.borrow_mut() {
            InnerNumber::Regular(n) => Number::regular(*n),
            InnerNumber::Pair(n0, n1) => n0.deep_clone().pair(n1.deep_clone()),
        }
    }

    fn add(self, other: Number) -> Number {
        let mut n = self.pair(other);
        while n.reduce() {}
        n
    }
    fn reduce(&mut self) -> bool {
        self.try_explode() || self.try_split()
    }
    fn try_explode(&mut self) -> bool {
        self.try_explode_inner(None, None, 0)
    }
    fn try_explode_inner(
        &mut self,
        left: Option<&mut Number>,
        right: Option<&mut Number>,
        depth: usize,
    ) -> bool {
        match &mut *self.0.borrow_mut() {
            InnerNumber::Regular(_) => false,
            pair @ InnerNumber::Pair(_, _) => {
                if let InnerNumber::Pair(n0, n1) = pair {
                    if depth >= 4 {
                        let n0 = n0.magnitude();
                        let n1 = n1.magnitude();
                        if let Some(left) = left {
                            left.inc_rightmost_regular(n0);
                        }
                        if let Some(right) = right {
                            right.inc_leftmost_regular(n1);
                        }
                        *pair = InnerNumber::Regular(0);
                        true
                    } else {
                        n0.try_explode_inner(left, Some(n1), depth + 1)
                            || n1.try_explode_inner(Some(n0), right, depth + 1)
                    }
                } else {
                    panic!("internal error: expected pair");
                }
            }
        }
    }
    fn inc_leftmost_regular(&mut self, delta: i64) {
        match &mut *self.0.borrow_mut() {
            InnerNumber::Regular(n) => *n += delta,
            InnerNumber::Pair(n0, _n1) => n0.inc_leftmost_regular(delta),
        }
    }
    fn inc_rightmost_regular(&mut self, delta: i64) {
        match &mut *self.0.borrow_mut() {
            InnerNumber::Regular(n) => *n += delta,
            InnerNumber::Pair(_n0, n1) => n1.inc_rightmost_regular(delta),
        }
    }

    fn try_split(&mut self) -> bool {
        match &mut *self.0.borrow_mut() {
            number @ InnerNumber::Regular(_) => {
                if let InnerNumber::Regular(n) = &number {
                    if *n >= 10 {
                        let n0 = *n / 2;
                        let n1 = *n - n0;
                        let n0 = Number::regular(n0);
                        let n1 = Number::regular(n1);
                        *number = InnerNumber::Pair(n0, n1);
                        true
                    } else {
                        false
                    }
                } else {
                    panic!("internal error: expected regular number");
                }
            }
            InnerNumber::Pair(n0, n1) => n0.try_split() || n1.try_split(),
        }
    }

    fn magnitude(&self) -> i64 {
        match &*self.0.borrow() {
            InnerNumber::Regular(n) => *n,
            InnerNumber::Pair(n0, n1) => 3 * n0.magnitude() + 2 * n1.magnitude(),
        }
    }
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &*self.0.borrow() {
            InnerNumber::Regular(n) => write!(f, "{}", n),
            InnerNumber::Pair(n0, n1) => write!(f, "[{},{}]", n0, n1),
        }
    }
}

fn number_regular(i: &str) -> IResult<&str, Number> {
    let (i, n) = i64(i)?;
    Ok((i, Number::regular(n)))
}
fn number_pair(i: &str) -> IResult<&str, Number> {
    let (i, _) = tag("[")(i)?;
    let (i, left) = number(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, right) = number(i)?;
    let (i, _) = tag("]")(i)?;

    Ok((i, left.pair(right)))
}
fn number(i: &str) -> IResult<&str, Number> {
    alt((number_regular, number_pair))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Number>> {
    separated_list1(line_ending, number)(i)
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
    // for number in &input {
    //     println!("{}", number);
    // }

    let mut iter = input.iter();
    let mut sum = iter.next().unwrap().deep_clone();
    for n in iter {
        sum = sum.add(n.deep_clone());
    }
    let result_a = sum.magnitude();

    let mut max_magnitude = 0;
    for ni in &input {
        for nj in &input {
            if ni != nj {
                let ni = ni.deep_clone();
                let nj = nj.deep_clone();
                let sum = ni.add(nj);
                let magnitude = sum.magnitude();
                max_magnitude = max_magnitude.max(magnitude);
            }
        }
    }
    let result_b = max_magnitude;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::number;

    #[test]
    fn test_add_0() {
        let n0 = number("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap().1;
        let n1 = number("[1,1]").unwrap().1;
        let n2 = number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().1;
        assert_eq!(n2, n0.add(n1));
    }

    #[test]
    fn test_try_explode_0() {
        let n0 = number("[[[[[9,8],1],2],3],4]").unwrap().1;
        let n1 = number("[[[[0,9],2],3],4]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_explode());
        assert_eq!(n1, n);
    }
    #[test]
    fn test_try_explode_1() {
        let n0 = number("[7,[6,[5,[4,[3,2]]]]]").unwrap().1;
        let n1 = number("[7,[6,[5,[7,0]]]]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_explode());
        assert_eq!(n1, n);
    }
    #[test]
    fn test_try_explode_2() {
        let n0 = number("[[6,[5,[4,[3,2]]]],1]").unwrap().1;
        let n1 = number("[[6,[5,[7,0]]],3]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_explode());
        assert_eq!(n1, n);
    }
    #[test]
    fn test_try_explode_3() {
        let n0 = number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap().1;
        let n1 = number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_explode());
        assert_eq!(n1, n);
    }
    #[test]
    fn test_try_explode_4() {
        let n0 = number("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap().1;
        let n1 = number("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_explode());
        assert_eq!(n1, n);
    }

    #[test]
    fn test_try_split_10() {
        let n0 = number("10").unwrap().1;
        let n1 = number("[5,5]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_split());
        assert_eq!(n1, n);
    }
    #[test]
    fn test_try_split_11() {
        let n0 = number("11").unwrap().1;
        let n1 = number("[5,6]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_split());
        assert_eq!(n1, n);
    }
    #[test]
    fn test_try_split_12() {
        let n0 = number("12").unwrap().1;
        let n1 = number("[6,6]").unwrap().1;
        let mut n = n0.clone();
        assert_eq!(true, n.try_split());
        assert_eq!(n1, n);
    }

    #[test]
    fn test_magnitude_0() {
        let n = number("[[1,2],[[3,4],5]]").unwrap().1;
        assert_eq!(143, n.magnitude());
    }
    #[test]
    fn test_magnitude_1() {
        let n = number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap().1;
        assert_eq!(1384, n.magnitude());
    }
    #[test]
    fn test_magnitude_2() {
        let n = number("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap().1;
        assert_eq!(445, n.magnitude());
    }
    #[test]
    fn test_magnitude_3() {
        let n = number("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap().1;
        assert_eq!(791, n.magnitude());
    }
    #[test]
    fn test_magnitude_4() {
        let n = number("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap().1;
        assert_eq!(1137, n.magnitude());
    }
    #[test]
    fn test_magnitude_5() {
        let n = number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .unwrap()
            .1;
        assert_eq!(3488, n.magnitude());
    }
}
