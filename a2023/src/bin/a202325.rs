use core::fmt;
use core::iter::once;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use rand::seq::SliceRandom;
use rand::Rng;

use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;

use util::IntDisjointSets;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Label<'a>(&'a str);
impl<'a> fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
fn label(i: &str) -> IResult<&str, Label> {
    map(alpha1, Label)(i)
}

#[derive(Clone, Debug)]
struct Component<'a> {
    name: Label<'a>,
    connected_names: Vec<Label<'a>>,
}
impl<'a> fmt::Display for Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:", self.name)?;
        for name in &self.connected_names {
            write!(f, " {}", name)?;
        }
        Ok(())
    }
}
fn component(i: &str) -> IResult<&str, Component<'_>> {
    let (i, name) = label(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, connected_names) = separated_list1(tag(" "), label)(i)?;
    Ok((i, Component { name, connected_names }))
}

#[derive(Clone, Debug)]
struct Input<'a> {
    components: Vec<Component<'a>>,
}
impl<'a> Input<'a> {
    fn new(components: Vec<Component<'a>>) -> Input<'a> {
        Input { components }
    }
}
impl<'a> fmt::Display for Input<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for component in &self.components {
            writeln!(f, "{}", component)?;
        }
        Ok(())
    }
}
fn input(i: &str) -> IResult<&str, Input<'_>> {
    map(separated_list1(line_ending, component), Input::new)(i)
}

#[derive(Clone, Debug)]
struct Graph {
    edges: Vec<(usize, usize)>,
    /// Disjoint sets used to merge nodes into supernodes.
    node_disjoint_sets: IntDisjointSets,
    supernode_count: usize,
}
impl Graph {
    fn new(input: &Input) -> Graph {
        let node_names = input
            .components
            .iter()
            .flat_map(|c| once(&c.name).chain(c.connected_names.iter()))
            .cloned()
            .collect::<HashSet<_>>();
        let mut node_names = node_names.into_iter().collect::<Vec<_>>();
        node_names.sort();

        let node_indices = node_names
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect::<HashMap<_, _>>();

        let mut edges = Vec::new();
        for component in &input.components {
            let name_a = &component.name;
            let a = node_indices[&name_a];
            for name_b in &component.connected_names {
                let b = node_indices[&name_b];
                edges.push((a, b));
            }
        }
        let mut node_disjoint_sets = IntDisjointSets::new();
        for (i, _) in node_names.iter().enumerate() {
            let ni = node_disjoint_sets.add();
            assert_eq!(i, ni);
        }
        let supernode_count = node_names.len();
        Graph { edges, node_disjoint_sets, supernode_count }
    }
    /// Returns representatives for distinct supernodes.
    fn supernodes(&self) -> HashSet<usize> {
        self.node_disjoint_sets.set_reprs()
    }
    /// Returns the number of nodes in a supernode.
    fn supernode_len(&self, n: usize) -> usize {
        let r = self.node_disjoint_sets.find(n);
        self.node_disjoint_sets.set_size(r)
    }
    /// Returns edges between distinct supernodes.
    /// There may be multiple edges between the same two supernodes.
    fn edges(&self) -> Vec<(usize, usize)> {
        self.edges
            .iter()
            .filter(|(ni0, ni1)| !self.unioned(*ni0, *ni1))
            .cloned()
            .collect::<Vec<_>>()
    }
    /// Combines two supernodes into one.
    fn union(&mut self, ni0: usize, ni1: usize) {
        if !self.unioned(ni0, ni1) {
            self.node_disjoint_sets.union(ni0, ni1);
            self.supernode_count -= 1;
        }
    }
    fn unioned(&self, ni0: usize, ni1: usize) -> bool {
        let r0 = self.node_disjoint_sets.find(ni0);
        let r1 = self.node_disjoint_sets.find(ni1);
        r0 == r1
    }
    fn supernode_count(&self) -> usize {
        self.supernode_count
    }
}

// See Sariel Har-Peled, "Minimum Cut in a Graph", September 3, 2002.

#[derive(Clone, Copy, Debug, Default)]
struct Cut {
    /// Number of nodes in the first supernode.
    node_count0: usize,
    /// Number of nodes in the second supernode.
    node_count1: usize,
    /// Number of edges between the supernodes.
    edge_count: usize,
}

fn contract<R: Rng>(g: &mut Graph, rng: &mut R, t: usize) {
    while g.supernode_count() > t {
        let &(n0, n1) = g.edges().choose(rng).unwrap();
        g.union(n0, n1);
    }
}

fn fast_cut<R: Rng>(g: &Graph, rng: &mut R) -> Cut {
    assert!(g.supernode_count() >= 2);
    if g.supernode_count() <= 6 {
        let supernodes = g.supernodes().iter().cloned().collect::<Vec<_>>();
        let len = supernodes.len();
        let mut min_edge_count = usize::MAX;
        let mut min_cut = Cut::default();
        for i in 1..(1 << (len - 1)) {
            let mut g0 = g.clone();
            let mut ni0s = Vec::new();
            let mut ni1s = Vec::new();
            for (j, &s) in supernodes.iter().enumerate() {
                if i & (1 << j) == 0 {
                    ni0s.push(s);
                } else {
                    ni1s.push(s);
                }
            }
            for j in 1..ni0s.len() {
                g0.union(ni0s[0], ni0s[j]);
            }
            for j in 1..ni1s.len() {
                g0.union(ni1s[0], ni1s[j]);
            }
            let final_supernodes = g0.supernodes();
            let mut it = final_supernodes.iter();
            let &s0 = it.next().unwrap();
            let &s1 = it.next().unwrap();
            let node_count0 = g0.supernode_len(s0);
            let node_count1 = g0.supernode_len(s1);
            let edge_count = g0
                .edges
                .iter()
                .filter(|(n0, n1)| {
                    (g0.unioned(*n0, s0) && g0.unioned(*n1, s1))
                        || (g0.unioned(*n0, s1) && g0.unioned(*n1, s0))
                })
                .count();
            if edge_count < min_edge_count {
                min_edge_count = edge_count;
                min_cut = Cut { node_count0, node_count1, edge_count };
            }
        }
        min_cut
    } else {
        // Target number of supernodes for the recursive calls.
        let t = g.supernode_count() / 2;

        let mut g0 = g.clone();
        contract(&mut g0, rng, t);
        let cut0 = fast_cut(&g0, rng);

        let mut g1 = g.clone();
        contract(&mut g1, rng, t);
        let cut1 = fast_cut(&g1, rng);

        if cut0.edge_count <= cut1.edge_count {
            cut0
        } else {
            cut1
        }
    }
}

fn min_cut(g: &mut Graph) -> Cut {
    let mut rng = rand::thread_rng();
    fast_cut(g, &mut rng)
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let result1 = loop {
        let mut graph = Graph::new(&input);
        let cut = min_cut(&mut graph);
        if cut.edge_count == 3 {
            break cut.node_count0 * cut.node_count1;
        }
    };

    println!("Part 1: {}", result1);

    Ok(())
}
