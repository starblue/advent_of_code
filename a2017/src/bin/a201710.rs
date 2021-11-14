use std::io;

use a2017::knot_hash;
use a2017::KnotHashState;

fn bytes_to_string(input: &[u8]) -> String {
    let mut result = String::new();
    for b in input {
        result.push_str(&format!("{:02x}", b));
    }
    result
}

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("I/O error");
    let lens = line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let mut state = KnotHashState::new();
    state.round(&lens);
    let result_a = state.check_value();

    let input = line
        .chars()
        .take_while(|c| *c != '\n')
        .map(|c| c as u8)
        .collect::<Vec<u8>>();
    let xs = knot_hash(&input);
    let result_b = bytes_to_string(&xs);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod test {
    use crate::bytes_to_string;
    use crate::knot_hash;

    #[test]
    fn test_empty() {
        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272".to_string(),
            bytes_to_string(&knot_hash(b""))
        );
    }
    #[test]
    fn test_aoc_2017() {
        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd".to_string(),
            bytes_to_string(&knot_hash(b"AoC 2017"))
        );
    }
    #[test]
    fn test_1_2_3() {
        assert_eq!(
            "3efbe78a8d82f29979031a4aa0b16a9d".to_string(),
            bytes_to_string(&knot_hash(b"1,2,3"))
        );
    }
    #[test]
    fn test_1_2_4() {
        assert_eq!(
            "63960835bcdc130f0b66d7ff4f6a5a8e".to_string(),
            bytes_to_string(&knot_hash(b"1,2,4"))
        );
    }
}
