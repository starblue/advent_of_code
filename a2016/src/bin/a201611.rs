use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::Hash;
use std::io;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use pathfinding::prelude::astar;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Device<Material> {
    Generator(Material),
    Microchip(Material),
}
impl<Material> Device<Material>
where
    Material: Clone,
{
    fn is_generator(&self) -> bool {
        match self {
            Device::Generator(_) => true,
            Device::Microchip(_) => false,
        }
    }
    fn material(&self) -> Material {
        match self {
            Device::Generator(m) => m.clone(),
            Device::Microchip(m) => m.clone(),
        }
    }
}
impl<Material> fmt::Display for Device<Material>
where
    Material: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Device::Generator(material) => write!(f, "a {} generator", material),
            Device::Microchip(material) => write!(f, "a {}-compatible microchip", material),
        }
    }
}

const FLOORS: [&str; 5] = ["ground", "first", "second", "third", "fourth"];

#[derive(Clone, Debug)]
struct Floor<Material> {
    number: usize,
    contents: Vec<Device<Material>>,
}
impl<'int, Material> fmt::Display for Floor<Material>
where
    Material: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "The {} floor contains ", FLOORS[self.number])?;
        if self.contents.is_empty() {
            write!(f, "nothing relevant.")?;
        } else if self.contents.len() == 1 {
            write!(f, "{}.", self.contents[0])?;
        } else if self.contents.len() == 2 {
            write!(f, "{} and {}.", self.contents[0], self.contents[1])?;
        } else {
            for i in 0..(self.contents.len() - 1) {
                write!(f, "{}, ", self.contents[i])?;
            }
            write!(f, "and {}.", self.contents[self.contents.len() - 1])?;
        }
        Ok(())
    }
}

fn string(i: &str) -> Result<(&str, String), nom::Err<nom::error::Error<&str>>> {
    map(recognize(alpha1), String::from)(i)
}

fn material(i: &str) -> IResult<&str, String> {
    let (i, s) = string(i)?;
    Ok((i, s))
}

fn device_generator(i: &str) -> IResult<&str, Device<String>> {
    let (i, _) = tag("a ")(i)?;
    let (i, material) = material(i)?;
    let (i, _) = tag(" generator")(i)?;
    Ok((i, Device::Generator(material)))
}
fn device_microchip(i: &str) -> IResult<&str, Device<String>> {
    let (i, _) = tag("a ")(i)?;
    let (i, material) = material(i)?;
    let (i, _) = tag("-compatible microchip")(i)?;
    Ok((i, Device::Microchip(material)))
}
fn device(i: &str) -> IResult<&str, Device<String>> {
    alt((device_generator, device_microchip))(i)
}

fn floor_number<'i>(i: &'i str) -> IResult<&'i str, usize> {
    alt((
        value(1, tag("first")),
        value(2, tag("second")),
        value(3, tag("third")),
        value(4, tag("fourth")),
    ))(i)
}

fn floor_contents_empty(i: &str) -> IResult<&str, Vec<Device<String>>> {
    value(Vec::<Device<String>>::new(), tag("nothing relevant"))(i)
}
fn floor_contents_single(i: &str) -> IResult<&str, Vec<Device<String>>> {
    let (i, d) = device(i)?;
    let (i, _) = peek(tag("."))(i)?;
    Ok((i, vec![d]))
}
fn floor_contents_double(i: &str) -> IResult<&str, Vec<Device<String>>> {
    let (i, d0) = device(i)?;
    let (i, _) = tag(" and ")(i)?;
    let (i, d1) = device(i)?;
    Ok((i, vec![d0, d1]))
}
fn floor_contents_multiple(i: &str) -> IResult<&str, Vec<Device<String>>> {
    let (i, mut ds) = separated_list1(tag(", "), device)(i)?;
    let (i, _) = tag(", and ")(i)?;
    let (i, d) = device(i)?;
    ds.push(d);
    Ok((i, ds))
}
fn floor_contents(i: &str) -> IResult<&str, Vec<Device<String>>> {
    alt((
        floor_contents_empty,
        floor_contents_single,
        floor_contents_double,
        floor_contents_multiple,
    ))(i)
}

fn floor(i: &str) -> IResult<&str, Floor<String>> {
    let (i, _) = tag("The ")(i)?;
    let (i, number) = floor_number(i)?;
    let (i, _) = tag(" floor contains ")(i)?;
    let (i, contents) = floor_contents(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Floor { number, contents }))
}

fn input(i: &str) -> IResult<&str, Vec<Floor<String>>> {
    many1(|i| floor(i))(i)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    elevator_floor: usize,
    floors: Vec<Vec<Device<usize>>>,
}
impl Node {
    fn start(input: &[Floor<usize>]) -> Node {
        let elevator_floor = 1;
        let mut floors = Vec::new();
        for floor in input {
            let n = floor.number;
            if floors.len() < n + 1 {
                floors.resize(n + 1, Vec::new());
            }
            let devices = &mut floors[n];
            for device in &floor.contents {
                devices.push(device.clone());
            }
            devices.sort();
        }
        Node {
            elevator_floor,
            floors,
        }
    }
    /// Nodes reached in a single step from this node
    fn successors(&self) -> Vec<(Node, usize)> {
        let mut result = Vec::new();
        let floor = self.elevator_floor;

        // try go up one floor with elevator
        if floor < 4 {
            let new_floor = floor + 1;

            // pick one or two devices to move up
            let devices = &self.floors[floor];

            // pick one device
            for d0 in devices {
                let mut new_floors = self.floors.clone();
                let i0 = new_floors[floor].binary_search(d0).unwrap();
                new_floors[floor].remove(i0);
                new_floors[new_floor].push(d0.clone());
                new_floors[new_floor].sort();
                let new_node = Node {
                    elevator_floor: new_floor,
                    floors: new_floors,
                };
                if new_node.is_allowed() {
                    result.push((new_node, 1));
                }
            }

            // pick two devices
            for d0 in devices {
                for d1 in devices {
                    if d0 < d1 {
                        let mut new_floors = self.floors.clone();
                        let i0 = new_floors[floor].binary_search(d0).unwrap();
                        new_floors[floor].remove(i0);
                        let i1 = new_floors[floor].binary_search(d1).unwrap();
                        new_floors[floor].remove(i1);
                        new_floors[new_floor].push(d0.clone());
                        new_floors[new_floor].push(d1.clone());
                        new_floors[new_floor].sort();
                        let new_node = Node {
                            elevator_floor: new_floor,
                            floors: new_floors,
                        };
                        if new_node.is_allowed() {
                            result.push((new_node, 1));
                        }
                    }
                }
            }
        }

        // try go down one floor with elevator
        if floor > 1 {
            let new_floor = floor - 1;

            // pick one or two devices to move up
            let devices = &self.floors[floor];

            // pick one device
            for d0 in devices {
                let mut new_floors = self.floors.clone();
                let i0 = new_floors[floor].binary_search(d0).unwrap();
                new_floors[floor].remove(i0);
                new_floors[new_floor].push(d0.clone());
                new_floors[new_floor].sort();
                let new_node = Node {
                    elevator_floor: new_floor,
                    floors: new_floors,
                };
                if new_node.is_allowed() {
                    result.push((new_node, 1));
                }
            }

            // pick two devices
            for d0 in devices {
                for d1 in devices {
                    if d0 < d1 {
                        let mut new_floors = self.floors.clone();
                        let i0 = new_floors[floor].binary_search(d0).unwrap();
                        new_floors[floor].remove(i0);
                        let i1 = new_floors[floor].binary_search(d1).unwrap();
                        new_floors[floor].remove(i1);
                        new_floors[new_floor].push(d0.clone());
                        new_floors[new_floor].push(d1.clone());
                        new_floors[new_floor].sort();
                        let new_node = Node {
                            elevator_floor: new_floor,
                            floors: new_floors,
                        };
                        if new_node.is_allowed() {
                            result.push((new_node, 1));
                        }
                    }
                }
            }
        }
        result
    }
    /// Returns the minimum number of steps needed
    /// to success from this node, or possibly less.
    fn heuristic(&self) -> usize {
        let mut steps_devices =
            3 * self.floors[1].len() + 2 * self.floors[2].len() + 1 * self.floors[3].len();

        //println!("{}: {}", self, steps_devices);

        // The lowest device which is not alone travels for free,
        // because the elevator can carry two devices.
        if self.floors[1].len() >= 2 {
            steps_devices -= 3;
        }
        if self.floors[2].len() >= 2 {
            steps_devices -= 2;
        }
        if self.floors[3].len() >= 2 {
            steps_devices -= 1;
        } else {
            // do nothing
        }
        let steps_elevator = 4 - self.elevator_floor;
        steps_devices.max(steps_elevator)
    }
    /// Return true if this a success node, false otherwise.
    fn is_success(&self) -> bool {
        self.elevator_floor == 4
            && self.floors[1].is_empty()
            && self.floors[2].is_empty()
            && self.floors[3].is_empty()
    }
    fn is_allowed(&self) -> bool {
        // check that no chips are fried
        self.floors.iter().all(|devices| {
            devices.iter().all(|d0| {
                if let Device::Microchip(m) = d0 {
                    devices.iter().any(|d1| d1 == &Device::Generator(m.clone()))
                        || devices.iter().all(|d1| !d1.is_generator())
                } else {
                    true
                }
            })
        })
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for floor in 1..=4 {
            write!(f, "{}: ", floor)?;
            let mut sep = "";
            for device in &self.floors[floor] {
                write!(f, "{}{}", sep, device)?;
                sep = ", ";
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn map_device<T0, T1>(map: &BTreeMap<T0, T1>, device: &Device<T0>) -> Device<T1>
where
    T0: PartialOrd + Ord,
    T1: Clone,
{
    match device {
        Device::Generator(ms) => Device::Generator(map[ms].clone()),
        Device::Microchip(ms) => Device::Microchip(map[ms].clone()),
    }
}

fn map_floor<T0, T1>(map: &BTreeMap<T0, T1>, floor: &Floor<T0>) -> Floor<T1>
where
    T0: PartialOrd + Ord,
    T1: Clone,
{
    let Floor { number, contents } = floor;
    let number = *number;
    let contents = contents
        .iter()
        .map(|d| map_device(map, d))
        .collect::<Vec<_>>();
    Floor { number, contents }
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
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for i in &input {
    //     println!("{}", i);
    // }

    // Construct a map from material names (strings) to ids (unsigned integers).
    let materials_a = input
        .iter()
        .flat_map(|floor| floor.contents.iter().map(|d| d.material()))
        .collect::<BTreeSet<_>>();
    let material_map_a = materials_a
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<BTreeMap<_, _>>();
    // println!("{:?}", material_map);

    // Map material names to ids throughout the input structure.
    // This speeds up the search by a factor of 2.5 .
    let floors_a = input
        .iter()
        .map(|floor| map_floor(&material_map_a, floor))
        .collect::<Vec<_>>();

    let start_node_a = Node::start(&floors_a);
    let search_result_a = astar(
        &start_node_a,
        Node::successors,
        Node::heuristic,
        Node::is_success,
    );
    let (_path, cost_a) = search_result_a.unwrap();
    let result_a = cost_a;

    // Add additional devices as specified in the problem part 2.
    let mut input_b = input;
    for floor in &mut input_b {
        if floor.number == 1 {
            for d in [
                Device::Generator("elerium".to_string()),
                Device::Microchip("elerium".to_string()),
                Device::Generator("dilithium".to_string()),
                Device::Microchip("dilithium".to_string()),
            ] {
                floor.contents.push(d);
            }
        }
    }

    // Construct a map from material names (strings) to ids (unsigned integers).
    let materials_b = input_b
        .iter()
        .flat_map(|floor| floor.contents.iter().map(|d| d.material()))
        .collect::<BTreeSet<_>>();
    let material_map_b = materials_b
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<BTreeMap<_, _>>();
    // println!("{:?}", materials_b);

    // map material names to ids throughout the input structure
    let floors_b = input_b
        .iter()
        .map(|floor| map_floor(&material_map_b, floor))
        .collect::<Vec<_>>();

    let start_node_b = Node::start(&floors_b);
    let search_result_b = astar(
        &start_node_b,
        Node::successors,
        Node::heuristic,
        Node::is_success,
    );
    let (_path, cost_b) = search_result_b.unwrap();
    let result_b = cost_b;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
