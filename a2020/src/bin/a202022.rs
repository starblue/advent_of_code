use core::str::FromStr;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Deck {
    cards: VecDeque<usize>,
}
impl Deck {
    fn card_count(&self) -> usize {
        self.cards.len()
    }
    fn has_no_cards(&self) -> bool {
        self.cards.is_empty()
    }
    fn draw_top_card(&mut self) -> usize {
        self.cards.pop_front().expect("no more cards")
    }
    fn add_bottom_card(&mut self, card: usize) {
        self.cards.push_back(card)
    }
    fn new_deck(&self, n: usize) -> Deck {
        let cards = self.cards.iter().take(n).cloned().collect::<VecDeque<_>>();
        Deck { cards }
    }
    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, c)| (i + 1) * c)
            .sum::<usize>()
    }
}
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in &self.cards {
            writeln!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GamePosition {
    deck1: Deck,
    deck2: Deck,
}
impl GamePosition {
    fn draw_top_cards(&mut self) -> (usize, usize) {
        let card1 = self.deck1.draw_top_card();
        let card2 = self.deck2.draw_top_card();
        (card1, card2)
    }
    fn winning_score(&self) -> usize {
        if self.deck1.has_no_cards() {
            self.deck2.score()
        } else if self.deck2.has_no_cards() {
            self.deck1.score()
        } else {
            panic!("game has not ended yet")
        }
    }
    fn subgame(&self, c1: usize, c2: usize) -> GamePosition {
        let deck1 = self.deck1.new_deck(c1);
        let deck2 = self.deck2.new_deck(c2);
        GamePosition { deck1, deck2 }
    }
}
impl fmt::Display for GamePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Player 1:")?;
        writeln!(f, "{}", self.deck1)?;
        writeln!(f, "Player 2:")?;
        writeln!(f, "{}", self.deck2)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum Outcome {
    Player1Wins,
    Player2Wins,
}

#[derive(Clone, Debug)]
struct Game {
    position: GamePosition,
    previous_rounds: HashSet<GamePosition>,
}
impl Game {
    fn new(p: &GamePosition) -> Game {
        Game {
            position: p.clone(),
            previous_rounds: HashSet::new(),
        }
    }
    fn play_round_a(&mut self) -> Option<Outcome> {
        if self.position.deck1.has_no_cards() {
            Some(Outcome::Player2Wins)
        } else if self.position.deck2.has_no_cards() {
            Some(Outcome::Player1Wins)
        } else {
            let (c1, c2) = self.position.draw_top_cards();
            let deck1 = &mut self.position.deck1;
            let deck2 = &mut self.position.deck2;
            if c1 > c2 {
                // player 1 won
                deck1.add_bottom_card(c1);
                deck1.add_bottom_card(c2);
            } else if c2 > c1 {
                // player 2 won
                deck2.add_bottom_card(c2);
                deck2.add_bottom_card(c1);
            } else {
                panic!("two equal cards, cheating detected")
            }

            // continue the game for now, check for win at beginning of next round
            None
        }
    }
    fn play_a(&mut self) -> Outcome {
        loop {
            if let Some(outcome) = self.play_round_a() {
                return outcome;
            }
        }
    }
    fn play_round_b(&mut self) -> Option<Outcome> {
        if self.position.deck1.has_no_cards() {
            Some(Outcome::Player2Wins)
        } else if self.position.deck2.has_no_cards() {
            Some(Outcome::Player1Wins)
        } else if self.previous_rounds.contains(&self.position) {
            Some(Outcome::Player1Wins)
        } else {
            self.previous_rounds.insert(self.position.clone());
            let (c1, c2) = self.position.draw_top_cards();
            let round_outcome = {
                if c1 <= self.position.deck1.card_count() && c2 <= self.position.deck2.card_count()
                {
                    // play recursive game
                    let mut subgame = self.subgame(c1, c2);
                    subgame.play_b()
                } else if c1 > c2 {
                    Outcome::Player1Wins
                } else if c2 > c1 {
                    Outcome::Player2Wins
                } else {
                    panic!("two equal cards, cheating detected")
                }
            };
            let deck1 = &mut self.position.deck1;
            let deck2 = &mut self.position.deck2;
            match round_outcome {
                Outcome::Player1Wins => {
                    deck1.add_bottom_card(c1);
                    deck1.add_bottom_card(c2);
                }
                Outcome::Player2Wins => {
                    deck2.add_bottom_card(c2);
                    deck2.add_bottom_card(c1);
                }
            }

            // continue the game for now, check for win at beginning of next round
            None
        }
    }
    fn subgame(&self, c1: usize, c2: usize) -> Game {
        let position = self.position.subgame(c1, c2);
        Game {
            position,
            previous_rounds: HashSet::new(),
        }
    }
    fn play_b(&mut self) -> Outcome {
        loop {
            if let Some(outcome) = self.play_round_b() {
                return outcome;
            }
        }
    }
    fn winning_score(&self) -> usize {
        self.position.winning_score()
    }
}

fn int(i: &str) -> IResult<&str, usize> {
    map_res(digit1, FromStr::from_str)(i)
}
fn deck(i: &str) -> IResult<&str, Deck> {
    let (i, cards) = separated_list1(line_ending, int)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Deck {
            cards: cards.into(),
        },
    ))
}

fn game_position(i: &str) -> IResult<&str, GamePosition> {
    let (i, _) = tag("Player 1:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, deck1) = deck(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Player 2:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, deck2) = deck(i)?;
    Ok((i, GamePosition { deck1, deck2 }))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = game_position(&input_data);
    //println!("{:?}", result);

    let game_position = result.unwrap().1;
    //println!("{}", game_position);

    let mut game_a = Game::new(&game_position);
    game_a.play_a();
    let result_a = game_a.winning_score();
    println!("a: {}", result_a);

    let mut game_b = Game::new(&game_position);
    game_b.play_b();
    let result_b = game_b.winning_score();
    println!("b: {}", result_b);
}
