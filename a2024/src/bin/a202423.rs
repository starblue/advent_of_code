use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Computer {
    name: String,
}

impl Computer {
    fn new(name: String) -> Computer {
        Computer { name }
    }
}
impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn computer(i: &str) -> IResult<&str, Computer> {
    let (i, cs) = recognize(many1(satisfy(|c| c.is_ascii_alphabetic())))(i)?;
    Ok((i, Computer::new(cs.to_string())))
}

#[derive(Clone, Debug)]
struct Connection(Computer, Computer);
impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

fn connection(i: &str) -> IResult<&str, Connection> {
    let (i, c0) = computer(i)?;
    let (i, _) = tag("-")(i)?;
    let (i, c1) = computer(i)?;
    Ok((i, Connection(c0, c1)))
}

#[derive(Clone, Debug)]
struct Input {
    connections: Vec<Connection>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for connection in &self.connections {
            writeln!(f, "{}", connection)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, connections) = separated_list1(line_ending, connection)(i)?;

    Ok((i, Input { connections }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let computers = input
        .connections
        .iter()
        .flat_map(|connection| [&connection.0, &connection.1].into_iter().cloned())
        .collect::<HashSet<_>>();

    let t_computers = computers
        .iter()
        .filter(|c| c.name.starts_with("t"))
        .collect::<Vec<_>>();

    let mut computer_connections = HashMap::new();
    for connection in &input.connections {
        let c0 = &connection.0;
        let c1 = &connection.1;

        let entry0 = computer_connections
            .entry(c0.clone())
            .or_insert_with(Vec::new);
        entry0.push(c1.clone());

        let entry1 = computer_connections
            .entry(c1.clone())
            .or_insert_with(Vec::new);
        entry1.push(c0.clone());
    }

    let mut t3_cliques = HashSet::new();
    for t in &t_computers {
        let cs = &computer_connections[t];
        let len = cs.len();
        for i in 0..(len - 1) {
            let ci = &cs[i];
            let ci_cs = &computer_connections[ci];
            for j in (i + 1)..len {
                let cj = &cs[j];
                if ci_cs.contains(cj) {
                    // We found a clique.
                    let mut clique = [t, ci, cj];
                    clique.sort();
                    t3_cliques.insert(clique);
                }
            }
        }
    }
    let result1 = t3_cliques.len();

    let mut n_cliques = HashSet::new();

    // All connections are 2-cliques.
    for connection in &input.connections {
        let c0 = &connection.0;
        let c1 = &connection.1;
        let mut clique = vec![c0, c1];
        clique.sort();
        n_cliques.insert(clique);
    }
    // Try to expand the cliques as much as possible.
    for _n in 3.. {
        // println!("{}: {}", n, n_cliques.len());

        let mut new_n_cliques = HashSet::new();
        for clique in &n_cliques {
            let c0 = &clique[0];
            for candidate in &computer_connections[c0] {
                let candidate_cs = &computer_connections[candidate];
                if clique[1..].iter().all(|c| candidate_cs.contains(c)) {
                    // We found an n-clique.
                    let mut new_clique = clique.to_vec();
                    new_clique.push(candidate);
                    new_clique.sort();
                    new_n_cliques.insert(new_clique);
                }
            }
        }
        if new_n_cliques.is_empty() {
            break;
        }

        n_cliques = new_n_cliques;
    }
    let maximal_cliques = n_cliques.into_iter().collect::<Vec<_>>();
    if maximal_cliques.len() != 1 {
        return Err("ambiguous maximal clique".into());
    }
    let mut maximal_clique = maximal_cliques[0].clone();
    maximal_clique.sort();
    let names = maximal_clique
        .into_iter()
        .map(|c| &c.name[..])
        .collect::<Vec<_>>();
    let result2 = names.join(",");

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
