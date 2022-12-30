use core::fmt;
use core::hash::Hash;
use core::hash::Hasher;
use core::str::FromStr;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io;

use nom::branch::alt;
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
struct Valve<Id> {
    id: Id,
    flow_rate: i64,
    tunnels_to: Vec<Id>,
}
impl<Id> fmt::Display for Valve<Id>
where
    Id: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Valve {} has flow rate={}; ", self.id, self.flow_rate,)?;
        if self.tunnels_to.len() == 1 {
            write!(f, "tunnel leads to valve ",)?;
        } else {
            write!(f, "tunnels lead to valves ",)?;
        }
        let mut sep = "";
        for t in &self.tunnels_to {
            write!(f, "{}{}", sep, t)?;
            sep = ", ";
        }
        Ok(())
    }
}

fn valve_id(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn valve(i: &str) -> IResult<&str, Valve<String>> {
    let (i, _) = tag("Valve ")(i)?;
    let (i, id) = valve_id(i)?;
    let (i, _) = tag(" has flow rate=")(i)?;
    let (i, flow_rate) = int(i)?;
    let (i, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(i)?;
    let (i, tunnels_to) = separated_list1(tag(", "), valve_id)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Valve {
            id,
            flow_rate,
            tunnels_to,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Valve<String>>> {
    many1(valve)(i)
}

fn distances(valves: &[Valve<usize>]) -> Vec<Vec<i64>> {
    let len = valves.len();
    let mut result = (0..len)
        .map(|_| (0..len).map(|_| i64::MAX).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // Pairs of valves whose distance has been updated.
    let mut stack = Vec::new();
    for i in 0..len {
        result[i][i] = 0;
        stack.push((i, i));
    }
    while let Some((i, j)) = stack.pop() {
        let distance = result[i][j];
        for &k in &valves[i].tunnels_to {
            if distance + 1 < result[k][j] {
                result[k][j] = distance + 1;
                stack.push((k, j));
            }
        }
        for &k in &valves[j].tunnels_to {
            if distance + 1 < result[i][k] {
                result[i][k] = distance + 1;
                stack.push((i, k));
            }
        }
    }
    result
}

#[derive(Clone, Debug)]
struct Context {
    valves: Vec<Valve<usize>>,
    distances: Vec<Vec<i64>>,
    start_id: usize,
    total_flow: i64,
}
impl Context {
    fn new(input: &[Valve<String>]) -> Context {
        let ids = input
            .iter()
            .enumerate()
            .map(|(i, v)| (v.id.clone(), i))
            .collect::<HashMap<_, _>>();
        let valves = input
            .iter()
            .map(|v| Valve {
                id: ids[&v.id],
                flow_rate: v.flow_rate,
                tunnels_to: v
                    .tunnels_to
                    .iter()
                    .map(|s| ids[s])
                    .collect::<Vec<_>>(),
            })
            .collect::<Vec<_>>();
        let distances = distances(&valves);
        let start_id = ids["AA"];
        let total_flow = valves.iter().map(|v| v.flow_rate).sum::<i64>();

        Context {
            valves,
            distances,
            start_id,
            total_flow,
        }
    }
    /// Return the ids of valves with a non-zero flow rate.
    fn useful_valves(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, v) in self.valves.iter().enumerate() {
            if v.flow_rate > 0 {
                result.push(i);
            }
        }
        result
    }
    fn total_flow(&self) -> i64 {
        self.total_flow
    }
    fn distance(&self, id0: usize, id1: usize) -> i64 {
        self.distances[id0][id1]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Worker {
    pos: usize,
    busy_until: i64,
}

#[derive(Clone, Debug)]
struct State<'s> {
    context: &'s Context,
    workers: Vec<Worker>,
    remaining_valves: Vec<usize>,
    remaining_time: i64,
    remaining_flow: i64,
    value: i64,
}
impl<'s> State<'s> {
    fn start(
        context: &'s Context,
        n_workers: usize,
        remaining_time: i64,
    ) -> State<'s> {
        let remaining_valves = context.useful_valves();
        let value = 0;
        let workers = (0..n_workers)
            .map(|_| Worker {
                pos: context.start_id,
                busy_until: remaining_time,
            })
            .collect::<Vec<_>>();
        State {
            context,
            workers,
            remaining_valves,
            remaining_time,
            remaining_flow: context.total_flow(),
            value,
        }
    }
    fn is_final(&self) -> bool {
        self.remaining_valves.is_empty() || self.remaining_time == 0
    }
    fn next_states(&self) -> Vec<State<'s>> {
        let mut result = Vec::new();
        if !self.remaining_valves.is_empty() {
            // There is more work to do.
            if let Some((wi, worker)) = self
                .workers
                .iter()
                .enumerate()
                .find(|(_, w)| w.busy_until >= self.remaining_time)
            {
                // We found an idle worker to do it.
                for &pos in &self.remaining_valves {
                    // The worker is assigned the task to go to the valve and open it.
                    // We already account for its effect on the value now.
                    // Time doesn't progress yet, as more workers may be idle.
                    let valve = &self.context.valves[pos];
                    let remaining_time = self.remaining_time
                        - self.context.distance(worker.pos, pos)
                        - 1;
                    if remaining_time > 0 {
                        let mut workers = self.workers.clone();
                        workers[wi].pos = pos;
                        workers[wi].busy_until = remaining_time;
                        let remaining_valves = self
                            .remaining_valves
                            .iter()
                            .filter(|&&id| id != pos)
                            .copied()
                            .collect::<Vec<_>>();
                        let remaining_flow =
                            self.remaining_flow - valve.flow_rate;
                        let value =
                            self.value + valve.flow_rate * remaining_time;
                        let new_state = State {
                            context: self.context,
                            workers,
                            remaining_valves,
                            remaining_flow,
                            remaining_time: self.remaining_time,
                            value,
                        };
                        result.push(new_state);
                    }
                }
                if result.is_empty() {
                    // Time must have ran out to do anything meaningful.
                    // Make it explicit.
                    let mut new_state = self.clone();
                    new_state.remaining_time = 0;
                    result.push(new_state);
                }
            } else {
                // All workers are busy.
                // Advance time until the first one is ready again.
                let remaining_time =
                    self.workers.iter().map(|w| w.busy_until).max().unwrap();
                let mut new_state = self.clone();
                new_state.remaining_time = remaining_time;
                for worker in &mut new_state.workers {
                    // Set the current remaining time for idle workers
                    // to get a canonical form for the set of seen states.
                    worker.busy_until = worker.busy_until.min(remaining_time);
                }
                result.push(new_state);
            }
        } else {
            // We cannot assign more work so we might as well finish now,
            // as the Work in progress will not change the final value.
        }
        result
    }
    fn limit_value(&self) -> i64 {
        let mut flows = Vec::new();
        for &id in &self.remaining_valves {
            let valve = &self.context.valves[id];
            flows.push(valve.flow_rate);
        }
        flows.sort();
        flows.reverse();
        let mut remaining_time = self.remaining_time - 1;
        let mut remaining_value = 0;
        let mut it = flows.iter();
        'outer: while remaining_time > 0 {
            for worker in &self.workers {
                // Add one because the worker may become available
                // in the middle of the time interval of length two.
                if remaining_time <= worker.busy_until + 1 {
                    if let Some(flow) = it.next() {
                        remaining_value += flow * remaining_time;
                    } else {
                        break 'outer;
                    }
                }
            }
            // A worker can at most open a valve every other cycle, since
            // moving takes at least a minute and opening the valve another.
            remaining_time -= 2;
        }
        self.value + remaining_value
    }
    fn priority(&self) -> i64 {
        self.value + self.limit_value()
    }
}
impl<'s> PartialEq for State<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}
impl<'s> Eq for State<'s> {}
impl<'s> PartialOrd for State<'s> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'s> Ord for State<'s> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}
impl<'s> Hash for State<'s> {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.workers.hash(hasher);
        self.remaining_valves.hash(hasher);
        self.remaining_time.hash(hasher);
        self.remaining_flow.hash(hasher);
        self.value.hash(hasher);
    }
}

fn main() -> util::Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // for valve in &input {
    //     println!("{}", valve);
    // }

    let context = Context::new(input.as_ref());

    let mut max_value = 0;
    {
        let start_state = State::start(&context, 1, 30);
        let mut queue = BinaryHeap::new();
        queue.push(start_state);
        while let Some(state) = queue.pop() {
            if state.is_final() {
                if state.value > max_value {
                    max_value = state.value;
                }
            } else {
                for next_state in state.next_states() {
                    if next_state.limit_value() > max_value {
                        queue.push(next_state);
                    }
                }
            }
        }
    }
    let result1 = max_value;

    let mut max_value = 0;
    {
        let start_state = State::start(&context, 2, 26);
        let mut queue = BinaryHeap::new();
        queue.push(start_state);
        while let Some(state) = queue.pop() {
            if state.is_final() {
                if state.value > max_value {
                    max_value = state.value;
                }
            } else {
                for next_state in state.next_states() {
                    if next_state.limit_value() > max_value {
                        queue.push(next_state);
                    }
                }
            }
        }
    }
    let result2 = max_value;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
