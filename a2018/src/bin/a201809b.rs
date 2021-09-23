use std::collections::VecDeque;
use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Config {
    n_players: i64,
    last_marble: i64,
}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn config(i: &str) -> IResult<&str, Config> {
    let (i, n_players) = int64(i)?;
    let (i, _) = tag(" players; last marble is worth ")(i)?;
    let (i, last_marble) = int64(i)?;
    let (i, _) = tag(" points")(i)?;
    Ok((
        i,
        Config {
            n_players,
            last_marble,
        },
    ))
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    // parse line
    let result = config(line.trim_end());
    //println!("{:?}", result);

    let config = result.unwrap().1;

    let n_players = config.n_players as usize;
    let last_marble = (config.last_marble as i64) * 100;

    let mut players = repeat(0_i64).take(n_players).collect::<Vec<_>>();
    let mut current_player = 0;
    let mut marbles = VecDeque::new();
    marbles.push_front(0);

    // the current marble is the first in the linked list
    for m in 1..=last_marble {
        if m % 23 == 0 {
            for _ in 0..7 {
                let m1 = marbles.pop_back().unwrap();
                marbles.push_front(m1);
            }
            players[current_player] += m + marbles.pop_front().unwrap();
        } else {
            for _ in 0..2 {
                let m1 = marbles.pop_front().unwrap();
                marbles.push_back(m1);
            }
            marbles.push_front(m);
        }
        current_player = (current_player + 1) % n_players;
    }

    let result = players.iter().max().unwrap();
    println!("{}", result);
}
