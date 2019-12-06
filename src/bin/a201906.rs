use std::collections::HashMap;
use std::io;
use std::io::Read;

use nom::*;

#[derive(Clone, Copy, Debug)]
struct Link {
    dir: usize,
    steps: usize,
}

named!(object<&str, String>,
    map!(recognize!(many1!(alphanumeric1)), String::from)
);

named!(
    orbit<&str, (String, String)>,
    do_parse!(
        obj0: object >>
        char!(')') >>
        obj1: object >> ((obj0, obj1)))
);

named!(
    input<&str, Vec<(String, String)>>,
    many1!(
        do_parse!(
            orbit: orbit >>
            line_ending >> (orbit)
        )
    )
);

fn orbit_count(m: &HashMap<String, String>, a: &str) -> i64 {
    let mut a = a;
    let mut count = 0;
    while let Some(b) = m.get(a) {
        a = b;
        count += 1;
    }
    count
}

fn orbit_path(m: &HashMap<String, String>, a: &str) -> Vec<String> {
    let mut a = a;
    let mut path = Vec::new();
    while let Some(b) = m.get(a) {
        a = b;
        path.push(a.to_string());
    }
    path.reverse();
    path
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push_str("\n");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let orbits = result.unwrap().1;

    // compute transitive closure
    let orbit_map: HashMap<String, String> = orbits
        .iter()
        .map(|(a, b)| (b.clone(), a.clone()))
        .collect::<HashMap<_, _>>();
    let mut count = 0;
    for a in orbit_map.keys() {
        count += orbit_count(&orbit_map, a);
    }

    let you_path = orbit_path(&orbit_map, "YOU");
    let san_path = orbit_path(&orbit_map, "SAN");
    let common_prefix = you_path
        .iter()
        .zip(san_path.iter())
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect::<Vec<_>>();
    let you_len = you_path.len() - common_prefix.len();
    let san_len = san_path.len() - common_prefix.len();
    
    let result_a = count;
    let result_b = you_len + san_len;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
