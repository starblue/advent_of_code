use std::collections::btree_set::BTreeSet;
use std::io;

fn main() {
    let mut line = String::new();

    let mut parents = BTreeSet::new();
    let mut children = BTreeSet::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let words = line
            .split(|c| c == ' ' || c == ',' || c == '\n')
            .filter(|w| w != &"")
            .map(|w| w.to_owned())
            .collect::<Vec<_>>();
        if words.len() < 2 {
            break;
        }

        let (ws0, ws1) = words.split_at(2);
        let parent = ws0[0].clone();
        parents.insert(parent);
        if ws1.len() > 1 {
            for w in ws1[1..].iter() {
                children.insert(w.clone());
            }
        }
    }
    let diff = parents.difference(&children).cloned().collect::<Vec<_>>();
    println!("{}", diff[0]);
}
