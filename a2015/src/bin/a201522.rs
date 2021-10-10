use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::IResult;

#[derive(Clone, Debug)]
struct Input {
    hit_points: i64,
    damage: i64,
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, _) = tag("Hit Points: ")(i)?;
    let (i, hit_points) = uint(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Damage: ")(i)?;
    let (i, damage) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Input { hit_points, damage }))
}

#[derive(Clone, Debug)]
struct State {
    hard: bool,
    turn: i64,
    player_turn: bool,
    player_hit_points: i64,
    player_mana: i64,
    boss_hit_points: i64,
    boss_damage: i64,
    shield_turns: i64,
    poison_turns: i64,
    recharge_turns: i64,
    mana_spent: i64,
}
impl State {
    fn start(input: &Input, hard: bool) -> State {
        State {
            hard,
            turn: 1,
            player_turn: true,
            player_hit_points: 50,
            player_mana: 500,
            boss_hit_points: input.hit_points,
            boss_damage: input.damage,
            shield_turns: 0,
            poison_turns: 0,
            recharge_turns: 0,
            mana_spent: 0,
        }
    }

    /// returns the minimum mana spent for a player win from this state,
    /// or `None` if the boss wins.
    fn min_win_mana(&self, min_mana: i64) -> i64 {
        if self.boss_hit_points <= 0 {
            self.mana_spent
        } else if self.player_hit_points <= 0 {
            std::i64::MAX
        } else {
            let mut after_effect_state = self.clone();

            after_effect_state.turn += 1;
            after_effect_state.player_turn = !self.player_turn;

            if self.hard {
                after_effect_state.player_hit_points -= 1;
                if self.player_hit_points <= 0 {
                    return std::i64::MAX;
                }
            }

            // apply effects
            let mut player_armor = 0;
            if self.shield_turns > 0 {
                player_armor += 7;
                after_effect_state.shield_turns -= 1;
            }
            if self.poison_turns > 0 {
                after_effect_state.boss_hit_points -= 3;
                after_effect_state.poison_turns -= 1;
                if after_effect_state.boss_hit_points <= 0 {
                    return after_effect_state.mana_spent;
                }
            }
            if self.recharge_turns > 0 {
                after_effect_state.player_mana += 101;
                after_effect_state.recharge_turns -= 1;
            }

            if self.player_turn {
                // try all possible moves

                let mut min_mana = min_mana;
                if after_effect_state.player_mana >= 53 {
                    let mut next_state = after_effect_state.clone();
                    next_state.player_mana -= 53;
                    next_state.mana_spent += 53;
                    if next_state.mana_spent < min_mana {
                        next_state.boss_hit_points -= 4;
                        min_mana = min_mana.min(next_state.min_win_mana(min_mana));
                    }
                }
                if after_effect_state.player_mana >= 73 {
                    let mut next_state = after_effect_state.clone();
                    next_state.player_mana -= 73;
                    next_state.mana_spent += 73;
                    if next_state.mana_spent < min_mana {
                        next_state.boss_hit_points -= 2;
                        next_state.player_hit_points += 2;
                        min_mana = min_mana.min(next_state.min_win_mana(min_mana));
                    }
                }
                if after_effect_state.player_mana >= 113 && after_effect_state.shield_turns == 0 {
                    let mut next_state = after_effect_state.clone();
                    next_state.player_mana -= 113;
                    next_state.mana_spent += 113;
                    if next_state.mana_spent < min_mana {
                        next_state.shield_turns = 6;
                        min_mana = min_mana.min(next_state.min_win_mana(min_mana));
                    }
                }
                if after_effect_state.player_mana >= 173 && after_effect_state.poison_turns == 0 {
                    let mut next_state = after_effect_state.clone();
                    next_state.player_mana -= 173;
                    next_state.mana_spent += 173;
                    if next_state.mana_spent < min_mana {
                        next_state.poison_turns = 6;
                        min_mana = min_mana.min(next_state.min_win_mana(min_mana));
                    }
                }
                if after_effect_state.player_mana >= 229 && after_effect_state.recharge_turns == 0 {
                    let mut next_state = after_effect_state;
                    next_state.player_mana -= 229;
                    next_state.mana_spent += 229;
                    if next_state.mana_spent < min_mana {
                        next_state.recharge_turns = 5;
                        min_mana = min_mana.min(next_state.min_win_mana(min_mana));
                    }
                }
                min_mana
            } else {
                // boss turn
                let damage = (self.boss_damage - player_armor).max(1);
                after_effect_state.player_hit_points -= damage;
                after_effect_state.min_win_mana(min_mana)
            }
        }
    }
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
    // println!("{:?}", input);

    let state = State::start(&input, false);
    let result_a = state.min_win_mana(std::i64::MAX);

    let state = State::start(&input, true);
    let result_b = state.min_win_mana(std::i64::MAX);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
