use std::iter::repeat;

const SERIAL: i64 = 2568;

fn power_level(serial: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let pl1 = rack_id * y + serial;
    let pl2 = pl1 * rack_id;
    let digit = (pl2 / 100) % 10;
    digit - 5
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Data {
    sum: i64,
    x: i64,
    y: i64,
    size: i64,
}

fn max_square(serial: i64) -> Data {
    let size = 300;
    let mut sums = repeat(repeat(0).take(size + 1).collect::<Vec<_>>())
        .take(size + 1)
        .collect::<Vec<_>>();

    for x in 0..size {
        for y in 0..size {
            sums[y + 1][x + 1] = sums[y][x + 1] + sums[y + 1][x] - sums[y][x]
                + power_level(serial, (x as i64) + 1, (y as i64) + 1);
        }
    }

    let mut max_sum = std::i64::MIN;
    let mut max_data = None;
    for x0 in 0..(size - 1) {
        for y0 in 0..(size - 1) {
            let s_limit = (size - x0).min(size - y0);
            for s in 1..s_limit {
                let x1 = x0 + s;
                let y1 = y0 + s;
                let sum = sums[y1][x1] - sums[y0][x1] - sums[y1][x0] + sums[y0][x0];
                if sum > max_sum {
                    max_sum = sum;
                    // top left in one-based coordinates
                    max_data = Some(Data {
                        sum: sum,
                        x: (x0 + 1) as i64,
                        y: (y0 + 1) as i64,
                        size: s as i64,
                    })
                }
            }
        }
    }
    max_data.unwrap()
}

fn main() {
    let result = max_square(SERIAL);
    println!("{}", result.sum);

    println!("{},{},{}", result.x, result.y, result.size);
}

#[cfg(test)]
mod test {
    use crate::max_square;
    use crate::power_level;
    use crate::Data;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(8, 3, 5), 4);
        assert_eq!(power_level(57, 122, 79), -5);
        assert_eq!(power_level(39, 217, 196), 0);
        assert_eq!(power_level(71, 101, 153), 4);
    }
    #[test]
    fn test_max_square() {
        assert_eq!(
            max_square(18),
            Data {
                sum: 113,
                x: 90,
                y: 269,
                size: 16
            }
        );
        assert_eq!(
            max_square(42),
            Data {
                sum: 119,
                x: 232,
                y: 251,
                size: 12
            }
        );
    }
}
