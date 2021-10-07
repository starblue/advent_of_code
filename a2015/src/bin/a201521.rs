use std::io;
use std::io::Read;
use std::iter::once;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::IResult;

#[derive(Clone, Debug)]
struct Item {
    #[allow(unused)]
    name: &'static str,
    cost: i64,
    damage: i64,
    armor: i64,
}

const WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: [Item; 5] = [
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Clone, Debug)]
struct Character {
    hit_points: i64,
    damage: i64,
    armor: i64,
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, Character> {
    let (i, _) = tag("Hit Points: ")(i)?;
    let (i, hit_points) = uint(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Damage: ")(i)?;
    let (i, damage) = uint(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Armor: ")(i)?;
    let (i, armor) = uint(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Character {
            hit_points,
            damage,
            armor,
        },
    ))
}

fn player_wins(player: &Character, boss: &Character) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();
    loop {
        // player attacks
        let damage = (player.damage - boss.armor).max(1);
        boss.hit_points -= damage;
        if boss.hit_points <= 0 {
            return true;
        }
        // boss attacks
        let damage = (boss.damage - player.armor).max(1);
        player.hit_points -= damage;
        if player.hit_points <= 0 {
            return false;
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
    println!("{:?}", input);

    let boss = input;

    // try all allowed combinations of items
    let mut min_cost = std::i64::MAX;
    for w in WEAPONS {
        for opt_a in once(None).chain(ARMOR.iter().map(Some)) {
            for b in 0..(1 << RINGS.len()) {
                let mut items = Vec::new();
                items.push(w.clone());
                if let Some(armor) = opt_a {
                    items.push(armor.clone());
                }
                let mut ring_count = 0;
                for i in 0..RINGS.len() {
                    if b & (1 << i) != 0 {
                        items.push(RINGS[i].clone());
                        ring_count += 1;
                    }
                }
                if ring_count <= 2 {
                    let cost = items.iter().map(|item| item.cost).sum();
                    let damage = items.iter().map(|item| item.damage).sum();
                    let armor = items.iter().map(|item| item.armor).sum();
                    let player = Character {
                        hit_points: 100,
                        damage,
                        armor,
                    };
                    if player_wins(&player, &boss) {
                        min_cost = min_cost.min(cost);
                    }
                }
            }
        }
    }
    let result_a = min_cost;

    // try all allowed combinations of items
    let mut max_cost = 0;
    for w in WEAPONS {
        for opt_a in once(None).chain(ARMOR.iter().map(Some)) {
            for b in 0..(1 << RINGS.len()) {
                let mut items = Vec::new();
                items.push(w.clone());
                if let Some(armor) = opt_a {
                    items.push(armor.clone());
                }
                let mut ring_count = 0;
                for i in 0..RINGS.len() {
                    if b & (1 << i) != 0 {
                        items.push(RINGS[i].clone());
                        ring_count += 1;
                    }
                }
                if ring_count <= 2 {
                    let cost = items.iter().map(|item| item.cost).sum();
                    let damage = items.iter().map(|item| item.damage).sum();
                    let armor = items.iter().map(|item| item.armor).sum();
                    let player = Character {
                        hit_points: 100,
                        damage,
                        armor,
                    };
                    if !player_wins(&player, &boss) {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
    }
    let result_b = max_cost;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
