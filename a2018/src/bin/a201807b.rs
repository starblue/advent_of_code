use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io;

use nom::character::complete::alpha1;
use nom::do_parse;
use nom::named;
use nom::tag;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Before(char, char);

named!(record<&str, Before>,
       do_parse!(
           tag!("Step ")
               >>step0: alpha1
               >> tag!(" must be finished before step ")
               >>step1: alpha1
               >> tag!(" can begin.")
               >> (Before(step0.chars().next().unwrap(), step1.chars().next().unwrap()))
       )
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Done {
    time: i32,
    step: char,
}

impl PartialOrd for Done {
    fn partial_cmp(&self, other: &Done) -> Option<Ordering> {
        other.time.partial_cmp(&self.time)
    }
}

impl Ord for Done {
    fn cmp(&self, other: &Done) -> Ordering {
        other.time.cmp(&self.time)
    }
}

fn duration(step: char) -> i32 {
    60 + u32::from(step) as i32 - 64
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
        let result = record(&line.trim_end());
        //println!("{:?}", result);

        let r = result.unwrap().1;
        records.push(r);
    }

    let mut steps_to_do = HashSet::new();
    for Before(s0, s1) in &records {
        steps_to_do.insert(*s0);
        steps_to_do.insert(*s1);
    }

    let mut done_times = BinaryHeap::new();
    let mut current_time = 0;
    while !steps_to_do.is_empty() || !done_times.is_empty() {
        let not_ready_steps = records
            .iter()
            .map(|&Before(_s0, s1)| s1)
            .collect::<HashSet<_>>();
        let mut ready_steps = steps_to_do
            .difference(&not_ready_steps)
            .cloned()
            .collect::<Vec<_>>();
        ready_steps.sort_unstable();

        for step in ready_steps {
            if done_times.len() >= 5 {
                // no worker ready
                break;
            }
            // start working on the step
            let time = current_time + duration(step);
            done_times.push(Done { time, step });

            steps_to_do.remove(&step);
        }

        // skip until next step is finished
        let Done { time, step } = done_times.pop().unwrap();
        current_time = time;

        // remove order conditions that are now satisfied
        records = records
            .into_iter()
            .filter(move |&Before(s0, _)| s0 != step)
            .collect();
    }

    println!("{}", current_time);
}
