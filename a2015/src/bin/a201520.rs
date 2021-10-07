use jmath_factor::Factorizer;
use jmath_factor::TableFactorizer;

fn main() {
    let input = 29000000;

    let limit = input / 10;

    let f = &TableFactorizer::new(limit as usize);
    let mut n = 1;
    while f.divisor_sum(n) < limit {
        n += 1;
    }
    let result_a = n;

    let mut n = 1;
    loop {
        let mut sum = 0;
        for d in f.divisors(n) {
            if d >= n / 50 {
                sum += d * 11;
            }
        }
        if sum >= input {
            break;
        }
        n += 1;
    }
    let result_b = n;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
