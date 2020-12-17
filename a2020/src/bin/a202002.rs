use std::convert::TryFrom;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::anychar;
use nom::digit;
use nom::do_parse;
use nom::line_ending;
use nom::many1;
use nom::map;
use nom::map_res;
use nom::named;
use nom::not_line_ending;
use nom::recognize;
use nom::tag;

#[derive(Clone, Debug)]
struct Record {
    c: char,
    n1: i64,
    n2: i64,
    password: String,
}
impl Record {
    fn is_valid1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();
        let count = i64::try_from(count).unwrap();
        let min = self.n1;
        let max = self.n2;
        min <= count && count <= max
    }
    fn is_valid2(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        let index1 = usize::try_from(self.n1).unwrap();
        let index2 = usize::try_from(self.n2).unwrap();
        let found_at_1 = chars[index1] == self.c;
        let found_at_2 = chars[index2] == self.c;
        found_at_1 != found_at_2
    }
}
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} {}: {}", self.n1, self.n2, self.c, self.password)
    }
}

named!(int64<&str, i64>,
    map_res!(digit, FromStr::from_str)
);

named!(password<&str, String>,
    map!(recognize!(not_line_ending), String::from)
);

named!(record<&str, Record>,
    do_parse!(
        n1: int64 >>
        tag!("-") >>
        n2: int64 >>
        tag!(" ") >>
        c: anychar >>
        tag!(":") >>
        password: password >>
        line_ending >> (Record { n1, n2, c, password })
    )
);

named!(
    input<&str, Vec<Record>>,
    many1!(record)
);

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

    let records = result.unwrap().1;
    //println!("{:?}", records);

    let result_a = records.iter().filter(|r| r.is_valid1()).count();
    let result_b = records.iter().filter(|r| r.is_valid2()).count();
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
