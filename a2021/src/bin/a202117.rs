use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::IResult;

use lowdim::bb2d;
use lowdim::p2d;
use lowdim::v2d;
use lowdim::BBox2d;

fn input(i: &str) -> IResult<&str, BBox2d> {
    let (i, _) = tag("target area: x=")(i)?;
    let (i, x_min) = i64(i)?;
    let (i, _) = tag("..")(i)?;
    let (i, x_max) = i64(i)?;
    let (i, _) = tag(", y=")(i)?;
    let (i, y_min) = i64(i)?;
    let (i, _) = tag("..")(i)?;
    let (i, y_max) = i64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, bb2d(x_min..=x_max, y_min..=y_max)))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // println!(
    //     "target area: x={}..{}, y={}..{}",
    //     input.x_min(),
    //     input.x_max(),
    //     input.y_min(),
    //     input.y_max()
    // );

    let target_area = input;

    let mut global_max_y = 0;
    let mut count = 0;

    // For the first part the trangular number for the initial x velocity
    // must be in the targets x range, then drag slows the probe
    // so that it falls down vertically within that range.
    // This allows to maximize the vertical velocity giving the maximal height
    // for the first part.
    // But for the second part we need more,
    // for example a direct hit after the first step.
    // So search the complete ranges, it's fast enough.
    for x in 0..=target_area.x_max() {
        // When coming down again, the probe will be at y=0 and v_y = -v0_y.
        // So if the initial upward velocity is larger than the lower bound
        // of the target area, it will move past the target in the next step.
        let y_min_abs = target_area.y_min().abs();
        for y in -y_min_abs..=y_min_abs {
            let mut v = v2d(x, y);
            let mut p = p2d(0, 0);
            let mut max_y = 0;
            loop {
                p += v;
                v += v2d(-v.x().signum(), -1);

                max_y = max_y.max(p.y());
                if target_area.contains(&p) {
                    // The probe is in the target area.
                    global_max_y = global_max_y.max(max_y);
                    count += 1;
                    break;
                }
                if p.y() < target_area.y_min() {
                    // The probe is below the target area and cannot reach it any more.
                    break;
                }
            }
        }
    }
    let result_a = global_max_y;
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
