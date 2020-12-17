const SERIAL: i64 = 2568;

fn power_level(x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let pl1 = rack_id * y + SERIAL;
    let pl2 = pl1 * rack_id;
    let digit = (pl2 / 100) % 10;
    digit - 5
}

fn main() {
    let size = 300;
    let mut max_sum = std::i64::MIN;
    let mut max_xy = None;
    for x in 0..(size - 3) {
        for y in 0..(size - 3) {
            let mut sum = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    sum += power_level(x + dx, y + dy);
                }
            }
            if sum > max_sum {
                max_sum = sum;
                max_xy = Some((x, y));
            }
        }
    }
    let (x, y) = max_xy.unwrap();
    println!("{},{}", x, y);
}
