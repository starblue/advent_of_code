use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

//use lowdim::p2d;
use lowdim::v2d;
use lowdim::Array2d;
use lowdim::Vec2d;
use lowdim::Vector;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn line(i: &str) -> IResult<&str, Vec<char>> {
    many1(satisfy(|c| c.is_ascii_uppercase()))(i)
}

fn input(i: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, line)(i)
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    let word_search: Array2d<i64, char> = Array2d::from_vec(input);
    let bbox = word_search.bbox();

    // for y in bbox.y_range() {
    //     for x in bbox.x_range() {
    //         print!("{}", word_search[p2d(x, y)]);
    //     }
    //     println!();
    // }
    // println!();

    let mut count = 0;
    for p in bbox {
        for v in Vec2d::unit_vecs_l_infty() {
            let mut found = true;
            for (i, c) in ['X', 'M', 'A', 'S'].iter().enumerate() {
                let i = i as i64;
                found = found && word_search.get(p + i * v) == Some(c);
            }
            if found {
                count += 1;
            }
        }
    }
    let result1 = count;

    let mut count = 0;
    for p in bbox {
        if word_search[p] == 'A' {
            let pmm = p + v2d(-1, -1);
            let pmp = p + v2d(-1, 1);
            let ppm = p + v2d(1, -1);
            let ppp = p + v2d(1, 1);
            if bbox.contains(&pmm)
                && bbox.contains(&pmp)
                && bbox.contains(&ppm)
                && bbox.contains(&ppp)
            {
                let cmm = word_search[pmm];
                let cmp = word_search[pmp];
                let cpm = word_search[ppm];
                let cpp = word_search[ppp];
                if ((cmm == 'M' && cpp == 'S') || (cpp == 'M' && cmm == 'S'))
                    && ((cmp == 'M' && cpm == 'S') || (cpm == 'M' && cmp == 'S'))
                {
                    count += 1;
                }
            }
        }
    }
    let result2 = count;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
