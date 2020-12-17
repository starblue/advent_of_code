use std::collections::btree_set::BTreeSet;
use std::io;

fn main() {
    let mut line = String::new();

    let mut count = 0;
    loop {
        line.clear();
        let num_bytes = io::stdin().read_line(&mut line).expect("I/O error");

        if num_bytes == 0 {
            break;
        }

        let mut words = BTreeSet::new();
        let mut valid = true;
        for w in line.split_whitespace() {
            if !words.contains(w) {
                words.insert(w);
            } else {
                valid = false;
            }
        }
        if valid {
            count += 1;
        }
    }
    println!("{}", count);
}
