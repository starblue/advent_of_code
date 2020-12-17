use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::digit;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::tag;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Config {
    n_players: i32,
    last_marble: i32,
}

// 459 players; last marble is worth 71790 points

named!(int32<&str, i32>,
    map_res!(digit, FromStr::from_str)
);

named!(config<&str, Config>,
       do_parse!(
           n_players: int32
               >> tag!(" players; last marble is worth ")
               >> last_marble: int32
               >> tag!(" points")
               >> (Config{ n_players, last_marble })
       )
);

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    // parse line
    let result = config(&line.trim_end());
    //println!("{:?}", result);

    let config = result.unwrap().1;

    let n_players = config.n_players as usize;
    let last_marble = config.last_marble;

    let mut players = repeat(0).take(n_players).collect::<Vec<_>>();
    let mut current_player = 0;
    let mut marbles = vec![0];
    let mut current = 0;
    for m in 1..=last_marble {
        if m % 23 == 0 {
            current = (current + marbles.len() - 7) % marbles.len();
            players[current_player] += m + marbles.remove(current);
        } else {
            current = (current + 2) % marbles.len();
            marbles.insert(current, m);
        }
        current_player = (current_player + 1) % n_players;
    }

    let result = players.iter().max().unwrap();
    println!("{}", result);
}
