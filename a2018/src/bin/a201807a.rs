use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Before(char, char);

fn record(i: &str) -> IResult<&str, Before> {
    let (i, _) = tag("Step ")(i)?;
    let (i, step0) = alpha1(i)?;
    let (i, _) = tag(" must be finished before step ")(i)?;
    let (i, step1) = alpha1(i)?;
    let (i, _) = tag(" can begin.")(i)?;
    Ok((
        i,
        Before(step0.chars().next().unwrap(), step1.chars().next().unwrap()),
    ))
}

fn main() {
    let mut records = Vec::new();

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        if line.is_empty() {
            break;
        }

        // parse line
        let result = record(line.trim_end());
        //println!("{:?}", result);

        let r = result.unwrap().1;
        records.push(r);
    }

    let mut steps = HashSet::new();
    for Before(s0, s1) in &records {
        steps.insert(*s0);
        steps.insert(*s1);
    }

    let mut result = String::new();
    while !steps.is_empty() {
        let afters = records
            .iter()
            .map(|&Before(_s0, s1)| s1)
            .collect::<HashSet<_>>();
        let mut d = steps.difference(&afters).collect::<Vec<_>>();
        d.sort_unstable();

        let step = *d.remove(0);
        result.push(step);

        steps.remove(&step);
        records = records
            .into_iter()
            .filter(move |&Before(s0, _)| s0 != step)
            .collect();
    }

    println!("{}", result);
}
