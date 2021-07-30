use std::collections::HashMap;
use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::alt;
use nom::char;
use nom::character::complete::digit1;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::tag;
use nom::value;

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

named!(int32<&str, i32>,
    map_res!(digit1, FromStr::from_str)
);

named!(timestamp<&str, Timestamp>,
    do_parse!(
        char!('[') >>
        year: int32 >>
        char!('-') >>
        month: int32 >>
        char!('-') >>
        day: int32 >>
        char!(' ') >>
        hour: int32 >>
        char!(':') >>
        minute: int32 >>
        char!(']') >>
            (Timestamp { year, month, day, hour, minute })
    )
);

named!(id<&str, i32>, do_parse!(char!('#') >> id: int32 >> (id) ));

named!(action<&str, Action>,
    alt!(
        value!(Action::FallAsleep, tag!(" falls asleep"))
            | value!(Action::WakeUp, tag!(" wakes up"))
            | do_parse!(
                tag!(" Guard ")
                    >> id: id
                    >> tag!(" begins shift")
                    >> (Action::BeginShift { id })
            )
    )
);

named!(
    record<&str, Record>,
    do_parse!(
        ts: timestamp
            >> action: action
            >> (Record { ts, action })
    )
);

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

    // compute sleep counts per guard and minute
    let mut sleep_counts = HashMap::new();
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
                let minute_counts = sleep_counts
                    .entry(current_id)
                    .or_insert(repeat(0).take(60).collect::<Vec<_>>());
                let sleep_end = r.ts.minute;
                for m in sleep_start..sleep_end {
                    minute_counts[m as usize] += 1;
                }
            }
        }
    }

    // find guard sleeping most
    let mut id_max = 0;
    let mut minute_max = 0;
    let mut count_max = 0;
    for (id, minute_counts) in sleep_counts {
        for m in 0..60 {
            let c = minute_counts[m as usize];
            if c > count_max {
                count_max = c;
                id_max = id;
                minute_max = m;
            }
        }
    }

    println!("{}", id_max * minute_max);
}
