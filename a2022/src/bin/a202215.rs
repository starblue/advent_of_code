use core::fmt;
use core::ops::Range;
use core::str::FromStr;

use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::p2d;
use lowdim::Point2d;

use util::runtime_error;

#[derive(Clone, Debug)]
struct Sensor {
    pos: Point2d,
    closest_beacon: Point2d,
}
impl Sensor {
    fn beacon_distance(&self) -> i64 {
        self.pos.distance_l1(self.closest_beacon)
    }
    fn x_range_at(&self, y: i64) -> Option<Range<i64>> {
        let dy = (y - self.pos.y()).abs();
        if dy <= self.beacon_distance() {
            let dx = self.beacon_distance() - dy;
            let x = self.pos.x();
            Some((x - dx)..(x + dx + 1))
        } else {
            None
        }
    }
}
impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            self.pos.x(),
            self.pos.y(),
            self.closest_beacon.x(),
            self.closest_beacon.y()
        )
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn point(i: &str) -> IResult<&str, Point2d> {
    let (i, _) = tag("x=")(i)?;
    let (i, x) = int(i)?;
    let (i, _) = tag(", y=")(i)?;
    let (i, y) = int(i)?;
    Ok((i, p2d(x, y)))
}

fn sensor(i: &str) -> IResult<&str, Sensor> {
    let (i, _) = tag("Sensor at ")(i)?;
    let (i, pos) = point(i)?;
    let (i, _) = tag(": closest beacon is at ")(i)?;
    let (i, closest_beacon) = point(i)?;
    Ok((
        i,
        Sensor {
            pos,
            closest_beacon,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(line_ending, sensor)(i)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    value: i64,
    is_end: bool,
}
impl Event {
    fn start(value: i64) -> Event {
        Event {
            value,
            is_end: false,
        }
    }
    fn end(value: i64) -> Event {
        Event {
            value,
            is_end: true,
        }
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for sensor in &input {
    //     println!("{}", sensor);
    // }

    let beacons = input
        .iter()
        .map(|sensor| sensor.closest_beacon)
        .collect::<HashSet<_>>();
    let y = 2_000_000;
    let mut events = Vec::new();
    for sensor in &input {
        if let Some(x_range) = sensor.x_range_at(y) {
            events.push(Event::start(x_range.start));
            events.push(Event::end(x_range.end));
        }
    }
    events.sort();
    let mut count = 0;
    let mut level = 0;
    let mut last = i64::MIN;
    for e in &events {
        if level > 0 {
            count += e.value - last;
        }
        last = e.value;
        level += {
            if e.is_end {
                -1
            } else {
                1
            }
        };
    }
    for p in &beacons {
        if p.y() == y {
            count -= 1;
        }
    }
    let result1 = count;

    let mut possible_positions = Vec::new();
    for y in 0..=4_000_000 {
        let mut events = Vec::new();
        for sensor in &input {
            if let Some(x_range) = sensor.x_range_at(y) {
                events.push(Event::start(x_range.start));
                events.push(Event::end(x_range.end));
            }
        }
        events.sort();
        let mut level = 0;
        let mut last = i64::MIN;
        for e in &events {
            if (0..=4_000_000).contains(&last) && level == 0 {
                for x in last..e.value {
                    possible_positions.push(p2d(x, y));
                }
            }
            last = e.value;
            level += {
                if e.is_end {
                    -1
                } else {
                    1
                }
            };
        }
    }
    if possible_positions.len() != 1 {
        return Err(runtime_error!(
            "wrong number of possible positions: {}",
            possible_positions.len()
        ));
    }
    let p = possible_positions[0];
    let result2 = p.x() * 4_000_000 + p.y();

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
