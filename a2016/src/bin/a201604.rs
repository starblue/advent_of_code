use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}
impl Room {
    fn is_real(&self) -> bool {
        self.computed_checksum() == self.checksum
    }
    fn computed_checksum(&self) -> String {
        let mut counts = HashMap::new();
        for c in self.encrypted_name.chars() {
            if c.is_alphabetic() {
                let entry = counts.entry(c).or_insert(0);
                *entry += 1;
            }
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|(ch0, count0), (ch1, count1)| count1.cmp(count0).then(ch0.cmp(ch1)));

        counts
            .into_iter()
            .take(5)
            .map(|(ch, _)| ch)
            .collect::<String>()
    }
    fn decrypted_name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| {
                if ('a'..='z').contains(&c) {
                    let code_a = u32::from('a');
                    let enc_n = u32::from(c) - code_a;
                    let dec_n = (enc_n + self.sector_id) % 26;
                    char::try_from(dec_n + code_a).unwrap()
                } else {
                    c
                }
            })
            .collect::<String>()
    }
}
impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}-{}[{}]",
            self.encrypted_name, self.sector_id, self.checksum
        )
    }
}

fn encrypted_name(i: &str) -> IResult<&str, String> {
    map(recognize(separated_list1(tag("-"), alpha1)), String::from)(i)
}

fn uint(i: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn checksum(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn room(i: &str) -> IResult<&str, Room> {
    let (i, encrypted_name) = encrypted_name(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, sector_id) = uint(i)?;
    let (i, _) = tag("[")(i)?;
    let (i, checksum) = checksum(i)?;
    let (i, _) = tag("]")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Room {
            encrypted_name,
            sector_id,
            checksum,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Room>> {
    many1(room)(i)
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
    // for room in &input {
    //     println!("{}", room);
    // }

    let result_a = input
        .iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum::<u32>();

    let result_b = input
        .iter()
        .find_map(|r| {
            if r.is_real() && r.decrypted_name() == "northpole-object-storage" {
                Some(r.sector_id)
            } else {
                None
            }
        })
        .unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
