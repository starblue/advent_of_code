use std::collections::HashSet;
use std::error;
use std::io;

use util::runtime_error;
use util::RuntimeError;

/// Return the Unicode scalar value of a `char`.
fn usv(c: char) -> i64 {
    i64::from(u32::from(c))
}

fn priority(c: char) -> Result<i64, RuntimeError> {
    match c {
        'a'..='z' => Ok(usv(c) - usv('a') + 1),
        'A'..='Z' => Ok(usv(c) - usv('A') + 27),
        _ => Err(runtime_error!("not an item type: '{}' ({:#x})", c, usv(c))),
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = io::read_to_string(io::stdin())?;
    let lines = input.lines().collect::<Vec<_>>();

    let mut sum = 0;
    for line in &lines {
        let h = line.len() / 2;
        let cs0 = line[0..h].chars().collect::<HashSet<char>>();
        let cs1 = line[h..].chars().collect::<HashSet<char>>();
        let common = cs0.intersection(&cs1).copied().collect::<HashSet<char>>();
        let c = common
            .into_iter()
            .next()
            .ok_or_else(|| runtime_error!("no common item type found"))?;
        sum += priority(c)?;
    }
    let result1 = sum;

    let mut sum = 0;
    for group in lines.chunks(3) {
        let cs0 = group[0].chars().collect::<HashSet<char>>();
        let cs1 = group[1].chars().collect::<HashSet<char>>();
        let cs2 = group[2].chars().collect::<HashSet<char>>();
        let common01 = cs0.intersection(&cs1).copied().collect::<HashSet<char>>();
        let common = common01
            .intersection(&cs2)
            .copied()
            .collect::<HashSet<char>>();
        let c = common
            .into_iter()
            .next()
            .ok_or_else(|| runtime_error!("no badge found"))?;
        sum += priority(c)?;
    }
    let result2 = sum;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
