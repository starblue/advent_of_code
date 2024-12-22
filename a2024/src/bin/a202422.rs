use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::u32;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn input(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(line_ending, u32)(i)
}

#[derive(Clone, Debug)]
struct SecretNumbers {
    secret: u32,
}
impl SecretNumbers {
    fn new(n: u32) -> SecretNumbers {
        SecretNumbers { secret: n }
    }
}
impl Iterator for SecretNumbers {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let s = self.secret;
        let mask = (1 << 24) - 1;
        let s = (s ^ (s << 6)) & mask;
        let s = (s ^ (s >> 5)) & mask;
        let s = (s ^ (s << 11)) & mask;
        self.secret = s;
        Some(s)
    }
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for n in &input {
    //     println!("{}", n);
    // }

    let result1 = input
        .iter()
        .map(|&n| {
            SecretNumbers::new(n)
                .nth(1999)
                .map(i64::from)
                .ok_or("internal error: no nth item".into())
        })
        .sum::<Result<i64>>()?;

    let mut sequence_bananas = HashMap::new();
    for &initial_secret in &input {
        let mut seen_sequences = HashSet::new();
        let mut changes = Vec::new();
        let mut previous_price = i64::from(initial_secret) % 10;
        for s in SecretNumbers::new(initial_secret).take(2000) {
            let price = i64::from(s) % 10;
            let change = price - previous_price;
            previous_price = price;

            changes.push(change);
            if changes.len() > 4 {
                changes.remove(0);
            }

            if changes.len() == 4 && !seen_sequences.contains(&changes) {
                seen_sequences.insert(changes.clone());
                let entry = sequence_bananas.entry(changes.clone()).or_insert(0);
                *entry += price;
            }
        }
    }
    let result2 = sequence_bananas
        .values()
        .max()
        .ok_or("internal error: no sequence")?;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(
            vec![
                15887950, 16495136, 527345, 704524, 1553684, //
                12683156, 11100544, 12249484, 7753432, 5908254,
            ],
            SecretNumbers { secret: 123 }.take(10).collect::<Vec<_>>()
        );
    }
    #[test]
    fn test_2000th() {
        assert_eq!(8685429, SecretNumbers::new(1).nth(1999).unwrap());
        assert_eq!(4700978, SecretNumbers::new(10).nth(1999).unwrap());
        assert_eq!(15273692, SecretNumbers::new(100).nth(1999).unwrap());
        assert_eq!(8667524, SecretNumbers::new(2024).nth(1999).unwrap());
    }
}
