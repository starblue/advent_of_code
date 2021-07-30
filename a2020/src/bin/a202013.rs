use core::mem::swap;

use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::alt;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::separated_list1;
use nom::tag;
use nom::value;

named!(
    int<&str, i128>,
    map_res!(digit1, FromStr::from_str)
);
named!(
    bus<&str, Option<i128>>,
    alt!(
        value!(None, tag!("x")) |
        do_parse!(
            n: int >> (Some(n))
        )
    )
);
named!(
    input<&str, (i128, Vec<Option<i128>>)>,
    do_parse!(
        earliest: int >>
        line_ending >>
        buses: separated_list1!(tag!(","), bus) >>
        line_ending >> ((earliest, buses))
    )
);

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
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

fn chinese_remainder_2(
    mr0: Option<(i128, i128)>,
    mr1: Option<(i128, i128)>,
) -> Option<(i128, i128)> {
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
fn chinese_remainder_n(mrs: &[Option<(i128, i128)>]) -> Option<(i128, i128)> {
    let mut mr = mrs[0];
    for mri in mrs.iter().skip(1) {
        mr = chinese_remainder_2(mr, *mri);
    }
    mr
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

    let (earliest, buses) = result.unwrap().1;
    //println!("{} {:?}", earliest, buses);

    let mut min_wait_time = std::i128::MAX;
    let mut first_bus = None;
    for b in &buses {
        if let Some(bus) = b {
            let wait_time = bus - 1 - ((earliest - 1) % bus);
            if wait_time < min_wait_time {
                min_wait_time = wait_time;
                first_bus = Some(bus);
            }
        }
    }
    let result_a = min_wait_time * first_bus.unwrap();

    let mrs = buses
        .into_iter()
        .enumerate()
        .filter_map(|(r, b)| {
            if let Some(bus) = b {
                Some(Some((bus, bus - r as i128)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let result_b = chinese_remainder_n(&mrs[..]).unwrap().1;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
