use std::io;
use std::io::Read;

use json::JsonValue;

fn number_sum1(json: &JsonValue) -> f64 {
    if json.is_number() {
        json.as_f64().unwrap()
    } else if json.is_object() {
        json.entries().map(|(_, v)| number_sum1(v)).sum()
    } else if json.is_array() {
        json.members().map(number_sum1).sum()
    } else {
        0.0
    }
}

fn number_sum2(json: &JsonValue) -> f64 {
    if json.is_number() {
        json.as_f64().unwrap()
    } else if json.is_object() {
        if !json.entries().any(|(_, v)| v == "red") {
            json.entries().map(|(_, v)| number_sum2(v)).sum()
        } else {
            0.0
        }
    } else if json.is_array() {
        json.members().map(number_sum2).sum()
    } else {
        0.0
    }
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    let parsed = json::parse(&input_data);
    //println!("{:?}", parsed);

    let json = parsed.unwrap();

    let result_a = number_sum1(&json);

    let result_b = number_sum2(&json);

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
