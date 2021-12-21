use core::fmt;

use std::collections::HashMap;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::IResult;

#[derive(Clone, Debug)]
struct Input {
    start1: i64,
    start2: i64,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Player 1 starting position: {}", self.start1)?;
        writeln!(f, "Player 2 starting position: {}", self.start2)?;
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("Player 1 starting position: ")(i)?;
    let (i, start1) = i64(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Player 2 starting position: ")(i)?;
    let (i, start2) = i64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Input { start1, start2 }))
}

#[derive(Clone, Debug)]
struct Die {
    next_value: i64,
    roll_count: i64,
}
impl Die {
    fn new() -> Die {
        Die {
            next_value: 1,
            roll_count: 0,
        }
    }
    fn roll(&mut self) -> i64 {
        self.roll_count += 1;
        let result = self.next_value;
        self.next_value += 1;
        if self.next_value > 100 {
            self.next_value = 1;
        }
        result
    }
    fn roll_count(&self) -> i64 {
        self.roll_count
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Player {
    space: i64,
    score: i64,
}
impl Player {
    fn new(start: i64) -> Player {
        Player {
            space: start,
            score: 0,
        }
    }
    fn roll(&self, roll: usize) -> Player {
        let roll = i64::try_from(roll).unwrap();
        let space = (self.space + roll - 1) % 10 + 1;
        let score = self.score + space;
        Player { space, score }
    }
    fn has_won(&self) -> bool {
        self.score >= 21
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Turn {
    Player1,
    Player2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    turn: Turn,
    player1: Player,
    player2: Player,
}
impl State {
    fn new(input: &Input) -> State {
        let player1 = Player::new(input.start1);
        let player2 = Player::new(input.start2);
        State {
            turn: Turn::Player1,
            player1,
            player2,
        }
    }
    fn roll(&self, roll: usize) -> State {
        match self.turn {
            Turn::Player1 => {
                let new_player1 = self.player1.roll(roll);
                State {
                    turn: Turn::Player2,
                    player1: new_player1,
                    player2: self.player2.clone(),
                }
            }
            Turn::Player2 => {
                let new_player2 = self.player2.roll(roll);
                State {
                    turn: Turn::Player1,
                    player1: self.player1.clone(),
                    player2: new_player2,
                }
            }
        }
    }
    fn player1_has_won(&self) -> bool {
        self.player1.has_won()
    }
    fn player2_has_won(&self) -> bool {
        self.player2.has_won()
    }
}

const ROLL_COUNTS: &[(usize, i64)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn wins(table: &mut HashMap<State, (i64, i64)>, state: &State) -> (i64, i64) {
    if let Some(&result) = table.get(state) {
        result
    } else {
        let mut player1_wins = 0;
        let mut player2_wins = 0;
        for &(roll, count) in ROLL_COUNTS.iter() {
            let new_state = state.roll(roll);
            if new_state.player1_has_won() {
                player1_wins += count;
            } else if new_state.player2_has_won() {
                player2_wins += count;
            } else {
                let (wins1, wins2) = wins(table, &new_state);
                player1_wins += count * wins1;
                player2_wins += count * wins2;
            }
        }
        let result = (player1_wins, player2_wins);
        table.insert(state.clone(), result);
        result
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // println!("{}", input);

    let mut die = Die::new();
    let mut space1 = input.start1;
    let mut space2 = input.start2;
    let mut score1 = 0;
    let mut score2 = 0;
    loop {
        space1 = (space1 + die.roll() + die.roll() + die.roll() - 1) % 10 + 1;
        score1 += space1;
        if score1 >= 1000 {
            break;
        }
        space2 = (space2 + die.roll() + die.roll() + die.roll() - 1) % 10 + 1;
        score2 += space2;
        if score2 >= 1000 {
            break;
        }
    }
    let losing_score = score1.min(score2);
    let result_a = losing_score * die.roll_count();

    let state = State::new(&input);
    let mut table = HashMap::new();
    let (wins1, wins2) = wins(&mut table, &state);
    let result_b = wins1.max(wins2);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
