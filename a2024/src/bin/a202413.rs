use core::fmt;

use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::multi::many_m_n;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::v2d;
use lowdim::Vec2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
struct Machine {
    a: Vec2d,
    b: Vec2d,
    prize: Vec2d,
}
impl Machine {
    fn offset_prize(&self, offset: Vec2d) -> Machine {
        let a = self.a;
        let b = self.b;
        let prize = self.prize + offset;
        Machine { a, b, prize }
    }
    fn solve(&self) -> Option<(i64, i64)> {
        // (ax bx) (a)   (px)
        // (ay by) (b) = (py)

        // (a)         ( by -bx) (px)
        // (b) = 1/d * (-ay  ax) (py)

        let ax = self.a.x();
        let ay = self.a.y();
        let bx = self.b.x();
        let by = self.b.y();
        let px = self.prize.x();
        let py = self.prize.y();
        let d = ax * by - ay * bx;
        if d == 0 {
            panic!("a and b are collinear");
        } else {
            let a_num = by * px - bx * py;
            let b_num = -ay * px + ax * py;
            if a_num % d == 0 && b_num % d == 0 {
                let a = a_num / d;
                let b = b_num / d;
                Some((a, b))
            } else {
                None
            }
        }
    }
}
impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Button A: X+{}, Y+{}", self.a.x(), self.a.y())?;
        writeln!(f, "Button B: X+{}, Y+{}", self.b.x(), self.b.y())?;
        writeln!(f, "Prize: X={}, Y={}", self.prize.x(), self.prize.y())
    }
}

fn machine(i: &str) -> IResult<&str, Machine> {
    let (i, _) = tag("Button A: X+")(i)?;
    let (i, x) = i64(i)?;
    let (i, _) = tag(", Y+")(i)?;
    let (i, y) = i64(i)?;
    let a = v2d(x, y);

    let (i, _) = line_ending(i)?;

    let (i, _) = tag("Button B: X+")(i)?;
    let (i, x) = i64(i)?;
    let (i, _) = tag(", Y+")(i)?;
    let (i, y) = i64(i)?;
    let b = v2d(x, y);

    let (i, _) = line_ending(i)?;

    let (i, _) = tag("Prize: X=")(i)?;
    let (i, x) = i64(i)?;
    let (i, _) = tag(", Y=")(i)?;
    let (i, y) = i64(i)?;
    let prize = v2d(x, y);

    Ok((i, Machine { a, b, prize }))
}

fn input(i: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(many_m_n(2, 2, line_ending), machine)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for machine in &input {
    //     println!("{}", machine);
    // }
    // println!();

    let result1 = input
        .iter()
        .filter_map(|m| m.solve().map(|(a, b)| 3 * a + b))
        .sum::<i64>();

    let offset = 10_000_000_000_000_i64 * v2d(1, 1);
    let result2 = input
        .iter()
        .filter_map(|m| m.offset_prize(offset).solve().map(|(a, b)| 3 * a + b))
        .sum::<i64>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
