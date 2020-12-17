use std::collections::HashMap;

fn play(input: Vec<usize>, target_turn: usize) -> usize {
    let mut turn = 1;
    let mut last_seen = HashMap::new();
    let mut previous_n = 0;
    let result;
    loop {
        let n;
        if turn - 1 < input.len() {
            n = input[turn - 1];
        } else {
            if let Some(turn0) = last_seen.get(&previous_n) {
                n = turn - 1 - turn0;
            } else {
                n = 0;
            }
        }

        if turn == target_turn {
            result = n;
            break;
        }

        last_seen.insert(previous_n, turn - 1);
        previous_n = n;

        turn += 1;
    }
    result
}

fn main() {
    let result_a = play(vec![1, 0, 15, 2, 10, 13], 2020);
    let result_b = play(vec![1, 0, 15, 2, 10, 13], 30_000_000);
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
