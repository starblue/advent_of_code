use core::fmt;
use core::str::FromStr;

use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winning: Vec<usize>,
    present: Vec<usize>,
}
impl Card {
    fn matching_count(&self) -> usize {
        let winning = self.winning.iter().collect::<HashSet<_>>();
        self.present.iter().filter(|n| winning.contains(n)).count()
    }
    fn value(&self) -> usize {
        if self.matching_count() == 0 {
            0
        } else {
            2_usize.pow(u32::try_from(self.matching_count() - 1).unwrap())
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Card {:3}: ", self.id)?;
        let mut sep = "";
        for n in &self.winning {
            write!(f, "{sep}{n:2}")?;
            sep = " ";
        }
        write!(f, " | ")?;
        let mut sep = "";
        for n in &self.present {
            write!(f, "{sep}{n:2}")?;
            sep = " ";
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Input {
    cards: Vec<Card>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for g in &self.cards {
            writeln!(f, "{}", g)?;
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn numbers(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, uint)(i)
}

fn card(i: &str) -> IResult<&str, Card> {
    let (i, _) = tag("Card")(i)?;
    let (i, _) = space1(i)?;
    let (i, id) = uint(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = space1(i)?;
    let (i, winning) = numbers(i)?;
    let (i, _) = space1(i)?;
    let (i, _) = tag("|")(i)?;
    let (i, _) = space1(i)?;
    let (i, present) = numbers(i)?;
    Ok((
        i,
        Card {
            id,
            winning,
            present,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, cards) = separated_list1(line_ending, card)(i)?;
    Ok((i, Input { cards }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = input.cards.iter().map(Card::value).sum::<usize>();

    let mut counts = input.cards.iter().map(|_| 1).collect::<Vec<_>>();
    let len = input.cards.len();
    for i in 0..len {
        let card = &input.cards[i];
        for j in (i + 1)..=(i + card.matching_count()) {
            if j < len {
                counts[j] += counts[i];
            }
        }
    }
    let result2 = counts.iter().sum::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
