use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Clone, Debug)]
struct Quantity {
    n: i64,
    chemical: String,
}
impl Quantity {
    fn chemical(&self) -> &str {
        &self.chemical
    }
}
impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.n, self.chemical)
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    ins: Vec<Quantity>,
    out: Quantity,
}
impl Reaction {
    fn inputs(&self) -> &[Quantity] {
        &self.ins
    }
    fn output(&self) -> &Quantity {
        &self.out
    }
    fn output_chemical(&self) -> &str {
        self.out.chemical()
    }
}
impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sep = "";
        for q in &self.ins {
            write!(f, "{}{}", sep, q)?;
            sep = ", ";
        }
        write!(f, " => {}", self.out)
    }
}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn chemical(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn quantity(i: &str) -> IResult<&str, Quantity> {
    let (i, n) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, chemical) = chemical(i)?;
    Ok((i, Quantity { n, chemical }))
}

fn reaction(i: &str) -> IResult<&str, Reaction> {
    let (i, ins) = separated_list1(tag(", "), quantity)(i)?;
    let (i, _) = tag(" => ")(i)?;
    let (i, out) = quantity(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Reaction { ins, out }))
}

fn input(i: &str) -> IResult<&str, Vec<Reaction>> {
    many1(reaction)(i)
}

fn required_ore(map: &HashMap<&str, &Reaction>, required_fuel: i64) -> i64 {
    let mut required = HashMap::new();
    required.insert("FUEL", required_fuel);
    loop {
        let mut done = true;
        let mut new_required = HashMap::new();
        for (req_c, req_n) in required {
            if req_c == "ORE" || req_n < 0 {
                // keep required ore or surplus chemicals
                *new_required.entry(req_c).or_insert(0) += req_n;
            } else if req_n == 0 {
                // empty amount, forget it
            } else {
                // reaction required
                let reaction = map[req_c];
                let out = reaction.output();
                let out_n = out.n;
                let q = (req_n + out_n - 1) / out_n;
                let r = q * out_n - req_n;
                // keep surplus produced as a negative number
                // may cancel with additionally required amount later
                *new_required.entry(req_c).or_insert(0) -= r;
                for Quantity { n, chemical } in reaction.inputs() {
                    *new_required.entry(chemical).or_insert(0) += q * n;
                }
                done = false;
            }
        }
        required = new_required;
        if done {
            break;
        }
    }
    required["ORE"]
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

    let reactions = result.unwrap().1;
    //println!("{:?}", reactions);

    let map = reactions
        .iter()
        .map(|r| (r.output_chemical(), r))
        .collect::<HashMap<_, _>>();

    let ore = 1_000_000_000_000;
    let req1 = required_ore(&map, 1);
    let mut fuel_est = ore / req1;
    let mut old_fuel_est = 0;
    let mut ore_est;
    while fuel_est != old_fuel_est {
        old_fuel_est = fuel_est;
        ore_est = required_ore(&map, fuel_est);
        fuel_est = (ore * fuel_est) / ore_est;
    }

    let result_a = req1;
    let result_b = fuel_est;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
