use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cave<'a> {
    name: &'a str,
}
impl<'a> Cave<'a> {
    fn start() -> Cave<'static> {
        Cave { name: "start" }
    }
    fn is_small(&self) -> bool {
        self.name.chars().all(|c| c.is_lowercase())
    }
    fn is_start(&self) -> bool {
        self.name == "start"
    }
    fn is_end(&self) -> bool {
        self.name == "end"
    }
}
impl<'a> fmt::Display for Cave<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug)]
struct Edge<'a> {
    cave0: Cave<'a>,
    cave1: Cave<'a>,
}
impl<'a> Edge<'a> {}
impl<'a> fmt::Display for Edge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}-{}", self.cave0, self.cave1)
    }
}

fn cave(i: &str) -> IResult<&str, Cave> {
    let (i, name) = alpha1(i)?;
    Ok((i, Cave { name }))
}

fn edge(i: &str) -> IResult<&str, Edge> {
    let (i, cave0) = cave(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, cave1) = cave(i)?;
    Ok((i, Edge { cave0, cave1 }))
}

fn input(i: &str) -> IResult<&str, Vec<Edge>> {
    separated_list1(line_ending, edge)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for edge in &input {
    //     println!("{}", edge);
    // }

    let mut adjacent_caves = HashMap::new();
    for edge in &input {
        let cave0 = edge.cave0.clone();
        let cave1 = edge.cave1.clone();
        // We can never move back to start, so remove it here already.
        if !cave1.is_start() {
            let e = adjacent_caves.entry(cave0).or_insert_with(Vec::new);
            e.push(cave1);
        }

        let cave0 = edge.cave0.clone();
        let cave1 = edge.cave1.clone();
        if !cave0.is_start() {
            let e = adjacent_caves.entry(cave1).or_insert_with(Vec::new);
            e.push(cave0);
        }
    }

    let mut count = 0;
    {
        // Do a depth-first search of all possible paths.
        let mut stack = Vec::new();
        let start = Cave::start();
        let start_adjacents = adjacent_caves[&start].clone();
        stack.push((start.clone(), start_adjacents));

        let mut small_visited = HashSet::new();
        small_visited.insert(start);

        while let Some((cave, mut next_caves)) = stack.pop() {
            if let Some(next_cave) = next_caves.pop() {
                stack.push((cave.clone(), next_caves));
                if next_cave.is_end() {
                    // We reached the end, count but don't explore further.
                    count += 1;
                } else if small_visited.contains(&next_cave) {
                    // We already visited this small cave, don't explore it again.
                } else {
                    // Explore further
                    if next_cave.is_small() {
                        small_visited.insert(next_cave.clone());
                    }
                    let next_adjacents = adjacent_caves[&next_cave].clone();
                    stack.push((next_cave, next_adjacents));
                }
            } else {
                // Nothing more to explore here, back out.
                small_visited.remove(&cave);
            }
        }
    }
    let result_a = count;

    let mut count = 0;
    {
        // Do a depth-first search of all possible paths.
        let mut stack = Vec::new();
        let start = Cave::start();
        let start_adjacents = adjacent_caves[&start].clone();
        stack.push((start.clone(), start_adjacents));

        let mut small_visited = HashSet::new();
        small_visited.insert(start);
        let mut small_visited_twice = None;

        while let Some((cave, mut next_caves)) = stack.pop() {
            if let Some(next_cave) = next_caves.pop() {
                stack.push((cave.clone(), next_caves));
                if next_cave.is_end() {
                    // We reached the end, count but don't explore further.
                    count += 1;
                } else if small_visited.contains(&next_cave) && small_visited_twice != None {
                    // We already visited this small cave,
                    // and we also visited some small cave twice.
                    // So we can't explore this cave again.
                } else {
                    // Explore further
                    if next_cave.is_small() {
                        if small_visited.contains(&next_cave) {
                            small_visited_twice = Some(next_cave.clone());
                        } else {
                            small_visited.insert(next_cave.clone());
                        }
                    }
                    let next_adjacents = adjacent_caves[&next_cave].clone();
                    stack.push((next_cave, next_adjacents));
                }
            } else {
                // Nothing more to explore here, back out.
                if Some(cave.clone()) == small_visited_twice {
                    small_visited_twice = None;
                } else {
                    small_visited.remove(&cave);
                }
            }
        }
    }
    let result_b = count;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
