use std::collections::btree_map::BTreeMap;
use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("I/O error");

    let mut state: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let len = state.len();

    let mut count = 0;
    let mut seen = BTreeMap::new();
    let result;
    loop {
        if seen.contains_key(&state) {
            result = count - seen[&state];
            break;
        } else {
            seen.insert(state.clone(), count);
        }

        // compute new state
        let mut i;
        let mut value;
        {
            let p = state
                .iter()
                .enumerate()
                .max_by(|&(i1, v1), &(i2, v2)| v1.cmp(&v2).then(i2.cmp(&i1)))
                .unwrap();
            i = p.0;
            value = *p.1;
        }

        state[i] = 0;
        while value > 0 {
            i = (i + 1) % len;
            state[i] += 1;
            value -= 1;
        }
        count += 1;
    }
    println!("{}", result);
}
