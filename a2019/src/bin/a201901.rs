use std::io;

fn fuel_a(mass: isize) -> isize {
    mass / 3 - 2
}

fn fuel_b(mass: isize) -> isize {
    let mut sum = 0;

    let mut mass = mass;
    loop {
        let fuel = fuel_a(mass);
        if fuel <= 0 {
            break;
        }
        sum += fuel;
        mass = fuel;
    }
    sum
}

fn main() {
    let mut line = String::new();

    let mut sum_a = 0;
    let mut sum_b = 0;
    loop {
        line.clear();
        io::stdin().read_line(&mut line).expect("I/O error");

        let mass: isize = match line.trim().parse() {
            Result::Ok(mass) => mass,
            Result::Err(_) => break,
        };
        sum_a += fuel_a(mass);
        sum_b += fuel_b(mass);
    }
    println!("a: {}", sum_a);
    println!("b: {}", sum_b);
}
