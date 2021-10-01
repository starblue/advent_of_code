fn look_and_say(s: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    let mut iter = s.iter();
    let mut count = 1;
    if let Some(&first) = iter.next() {
        let mut previous = first;
        for &d in iter {
            if d == previous {
                count += 1;
            } else {
                result.push(count);
                result.push(previous);
                previous = d;
                count = 1;
            }
        }
        result.push(count);
        result.push(previous);
    }
    result
}

fn main() {
    let input = "1113122113";

    let mut sequence = input
        .chars()
        .map(|c| c.to_digit(10).unwrap().into())
        .collect::<Vec<i64>>();
    for _ in 0..40 {
        sequence = look_and_say(&sequence);
    }

    let result_a = sequence.len();

    for _ in 0..10 {
        sequence = look_and_say(&sequence);
    }
    let result_b = sequence.len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
