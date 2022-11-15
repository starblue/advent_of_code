use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vein {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
}

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn vein_x(i: &str) -> IResult<&str, Vein> {
    let (i, _) = tag("x=")(i)?;
    let (i, x) = int64(i)?;
    let (i, _) = tag(", ")(i)?;
    let (i, _) = tag("y=")(i)?;
    let (i, y0) = int64(i)?;
    let (i, _) = tag("..")(i)?;
    let (i, y1) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Vein {
            x0: x,
            x1: x,
            y0,
            y1,
        },
    ))
}

fn vein_y(i: &str) -> IResult<&str, Vein> {
    let (i, _) = tag("y=")(i)?;
    let (i, y) = int64(i)?;
    let (i, _) = tag(", ")(i)?;
    let (i, _) = tag("x=")(i)?;
    let (i, x0) = int64(i)?;
    let (i, _) = tag("..")(i)?;
    let (i, x1) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Vein {
            x0,
            x1,
            y0: y,
            y1: y,
        },
    ))
}

fn vein(i: &str) -> IResult<&str, Vein> {
    alt((vein_x, vein_y))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vein>> {
    many1(vein)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let veins = result.unwrap().1;
    let x_min = veins.iter().map(|v| v.x0).min().unwrap();
    let x_max = veins.iter().map(|v| v.x1).max().unwrap();
    let y_min = veins.iter().map(|v| v.y0).min().unwrap();
    let y_max = veins.iter().map(|v| v.y1).max().unwrap();

    // add borders to avoid range checking everywhere
    let x_base = x_min - 2;
    let y_base = 0;
    let x_size = x_max - x_min + 5;
    let y_size = y_max + 1;

    let mut map = repeat(repeat('.').take(x_size as usize).collect::<Vec<_>>())
        .take(y_size as usize)
        .collect::<Vec<_>>();
    map[(0 - y_base) as usize][(500 - x_base) as usize] = '+';
    for v in veins {
        let Vein { x0, x1, y0, y1 } = v;
        let x0 = (x0 - x_base) as usize;
        let x1 = (x1 - x_base) as usize;
        let y0 = (y0 - y_base) as usize;
        let y1 = (y1 - y_base) as usize;
        for y in y0..=y1 {
            for x in x0..=x1 {
                map[y][x] = '#';
            }
        }
    }

    let mut open = vec![(500, 0)];
    while let Some((x0, y0)) = open.pop() {
        // consider position below
        let x1 = x0;
        let y1 = y0 + 1;
        if y1 <= y_max {
            let c1 = map[(y1 - y_base) as usize][(x1 - x_base) as usize];
            match c1 {
                '.' => {
                    // sand, water flows down
                    map[(y1 - y_base) as usize][(x1 - x_base) as usize] = '|';
                    open.push((x1, y1));
                }
                '#' | '~' => {
                    // clay or standing water, try flow sideways instead

                    let x_left = x0 - 1;
                    let y_left = y0;
                    let c_left = map[(y_left - y_base) as usize][(x_left - x_base) as usize];
                    match c_left {
                        '.' => {
                            // sand, water flows left
                            map[(y_left - y_base) as usize][(x_left - x_base) as usize] = '|';
                            open.push((x_left, y_left));
                        }
                        '#' => {
                            // clay, water is blocked

                            // check if flowing water reaches clay on other side
                            let mut xt = x0;
                            while map[(y0 - y_base) as usize][(xt - x_base) as usize] == '|' {
                                xt += 1;
                            }
                            if map[(y0 - y_base) as usize][(xt - x_base) as usize] == '#' {
                                // we have standing water
                                for x in x0..=(xt - 1) {
                                    map[(y0 - y_base) as usize][(x - x_base) as usize] = '~';
                                    // reconsider positions above now standing water
                                    if map[(y0 - 1 - y_base) as usize][(x - x_base) as usize] == '|'
                                    {
                                        open.push((x, y0 - 1));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    let x_right = x0 + 1;
                    let y_right = y0;
                    let c_right = map[(y_right - y_base) as usize][(x_right - x_base) as usize];
                    match c_right {
                        '.' => {
                            // sand, water flows right
                            map[(y_right - y_base) as usize][(x_right - x_base) as usize] = '|';
                            open.push((x_right, y_right));
                        }
                        '#' => {
                            // clay, water is blocked

                            // check if flowing water reaches clay on other side
                            let mut xt = x0;
                            while map[(y0 - y_base) as usize][(xt - x_base) as usize] == '|' {
                                xt -= 1;
                            }
                            if map[(y0 - y_base) as usize][(xt - x_base) as usize] == '#' {
                                // we have standing water
                                for x in (xt + 1)..=x0 {
                                    map[(y0 - y_base) as usize][(x - x_base) as usize] = '~';
                                    // reconsider positions above now standing water
                                    if map[(y0 - 1 - y_base) as usize][(x - x_base) as usize] == '|'
                                    {
                                        open.push((x, y0 - 1));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    // for y in y_base..(y_base + y_size) {
    //     for x in x_base..(x_base + x_size) {
    //         let c = map[(y - y_base) as usize][(x - x_base) as usize];
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let mut count_flowing = 0;
    let mut count_standing = 0;
    for y in y_min..=y_max {
        for x in x_base..(x_base + x_size) {
            let c = map[(y - y_base) as usize][(x - x_base) as usize];
            match c {
                '|' => {
                    count_flowing += 1;
                }
                '~' => {
                    count_standing += 1;
                }
                _ => {}
            }
        }
    }

    println!("a: {}", count_flowing + count_standing);
    println!("b: {}", count_standing);
}

#[cfg(test)]
mod test {
    use crate::vein;
    use crate::Vein;

    #[test]
    fn test_vein() {
        assert_eq!(
            vein("x=507, y=1652..1666\n"),
            Ok((
                "",
                Vein {
                    x0: 507,
                    x1: 507,
                    y0: 1652,
                    y1: 1666
                }
            ))
        );
        assert_eq!(
            vein("y=650, x=583..593\n"),
            Ok((
                "",
                Vein {
                    x0: 583,
                    x1: 593,
                    y0: 650,
                    y1: 650
                }
            ))
        );
    }
}
