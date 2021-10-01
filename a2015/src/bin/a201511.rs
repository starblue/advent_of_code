use std::collections::HashSet;
use std::convert::TryFrom;

fn next_char(c: char) -> Option<char> {
    let next = char::try_from(u32::from(c) + 1).unwrap();
    if ('a'..='z').contains(&next) {
        Some(next)
    } else {
        None
    }
}

fn next_password(s: &[char]) -> Option<Vec<char>> {
    let mut pw = s.to_vec();
    let mut i = pw.len();
    while i >= 1 {
        if let Some(c) = next_char(pw[i - 1]) {
            pw[i - 1] = c;
            for j in i..8 {
                pw[j] = 'a';
            }
            return Some(pw);
        } else {
            i -= 1;
        }
    }
    None
}

fn is_valid_char(c: char) -> bool {
    c != 'i' && c != 'o' && c != 'l'
}

fn satisfies_rule_1(s: &[char]) -> bool {
    // contains incrementing sequence of length 3
    for i in 2..s.len() {
        let c0 = s[i - 2];
        let c1 = s[i - 1];
        let c2 = s[i];
        if next_char(c0) == Some(c1) && next_char(c1) == Some(c2) {
            return true;
        }
    }
    false
}

fn satisfies_rule_2(s: &[char]) -> bool {
    // contains only valid chars
    s.iter().all(|&c| is_valid_char(c))
}

fn satisfies_rule_3(s: &[char]) -> bool {
    // contains two different pairs of letters
    let mut pair_seen = HashSet::new();
    for i in 1..s.len() {
        let c0 = s[i - 1];
        let c1 = s[i];
        if c0 == c1 {
            pair_seen.insert(c0);
        }
    }
    pair_seen.len() >= 2
}

fn is_valid_password(s: &[char]) -> bool {
    satisfies_rule_1(s) && satisfies_rule_2(s) && satisfies_rule_3(s)
}

fn next_valid_password(s: &[char]) -> Option<Vec<char>> {
    let mut s = s.to_vec();
    loop {
        s = next_password(&s)?;
        if is_valid_password(&s) {
            break;
        }
    }
    Some(s)
}

fn main() {
    let input = "cqjxjnds";
    let password = input.chars().collect::<Vec<char>>();

    let password_a = next_valid_password(&password).unwrap();
    let result_a = password_a.iter().collect::<String>();

    let password_b = next_valid_password(&password_a).unwrap();
    let result_b = password_b.iter().collect::<String>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
