fn accepted_a(n: i64) -> bool {
    let mut n = n;
    let mut equal_found = false;
    let mut last_d = 10;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        if d > last_d {
            return false;
        } else if d == last_d {
            equal_found = true;
        } else {
            // d < last_d, OK
        }
        last_d = d;
    }
    equal_found
}

fn accepted_b(n: i64) -> bool {
    let mut n = n;
    let mut equal_found = false;
    let mut last_d = 10;
    let mut equal_count = 1;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        if d > last_d {
            return false;
        } else if d == last_d {
            equal_count += 1;
        } else {
            if equal_count == 2 {
                equal_found = true;
            }
            equal_count = 1;
        }
        last_d = d;
    }
    if equal_count == 2 {
        equal_found = true;
    }
    equal_found
}

fn main() {
    let (a, b) = (138307, 654504);
    let mut count_a = 0;
    for n in a..=b {
        if accepted_a(n) {
            count_a += 1;
        }
    }
    let mut count_b = 0;
    for n in a..=b {
        if accepted_b(n) {
            count_b += 1;
        }
    }

    let result_a = count_a;
    let result_b = count_b;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
