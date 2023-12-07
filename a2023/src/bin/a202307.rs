use core::cmp;
use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many_m_n;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    // Order of increasing strength
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}
impl Card {
    fn to_char(self) -> char {
        match self {
            Card::N2 => '2',
            Card::N3 => '3',
            Card::N4 => '4',
            Card::N5 => '5',
            Card::N6 => '6',
            Card::N7 => '7',
            Card::N8 => '8',
            Card::N9 => '9',
            Card::T => 'T',
            Card::J => 'J',
            Card::Q => 'Q',
            Card::K => 'K',
            Card::A => 'A',
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct JokerCard(Card);
impl fmt::Display for JokerCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}
impl Ord for JokerCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.0, other.0) {
            (Card::J, Card::J) => Ordering::Equal,
            (Card::J, _) => Ordering::Less,
            (_, Card::J) => Ordering::Greater,
            (c0, c1) => c0.cmp(&c1),
        }
    }
}
impl PartialOrd for JokerCard {
    fn partial_cmp(&self, other: &JokerCard) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    // Order of increasing strength
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl HandType {
    fn for_cards(cards: &[Card]) -> HandType {
        assert!(cards.len() == 5);
        let mut counts = HashMap::new();
        for c in cards {
            let e = counts.entry(c).or_insert(0);
            *e += 1;
        }
        let mut counts = counts.values().copied().collect::<Vec<_>>();
        counts.sort();
        counts.reverse();

        HandType::from_counts(&counts)
    }
    fn for_joker_cards(cards: &[Card]) -> HandType {
        assert!(cards.len() == 5);
        let mut counts = HashMap::new();
        for c in cards {
            let e = counts.entry(c).or_insert(0);
            *e += 1;
        }
        let joker_count = counts.remove(&Card::J).unwrap_or(0);
        let mut counts = counts.values().copied().collect::<Vec<_>>();
        if counts.is_empty() {
            // We have five jokers and no other card,
            // add a dummy for accepting the jokers.
            counts.push(0);
        }
        counts.sort();
        counts.reverse();

        // The jokers are added to the card with the most occurrences.
        counts[0] += joker_count;

        HandType::from_counts(&counts)
    }
    fn from_counts(counts: &[usize]) -> HandType {
        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if counts[0] == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if counts[0] == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}
impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        assert!(cards.len() == 5);
        let hand_type = HandType::for_cards(&cards);
        Hand { hand_type, cards }
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct JokerHand {
    hand_type: HandType,
    joker_cards: Vec<JokerCard>,
}
impl JokerHand {
    fn new(cards: Vec<Card>) -> JokerHand {
        assert!(cards.len() == 5);
        let hand_type = HandType::for_joker_cards(&cards);
        let joker_cards = cards.into_iter().map(JokerCard).collect::<Vec<_>>();
        JokerHand {
            hand_type,
            joker_cards,
        }
    }
}

#[derive(Clone, Debug)]
struct Item {
    hand: Hand,
    bid: usize,
}
impl Item {
    fn hand(&self) -> Hand {
        self.hand.clone()
    }
    fn joker_hand(&self) -> JokerHand {
        let cards = self.hand.cards.to_vec();
        JokerHand::new(cards)
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.hand, self.bid)
    }
}

#[derive(Clone, Debug)]
struct Input {
    items: Vec<Item>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

fn uint(i: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn card(i: &str) -> IResult<&str, Card> {
    alt((
        value(Card::N2, tag("2")),
        value(Card::N3, tag("3")),
        value(Card::N4, tag("4")),
        value(Card::N5, tag("5")),
        value(Card::N6, tag("6")),
        value(Card::N7, tag("7")),
        value(Card::N8, tag("8")),
        value(Card::N9, tag("9")),
        value(Card::T, tag("T")),
        value(Card::J, tag("J")),
        value(Card::Q, tag("Q")),
        value(Card::K, tag("K")),
        value(Card::A, tag("A")),
    ))(i)
}

fn hand(i: &str) -> IResult<&str, Hand> {
    let (i, cards) = many_m_n(5, 5, card)(i)?;
    Ok((i, Hand::new(cards)))
}

fn item(i: &str) -> IResult<&str, Item> {
    let (i, hand) = hand(i)?;
    let (i, _) = space1(i)?;
    let (i, bid) = uint(i)?;
    Ok((i, Item { hand, bid }))
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, items) = separated_list1(line_ending, item)(i)?;
    Ok((i, Input { items }))
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input1);

    let mut items1 = input.items.clone();
    items1.sort_by_key(|item| item.hand());
    let result1 = items1
        .into_iter()
        .enumerate()
        .map(|(i, item)| (i + 1) * item.bid)
        .sum::<usize>();

    let mut items2 = input.items.clone();
    items2.sort_by_key(|item| item.joker_hand());
    let result2 = items2
        .into_iter()
        .enumerate()
        .map(|(i, item)| (i + 1) * item.bid)
        .sum::<usize>();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
