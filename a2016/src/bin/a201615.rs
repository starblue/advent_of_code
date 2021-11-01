use core::mem::swap;

use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Disc {
    number: i64,
    positions: i64,
    initial_position: i64,
}
impl fmt::Display for Disc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Disc #{} has {} positions; at time=0, it is at position {}.",
            self.number, self.positions, self.initial_position
        )
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn disc(i: &str) -> IResult<&str, Disc> {
    let (i, _) = tag("Disc #")(i)?;
    let (i, number) = int(i)?;
    let (i, _) = tag(" has ")(i)?;
    let (i, positions) = int(i)?;
    let (i, _) = tag(" positions; at time=0, it is at position ")(i)?;
    let (i, initial_position) = int(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Disc {
            number,
            positions,
            initial_position,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Disc>> {
    many1(disc)(i)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 && b == 0 {
        (0, 0, 0)
    } else {
        let mut a = a;
        let mut sa = 1;
        let mut ta = 0;
        let mut b = b;
        let mut sb = 0;
        let mut tb = 1;
        if a < b {
            swap(&mut a, &mut b);
            swap(&mut sa, &mut sb);
            swap(&mut ta, &mut tb);
        }
        while b > 0 {
            let d = a / b;
            let r = a % b; // r == a - d * b
            let s_new = sa - d * sb;
            let t_new = ta - d * tb;
            a = b;
            sa = sb;
            ta = tb;
            b = r;
            sb = s_new;
            tb = t_new;
        }
        (a, sa, ta)
    }
}

fn chinese_remainder_2(mr0: Option<(i64, i64)>, mr1: Option<(i64, i64)>) -> Option<(i64, i64)> {
    if let (Some((m0, r0)), Some((m1, r1))) = (mr0, mr1) {
        let (gcd, s, t) = egcd(m0, m1);
        if r0 % gcd != 0 || r1 % gcd != 0 {
            // no solution
            None
        } else {
            let m = m0 / gcd * m1;
            let r = (r0 * m1 * t + r1 * m0 * s).rem_euclid(m);
            Some((m, r))
        }
    } else {
        // propagate case of no solution from arguments
        None
    }
}
fn chinese_remainder_n(mrs: &[Option<(i64, i64)>]) -> Option<(i64, i64)> {
    let mut mr = mrs[0];
    for mri in mrs.iter().skip(1) {
        mr = chinese_remainder_2(mr, *mri);
    }
    mr
}

fn disc_to_mr(disc: &Disc) -> Option<(i64, i64)> {
    // At time t + disc.number + disc.initial_position
    // the disc must be in position 0.
    // So t + n + i = 0 (mod positions), or equivalently
    // t = -n - i (mod positions).
    Some((disc.positions, -disc.number - disc.initial_position))
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
    // for disc in &input {
    //     println!("{}", disc);
    // }

    let mut mrs = input.iter().map(disc_to_mr).collect::<Vec<_>>();
    let (_m, r) = chinese_remainder_n(&mrs).expect("no solution");
    let result_a = r;

    let last_disc = input.iter().last().unwrap();
    let number = last_disc.number + 1;
    let new_disc = Disc {
        number,
        positions: 11,
        initial_position: 0,
    };
    mrs.push(disc_to_mr(&new_disc));
    let (_m, r) = chinese_remainder_n(&mrs).expect("no solution");
    let result_b = r;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
