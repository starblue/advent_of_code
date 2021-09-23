use std::collections::HashMap;
use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::value;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Timestamp {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    BeginShift { id: i32 },
    WakeUp,
    FallAsleep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Record {
    ts: Timestamp,
    action: Action,
}

#[derive(Clone, Debug)]
enum Error {}

fn int32(i: &str) -> IResult<&str, i32> {
    map_res(digit1, FromStr::from_str)(i)
}

fn timestamp(i: &str) -> IResult<&str, Timestamp> {
    let (i, _) = char('[')(i)?;
    let (i, year) = int32(i)?;
    let (i, _) = char('-')(i)?;
    let (i, month) = int32(i)?;
    let (i, _) = char('-')(i)?;
    let (i, day) = int32(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, hour) = int32(i)?;
    let (i, _) = char(':')(i)?;
    let (i, minute) = int32(i)?;
    let (i, _) = char(']')(i)?;
    Ok((
        i,
        Timestamp {
            year,
            month,
            day,
            hour,
            minute,
        },
    ))
}

fn id(i: &str) -> IResult<&str, i32> {
    let (i, _) = char('#')(i)?;
    let (i, id) = int32(i)?;
    Ok((i, id))
}

fn action_begin_shift(i: &str) -> IResult<&str, Action> {
    let (i, _) = tag(" Guard ")(i)?;
    let (i, id) = id(i)?;
    let (i, _) = tag(" begins shift")(i)?;
    Ok((i, Action::BeginShift { id }))
}

fn action(i: &str) -> IResult<&str, Action> {
    let p0 = value(Action::FallAsleep, tag(" falls asleep"));
    let p1 = value(Action::WakeUp, tag(" wakes up"));
    alt((p0, p1, action_begin_shift))(i)
}

fn record(i: &str) -> IResult<&str, Record> {
    let (i, ts) = timestamp(i)?;
    let (i, action) = action(i)?;
    Ok((i, Record { ts, action }))
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

    // sort chronologically
    records.sort_unstable();

    // compute sleep durations per guard
    let mut sleep_durations = HashMap::new();
    let mut current_id = 0;
    let mut sleep_start = 0;
    for r in &records {
        match r.action {
            Action::BeginShift { id } => {
                current_id = id;
            }
            Action::FallAsleep => {
                sleep_start = r.ts.minute;
            }
            Action::WakeUp => {
                let sleep_end = r.ts.minute;
                let sleep_duration = sleep_durations.entry(current_id).or_insert(0);
                *sleep_duration += sleep_end - sleep_start;
            }
        }
    }

    // find guard sleeping most
    let mut d_max = 0;
    let mut id_max = 0;
    for (id, d) in sleep_durations {
        if d > d_max {
            d_max = d;
            id_max = id;
        }
    }

    // find sleep count for minutes
    let mut minute_counts = repeat(0).take(60).collect::<Vec<_>>();
    let mut current_id = 0;
    let mut sleep_start = 0;
    for r in &records {
        match r.action {
            Action::BeginShift { id } => {
                current_id = id;
            }
            Action::FallAsleep => {
                sleep_start = r.ts.minute;
            }
            Action::WakeUp => {
                if current_id == id_max {
                    let sleep_end = r.ts.minute;
                    for m in sleep_start..sleep_end {
                        minute_counts[m as usize] += 1;
                    }
                }
            }
        }
    }

    let mut count_max = 0;
    let mut minute_max = 0;
    for m in 0..60 {
        let c = minute_counts[m as usize];
        if c > count_max {
            count_max = c;
            minute_max = m;
        }
    }

    println!("{}", id_max * minute_max);
}
