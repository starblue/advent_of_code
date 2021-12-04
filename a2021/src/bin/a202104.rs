use core::fmt;
use core::ops;
use core::str::FromStr;

use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;
use lowdim::Point2d;

#[derive(Clone, Debug)]
struct Board(Array2d<i32, i64>);
impl Board {
    fn bbox(&self) -> BBox2d<i32> {
        self.0.bbox()
    }
}
impl ops::Index<Point2d<i32>> for Board {
    type Output = i64;
    fn index(&self, p: Point2d<i32>) -> &i64 {
        &self.0[p]
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for y in self.bbox().y_range() {
            let mut sep = "";
            for x in self.bbox().x_range() {
                write!(f, "{}{:2}", sep, self[p2d(x, y)])?;
                sep = " ";
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    draws: Vec<i64>,
    boards: Vec<Board>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut sep = "";
        for d in &self.draws {
            write!(f, "{}{}", sep, d)?;
            sep = ",";
        }
        writeln!(f)?;
        writeln!(f)?;
        for b in &self.boards {
            writeln!(f, "{}", b)?;
        }
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn draws(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), int)(i)
}

fn number(i: &str) -> IResult<&str, i64> {
    let (i, _) = space0(i)?;
    let (i, n) = int(i)?;
    Ok((i, n))
}

fn row(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, number)(i)
}

fn board(i: &str) -> IResult<&str, Board> {
    let (i, rows) = separated_list1(line_ending, row)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Board(Array2d::from_vec(rows))))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, draws) = draws(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, boards) = separated_list1(line_ending, board)(i)?;
    Ok((i, Input { draws, boards }))
}

fn play_bingo(board: &Board, draws: &[i64]) -> Option<(usize, i64)> {
    let bbox = board.bbox();
    let mut moves = 0;
    let mut marked = Array2d::new(bbox, false);
    for &d in draws {
        moves += 1;

        for p in bbox.iter() {
            if board[p] == d {
                marked[p] = true;
            }
        }
        if bbox
            .y_range()
            .any(|y| bbox.x_range().all(|x| marked[p2d(x, y)]))
            || bbox
                .x_range()
                .any(|x| bbox.y_range().all(|y| marked[p2d(x, y)]))
        {
            // Bingo!
            let unmarked_sum = bbox
                .iter()
                .filter(|&p| !marked[p])
                .map(|p| board[p])
                .sum::<i64>();
            let score = unmarked_sum * d;
            return Some((moves, score));
        }
    }
    None
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
    // println!("{}", input);

    let mut min_moves = usize::MAX;
    let mut min_moves_score = 0;
    for b in &input.boards {
        if let Some((moves, score)) = play_bingo(b, &input.draws) {
            if moves < min_moves {
                min_moves = moves;
                min_moves_score = score;
            }
        }
    }
    let result_a = min_moves_score;

    let mut max_moves = usize::MIN;
    let mut max_moves_score = 0;
    for b in &input.boards {
        if let Some((moves, score)) = play_bingo(b, &input.draws) {
            if moves > max_moves {
                max_moves = moves;
                max_moves_score = score;
            }
        }
    }
    let result_b = max_moves_score;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
