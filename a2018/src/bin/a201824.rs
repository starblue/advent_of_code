use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::iter::repeat;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Group {
    side: Side,
    n: i64,
    hit_points: i64,
    attack_damage: i64,
    attack_type: AttackType,
    initiative: i64,
    weaknesses: HashSet<AttackType>,
    immunities: HashSet<AttackType>,
}

impl Group {
    fn is_enemy(&self, other: &Group) -> bool {
        self.side != other.side
    }
    fn effective_power(&self) -> i64 {
        self.n * self.attack_damage
    }
    fn damage(&self, attacked: &Group) -> i64 {
        let ep = self.effective_power();
        if attacked.weaknesses.contains(&self.attack_type) {
            2 * ep
        } else if attacked.immunities.contains(&self.attack_type) {
            0
        } else {
            ep
        }
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        other
            .effective_power()
            .partial_cmp(&self.effective_power())
            .and_then(|ordering| match ordering {
                Ordering::Equal => other.initiative.partial_cmp(&self.initiative),
                ordering => Some(ordering),
            })
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Group) -> Ordering {
        other
            .effective_power()
            .cmp(&self.effective_power())
            .then(other.initiative.cmp(&self.initiative))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SelectionPriority {
    damage: i64,
    effective_power: i64,
    initiative: i64,
}

impl PartialOrd for SelectionPriority {
    fn partial_cmp(&self, other: &SelectionPriority) -> Option<Ordering> {
        self.damage
            .partial_cmp(&other.damage)
            .and_then(|ordering| match ordering {
                Ordering::Equal => self.effective_power.partial_cmp(&other.effective_power),
                ordering => Some(ordering),
            })
            .and_then(|ordering| match ordering {
                Ordering::Equal => self.initiative.partial_cmp(&other.initiative),
                ordering => Some(ordering),
            })
    }
}

impl Ord for SelectionPriority {
    fn cmp(&self, other: &SelectionPriority) -> Ordering {
        self.damage
            .cmp(&other.damage)
            .then(self.effective_power.cmp(&other.effective_power))
            .then(self.initiative.cmp(&other.initiative))
    }
}

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn attack_type(i: &str) -> IResult<&str, AttackType> {
    alt((
        value(AttackType::Bludgeoning, tag("bludgeoning")),
        value(AttackType::Cold, tag("cold")),
        value(AttackType::Fire, tag("fire")),
        value(AttackType::Radiation, tag("radiation")),
        value(AttackType::Slashing, tag("slashing")),
    ))(i)
}

fn attack_types(i: &str) -> IResult<&str, HashSet<AttackType>> {
    let (i, attack_types) = separated_list1(tag(", "), attack_type)(i)?;
    Ok((i, attack_types.into_iter().collect::<HashSet<_>>()))
}

fn weaknesses(i: &str) -> IResult<&str, HashSet<AttackType>> {
    let (i, _) = tag("weak to ")(i)?;
    let (i, attack_types) = attack_types(i)?;
    Ok((i, attack_types))
}

fn immunities(i: &str) -> IResult<&str, HashSet<AttackType>> {
    let (i, _) = tag("immune to ")(i)?;
    let (i, attack_types) = attack_types(i)?;
    Ok((i, attack_types))
}

fn opt_weaknesses_immunities_clause_w(
    i: &str,
) -> IResult<&str, (HashSet<AttackType>, HashSet<AttackType>)> {
    let (i, _) = tag("(")(i)?;
    let (i, weaknesses) = weaknesses(i)?;
    let (i, _) = tag(") ")(i)?;
    Ok((i, (weaknesses, HashSet::new())))
}

fn opt_weaknesses_immunities_clause_i(
    i: &str,
) -> IResult<&str, (HashSet<AttackType>, HashSet<AttackType>)> {
    let (i, _) = tag("(")(i)?;
    let (i, immunities) = immunities(i)?;
    let (i, _) = tag(") ")(i)?;
    Ok((i, (HashSet::new(), immunities)))
}

fn opt_weaknesses_immunities_clause_wi(
    i: &str,
) -> IResult<&str, (HashSet<AttackType>, HashSet<AttackType>)> {
    let (i, _) = tag("(")(i)?;
    let (i, weaknesses) = weaknesses(i)?;
    let (i, _) = tag("; ")(i)?;
    let (i, immunities) = immunities(i)?;
    let (i, _) = tag(") ")(i)?;
    Ok((i, (weaknesses, immunities)))
}

fn opt_weaknesses_immunities_clause_iw(
    i: &str,
) -> IResult<&str, (HashSet<AttackType>, HashSet<AttackType>)> {
    let (i, _) = tag("(")(i)?;
    let (i, immunities) = immunities(i)?;
    let (i, _) = tag("; ")(i)?;
    let (i, weaknesses) = weaknesses(i)?;
    let (i, _) = tag(") ")(i)?;
    Ok((i, (weaknesses, immunities)))
}

fn opt_weaknesses_immunities_clause_none(
    i: &str,
) -> IResult<&str, (HashSet<AttackType>, HashSet<AttackType>)> {
    value((HashSet::new(), HashSet::new()), tag(""))(i)
}

fn opt_weaknesses_immunities_clause(
    i: &str,
) -> IResult<&str, (HashSet<AttackType>, HashSet<AttackType>)> {
    alt((
        opt_weaknesses_immunities_clause_w,
        opt_weaknesses_immunities_clause_i,
        opt_weaknesses_immunities_clause_wi,
        opt_weaknesses_immunities_clause_iw,
        opt_weaknesses_immunities_clause_none,
    ))(i)
}

fn group(side: Side, i: &str) -> IResult<&str, Group> {
    let (i, n) = int64(i)?;
    let (i, _) = tag(" units each with ")(i)?;
    let (i, hit_points) = int64(i)?;
    let (i, _) = tag(" hit points ")(i)?;
    let (i, wais) = opt_weaknesses_immunities_clause(i)?;
    let (i, _) = tag("with an attack that does ")(i)?;
    let (i, attack_damage) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, attack_type) = attack_type(i)?;
    let (i, _) = tag(" damage at initiative ")(i)?;
    let (i, initiative) = int64(i)?;
    Ok((
        i,
        Group {
            side,
            n,
            hit_points,
            attack_damage,
            attack_type,
            initiative,
            weaknesses: wais.0,
            immunities: wais.1,
        },
    ))
}

fn army<'a>(side: Side, i: &'a str) -> IResult<&'a str, Vec<Group>> {
    separated_list1(line_ending, |i: &'a str| group(side, i))(i)
}

fn input(i: &str) -> IResult<&str, (Vec<Group>, Vec<Group>)> {
    let (i, _) = tag("Immune System:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, army1) = army(Side::ImmuneSystem, i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("Infection:")(i)?;
    let (i, _) = line_ending(i)?;
    let (i, army2) = army(Side::Infection, i)?;
    Ok((i, (army1, army2)))
}

/// Run the fight until no change occurs.
///
/// Note that this doesn't guarantee that one side wins.
fn fight(groups: Vec<Group>) -> Vec<Group> {
    let mut groups = groups;
    loop {
        let mut attacks = repeat(None).take(groups.len()).collect::<Vec<_>>();
        let mut attacked_by = repeat(None).take(groups.len()).collect::<Vec<_>>();

        // choose attacks
        groups.sort();
        for i in 0..groups.len() {
            let g1 = &groups[i];
            if g1.n > 0 {
                let mut max_selection_priority = SelectionPriority {
                    damage: 0,
                    effective_power: 0,
                    initiative: 0,
                };
                let mut max_j = None;
                for j in 0..groups.len() {
                    let g2 = &groups[j];
                    if g1.is_enemy(g2) && attacked_by[j] == None {
                        let damage = g1.damage(g2);
                        let effective_power = g2.effective_power();
                        let initiative = g2.initiative;
                        let selection_priority = SelectionPriority {
                            damage,
                            effective_power,
                            initiative,
                        };
                        if damage > 0 && selection_priority > max_selection_priority {
                            max_selection_priority = selection_priority;
                            max_j = Some(j);
                        }
                    }
                }
                if let Some(j) = max_j {
                    attacked_by[j] = Some(i);
                    attacks[i] = Some(j);
                }
            }
        }

        // execute attacks
        let mut change = false;

        // sort index vector in attack order
        let mut is = (0..groups.len()).collect::<Vec<_>>();
        is.sort_by(|a, b| groups[*b].initiative.cmp(&groups[*a].initiative));

        for k in 0..groups.len() {
            let i = is[k];
            if let Some(j) = attacks[i] {
                let g1 = &groups[i];
                let g2 = &groups[j];
                let damage = g1.damage(g2);
                let kills = damage / g2.hit_points;
                groups[j].n = (g2.n - kills).max(0);

                change |= kills > 0;
            }
        }

        if !change {
            break;
        }

        // remove empty groups
        groups = groups.into_iter().filter(|g| g.n > 0).collect::<Vec<_>>();
    }
    groups
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

    let armies = result.unwrap().1;

    let result1 = {
        let mut groups = armies
            .0
            .iter()
            .chain(armies.1.iter())
            .cloned()
            .collect::<Vec<_>>();

        groups = fight(groups);
        groups.iter().map(|g| g.n).sum::<i64>()
    };

    let mut boost = 1;
    let result2 = {
        loop {
            let mut groups = armies
                .0
                .iter()
                .chain(armies.1.iter())
                .cloned()
                .collect::<Vec<_>>();

            for g in &mut groups {
                if g.side == Side::ImmuneSystem {
                    g.attack_damage += boost;
                }
            }
            groups = fight(groups);
            let n_immune_system = groups
                .iter()
                .filter(|g| g.side == Side::ImmuneSystem)
                .map(|g| g.n)
                .sum::<i64>();
            let n_infection = groups
                .iter()
                .filter(|g| g.side == Side::Infection)
                .map(|g| g.n)
                .sum::<i64>();
            if n_immune_system > 0 && n_infection == 0 {
                break n_immune_system;
            }
            boost += 1;
        }
    };

    println!("1: {}", result1);
    println!("2: {}", result2);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::attack_types;
    use crate::group;
    use crate::opt_weaknesses_immunities_clause;
    use crate::weaknesses;
    use crate::AttackType::*;
    use crate::Group;
    use crate::Side;

    #[test]
    fn test_attack_types() {
        assert_eq!(
            attack_types("radiation, bludgeoning;"),
            Ok((
                ";",
                vec![Radiation, Bludgeoning]
                    .into_iter()
                    .collect::<HashSet<_>>()
            ))
        );
    }

    #[test]
    fn test_weaknesses() {
        assert_eq!(
            weaknesses("weak to radiation, bludgeoning;"),
            Ok((
                ";",
                vec![Radiation, Bludgeoning]
                    .into_iter()
                    .collect::<HashSet<_>>()
            ))
        );
    }

    #[test]
    fn test_opt_weaknesses_immunities_clause() {
        assert_eq!(
            opt_weaknesses_immunities_clause(" "),
            Ok((" ", (HashSet::new(), HashSet::new())))
        );
        assert_eq!(
            opt_weaknesses_immunities_clause("(immune to radiation, bludgeoning; weak to cold) "),
            Ok((
                "",
                (
                    vec![Cold].into_iter().collect::<HashSet<_>>(),
                    vec![Radiation, Bludgeoning]
                        .into_iter()
                        .collect::<HashSet<_>>(),
                )
            ))
        );
    }

    #[test]
    fn test_parse_group() {
        assert_eq!(
            group(
                Side::ImmuneSystem,
                "8138 units each with 8987 hit points with an attack that does 10 bludgeoning damage at initiative 2\n",
            ),
            Ok(("\n", Group {
                side: Side::ImmuneSystem,
                n: 8138,
                hit_points: 8987,
                attack_damage: 10,
                attack_type: Bludgeoning,
                initiative: 2,
                weaknesses: HashSet::new(),
                immunities: HashSet::new()}))
        );
    }
}
