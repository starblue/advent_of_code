use std::io;
use std::ops;

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn turn_left(&self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("I/O error");

    let a: usize = line.trim().parse().unwrap();

    const HALF: isize = 16;
    const SIZE: usize = 2 * (HALF as usize) + 1;
    let mut s = [[0_usize; SIZE]; SIZE];

    let p0 = Vector { x: HALF, y: HALF };
    s[p0.x as usize][p0.y as usize] = 1;

    let result;

    // start point and direction
    let mut p = p0;
    let mut d = Vector { x: 1, y: 0 };
    loop {
        // go one step
        p += d;

        {
            // direction left
            let d_left = d.turn_left();
            let p_left = p + d_left;
            // if square left is empty turn left
            if s[p_left.x as usize][p_left.y as usize] == 0 {
                d = d_left;
            }
        }

        // compute sum of neighbours
        let mut sum: usize = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                if dx != 0 || dy != 0 {
                    let d1 = Vector { x: dx, y: dy };
                    let p1 = p + d1;
                    sum += s[p1.x as usize][p1.y as usize]
                }
            }
        }
        if sum > a {
            result = sum;
            break;
        }
        s[p.x as usize][p.y as usize] = sum;
    }
    println!("{}", result);
}
