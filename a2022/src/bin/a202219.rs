use core::cmp;
use core::fmt;
use core::ops;
use core::str::FromStr;

use std::collections::HashMap;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn s(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResourceType {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}
use ResourceType::*;
impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ore => write!(f, "ore"),
            Clay => write!(f, "clay"),
            Obsidian => write!(f, "obsidian"),
            Geode => write!(f, "geode"),
        }
    }
}
const RESOURCE_TYPES: [ResourceType; 4] = [Ore, Clay, Obsidian, Geode];

#[derive(Clone, Copy, Debug)]
struct Resource {
    amount: i32,
    type_: ResourceType,
}
impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.type_)
    }
}

#[derive(Clone, Debug)]
struct Cost {
    resources: ResourceMap<i32>,
}
impl fmt::Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        for rt in RESOURCE_TYPES {
            let amount = self.resources[rt];
            if amount > 0 {
                write!(f, "{}{} {}", sep, amount, rt)?;
                sep = " and ";
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Robot {
    product: ResourceType,
    cost: Cost,
}
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Each {} robot costs {}.", self.product, self.cost)
    }
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: i32,
    robots: ResourceMap<Robot>,
    max_production: ResourceMap<i32>,
}
impl Blueprint {
    fn new(id: i32, bots: Vec<Robot>) -> Blueprint {
        let robots = ResourceMap::with(|rt| {
            bots.iter().cloned().find(|b| b.product == rt).unwrap()
        });
        let mut max_production = ResourceMap::<i32>::new();
        // Limit production to maximum that can be used.
        for robot in bots {
            for rt in RESOURCE_TYPES {
                max_production[rt] =
                    max_production[rt].max(robot.cost.resources[rt]);
            }
        }
        // Don't limit geode production.
        max_production[Geode] = i32::MAX;

        Blueprint {
            id,
            robots,
            max_production,
        }
    }
    fn max_geodes(&self, minutes: i32) -> i32 {
        let state = State::initial(minutes);
        let mut table = HashMap::new();
        state.max_geodes(self, &mut table, 0)
    }
    fn robot_cost(&self, rt: ResourceType) -> &ResourceMap<i32> {
        &self.robots[rt].cost.resources
    }
}
impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blueprint {}: ", self.id)?;
        let mut sep = "";
        for rt in RESOURCE_TYPES {
            write!(f, "{}{}", sep, self.robots[rt])?;
            sep = " ";
        }
        Ok(())
    }
}

fn resource_type(i: &str) -> IResult<&str, ResourceType> {
    alt((
        value(ResourceType::Ore, tag("ore")),
        value(ResourceType::Clay, tag("clay")),
        value(ResourceType::Obsidian, tag("obsidian")),
        value(ResourceType::Geode, tag("geode")),
    ))(i)
}

fn int(i: &str) -> IResult<&str, i32> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn resource(i: &str) -> IResult<&str, Resource> {
    let (i, amount) = int(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, type_) = resource_type(i)?;
    Ok((i, Resource { amount, type_ }))
}

fn cost(i: &str) -> IResult<&str, Cost> {
    let (i, rs) = separated_list1(tag(" and "), resource)(i)?;
    let mut resources = ResourceMap::new();
    for r in rs {
        resources[r.type_] += r.amount;
    }
    Ok((i, Cost { resources }))
}

fn robot(i: &str) -> IResult<&str, Robot> {
    let (i, _) = tag("Each ")(i)?;
    let (i, product) = resource_type(i)?;
    let (i, _) = tag(" robot costs ")(i)?;
    let (i, cost) = cost(i)?;
    let (i, _) = tag(".")(i)?;
    Ok((i, Robot { product, cost }))
}

fn blueprint(i: &str) -> IResult<&str, Blueprint> {
    let (i, _) = tag("Blueprint ")(i)?;
    let (i, id) = int(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = multispace1(i)?;
    let (i, robots) = separated_list1(multispace1, robot)(i)?;
    Ok((i, Blueprint::new(id, robots)))
}

fn input(i: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(multispace1, blueprint)(i)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct ResourceMap<T> {
    values: [T; 4],
}
impl<T> ResourceMap<T> {
    fn new() -> ResourceMap<T>
    where
        T: Copy + Default,
    {
        ResourceMap::default()
    }
    fn with<F>(f: F) -> ResourceMap<T>
    where
        F: Fn(ResourceType) -> T,
    {
        ResourceMap {
            values: [f(Ore), f(Clay), f(Obsidian), f(Geode)],
        }
    }
}

impl<T> ops::Index<ResourceType> for ResourceMap<T> {
    type Output = T;
    fn index(&self, rt: ResourceType) -> &T {
        &self.values[rt as usize]
    }
}
impl<T> ops::IndexMut<ResourceType> for ResourceMap<T> {
    fn index_mut(&mut self, rt: ResourceType) -> &mut T {
        &mut self.values[rt as usize]
    }
}

impl ops::Add<ResourceMap<i32>> for ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn add(self, other: ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] + other[rt])
    }
}
impl ops::Add<ResourceMap<i32>> for &ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn add(self, other: ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] + other[rt])
    }
}
impl ops::Add<&ResourceMap<i32>> for ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn add(self, other: &ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] + other[rt])
    }
}
impl ops::Add<&ResourceMap<i32>> for &ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn add(self, other: &ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] + other[rt])
    }
}

impl ops::Sub<ResourceMap<i32>> for ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn sub(self, other: ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] - other[rt])
    }
}
impl ops::Sub<ResourceMap<i32>> for &ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn sub(self, other: ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] - other[rt])
    }
}
impl ops::Sub<&ResourceMap<i32>> for ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn sub(self, other: &ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] - other[rt])
    }
}
impl ops::Sub<&ResourceMap<i32>> for &ResourceMap<i32> {
    type Output = ResourceMap<i32>;
    fn sub(self, other: &ResourceMap<i32>) -> ResourceMap<i32> {
        ResourceMap::with(|rt| self[rt] - other[rt])
    }
}

impl ops::AddAssign<ResourceMap<i32>> for ResourceMap<i32> {
    fn add_assign(&mut self, other: ResourceMap<i32>) {
        for rt in RESOURCE_TYPES {
            self[rt] += other[rt];
        }
    }
}
impl ops::AddAssign<&ResourceMap<i32>> for ResourceMap<i32> {
    fn add_assign(&mut self, other: &ResourceMap<i32>) {
        for rt in RESOURCE_TYPES {
            self[rt] += other[rt];
        }
    }
}

impl ops::SubAssign<ResourceMap<i32>> for ResourceMap<i32> {
    fn sub_assign(&mut self, other: ResourceMap<i32>) {
        for rt in RESOURCE_TYPES {
            self[rt] -= other[rt];
        }
    }
}
impl ops::SubAssign<&ResourceMap<i32>> for ResourceMap<i32> {
    fn sub_assign(&mut self, other: &ResourceMap<i32>) {
        for rt in RESOURCE_TYPES {
            self[rt] -= other[rt];
        }
    }
}

fn partial_and_then(
    a: Option<cmp::Ordering>,
    b: Option<cmp::Ordering>,
) -> Option<cmp::Ordering> {
    use cmp::Ordering::*;
    match (a, b) {
        (Some(Less), Some(Less)) => Some(Less),
        (Some(Greater), Some(Greater)) => Some(Greater),
        (Some(Equal), ordering) => ordering,
        (ordering, Some(Equal)) => ordering,
        _ => None,
    }
}
impl PartialOrd<ResourceMap<i32>> for ResourceMap<i32> {
    fn partial_cmp(
        &self,
        other: &ResourceMap<i32>,
    ) -> Option<std::cmp::Ordering> {
        partial_and_then(
            partial_and_then(
                partial_and_then(
                    self[Ore].partial_cmp(&other[Ore]),
                    self[Clay].partial_cmp(&other[Clay]),
                ),
                self[Obsidian].partial_cmp(&other[Obsidian]),
            ),
            self[Geode].partial_cmp(&other[Geode]),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    minutes: i32,
    resources: ResourceMap<i32>,
    robots: ResourceMap<i32>,
}
impl State {
    fn initial(minutes: i32) -> State {
        let resources = ResourceMap::new();
        let mut robots = ResourceMap::new();
        robots[Ore] += 1;
        State {
            minutes,
            resources,
            robots,
        }
    }
    fn max_geodes(
        &self,
        blueprint: &Blueprint,
        table: &mut HashMap<State, i32>,
        max_geodes: i32,
    ) -> i32 {
        if self.minutes == 0 {
            max_geodes.max(self.resources[Geode])
        } else if self.minutes == 1 {
            // A new robot won't have time to produce anything.
            max_geodes.max(self.resources[Geode] + self.robots[Geode])
        } else {
            // Can we improve the maximum if we produce a new geode robot
            // every minute?
            if self.resources[Geode]
                + self.minutes * self.robots[Geode]
                + s(self.minutes - 1)
                < max_geodes
            {
                // This can't improve the maximum, cut it.
                max_geodes
            } else if let Some(&geodes) = table.get(self) {
                // We already did this before, use the stored value.
                max_geodes.max(geodes)
            } else {
                // Let's try to do better.
                let mut max_geodes = max_geodes;
                let mut saving_makes_sense = false;

                // Try producing end products first to get a high value early.
                // This will enable cutting of later branches.
                for &rt in &[Geode, Obsidian, Clay, Ore] {
                    let robot_cost = blueprint.robot_cost(rt);
                    if &self.resources >= robot_cost {
                        // We have the resources to build this robot.
                        if self.robots[rt] < blueprint.max_production[rt] {
                            // Produce robot, as its product may be needed.
                            let minutes = self.minutes - 1;
                            let resources =
                                &self.resources - robot_cost + &self.robots;
                            let mut robots = self.robots.clone();
                            robots[rt] += 1;
                            let state = State {
                                minutes,
                                resources,
                                robots,
                            };
                            max_geodes =
                                state.max_geodes(blueprint, table, max_geodes);
                        }
                    } else {
                        // We can't build this robot due to lack of resources.
                        saving_makes_sense = true;
                    }
                }
                if saving_makes_sense {
                    // Do not produce a new robot this turn.
                    let minutes = self.minutes - 1;
                    let resources = &self.resources + &self.robots;
                    let robots = self.robots.clone();
                    let state = State {
                        minutes,
                        resources,
                        robots,
                    };
                    max_geodes =
                        state.max_geodes(blueprint, table, max_geodes);
                }

                table.insert(self.clone(), max_geodes);
                max_geodes
            }
        }
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for blueprint in &input {
    //     println!("{}", blueprint);
    // }

    let mut sum = 0;
    for blueprint in &input {
        let max_geodes = blueprint.max_geodes(24);
        // println!("Blueprint {}: {}", blueprint.id, max_geodes);
        sum += blueprint.id * max_geodes;
    }
    let result1 = sum;

    let mut product = 1;
    for blueprint in &input[..3] {
        let max_geodes = blueprint.max_geodes(32);
        // println!("Blueprint {}: {}", blueprint.id, max_geodes);
        product *= max_geodes;
    }
    let result2 = product;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
