use std::collections::HashMap;
use std::io;

use gamedim::p2d;
use gamedim::v2d;
use gamedim::Point2d;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let input = line.trim().parse().unwrap();

    let mut grid: HashMap<Point2d, i64> = HashMap::new();

    let p0 = p2d(0, 0);
    grid.insert(p0, 1);

    let result;

    // start point and direction
    let mut p = p0;
    let mut d = v2d(1, 0);
    loop {
        // go one step
        p += d;

        {
            // direction left
            let d_left = d.rotate_left();
            let p_left = p + d_left;
            // if square left is empty turn left
            if !grid.contains_key(&p_left) {
                d = d_left;
            }
        }

        // compute sum of neighbours
        let sum = p
            .neighbors_l_infty()
            .iter()
            .flat_map(|np| grid.get(np))
            .sum();
        grid.insert(p, sum);

        if sum > input {
            result = sum;
            break;
        }
    }
    println!("{}", result);
}
