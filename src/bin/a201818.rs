use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::iter::repeat;
use std::iter::repeat_with;

use nom::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    x_size: i64,
    y_size: i64,
    cells: Vec<Vec<char>>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.cells {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum Error {}

named!(cell<&str, char>,
    alt!(
        char!('.') |
        char!('|') |
        char!('#')
    )
);

named!(line<&str, Vec<char>>,
    many1!(cell)
);

named!(cells<&str, Vec<Vec<char>>>,
    many1!(
        do_parse!(
            line: line >>
            line_ending >>
                (line)
        )
    )
);

impl State {
    fn step(&self) -> State {
        let mut cells = repeat_with(|| repeat(' ').take(self.x_size as usize).collect::<Vec<_>>())
            .take(self.y_size as usize)
            .collect::<Vec<_>>();

        for yc in 0..self.y_size {
            for xc in 0..self.x_size {
                let mut tree_count = 0;
                let mut lumber_count = 0;
                let x0 = (xc - 1).max(0);
                let x1 = (xc + 1).min(self.x_size - 1);
                let y0 = (yc - 1).max(0);
                let y1 = (yc + 1).min(self.y_size - 1);
                for y in y0..=y1 {
                    for x in x0..=x1 {
                        if x != xc || y != yc {
                            let c = self.cells[y as usize][x as usize];
                            match c {
                                '|' => {
                                    tree_count += 1;
                                }
                                '#' => {
                                    lumber_count += 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                let cc = self.cells[yc as usize][xc as usize];
                cells[yc as usize][xc as usize] = match cc {
                    '.' => {
                        if tree_count >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        if lumber_count >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    }
                    '#' => {
                        if tree_count >= 1 && lumber_count >= 1 {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    c => c,
                };
            }
        }

        State { cells, ..*self }
    }

    fn value(&self) -> i64 {
        let mut tree_count = 0;
        let mut lumber_count = 0;
        for line in &self.cells {
            for c in line {
                match c {
                    '|' => {
                        tree_count += 1;
                    }
                    '#' => {
                        lumber_count += 1;
                    }
                    _ => {}
                }
            }
        }
        tree_count * lumber_count
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push_str("\n");

    // parse input
    let result = cells(&input_data);
    //println!("{:?}", result);

    let cells = result.unwrap().1;
    let x_size = cells[0].len() as i64;
    let y_size = cells.len() as i64;

    let mut state = State {
        x_size,
        y_size,
        cells,
    };

    //println!("{}", state);
    let mut minute = 0;
    let mut value10 = 0;
    let mut old_states = HashMap::new();
    let mut target_minute = None;
    loop {
        state = state.step();
        minute += 1;
        if minute == 10 {
            value10 = state.value();
        }

        match target_minute {
            None => {
                if let Some(previous_minute) = old_states.get(&state) {
                    // we found a loop

                    let period = minute - previous_minute;
                    target_minute = Some(minute + (1_000_000_000 - minute) % period);
                    //println!("minute: {},  period: {}", minute, period);
                }
            }
            Some(m) => {
                if minute == m {
                    break;
                }
            }
        }
        old_states.insert(state.clone(), minute);

        //println!("Minute {}", minute);
        //println!("{}", state);
    }

    println!("1: {}", value10);
    println!("2: {}", state.value());
}
