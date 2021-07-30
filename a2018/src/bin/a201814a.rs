use std::fmt;

struct State {
    data: Vec<usize>,
    i1: usize,
    i2: usize,
}

impl State {
    fn next(&mut self) {
        let d1 = self.data[self.i1];
        let d2 = self.data[self.i2];
        let n = d1 + d2;
        if n >= 10 {
            self.data.push(1);
        }
        self.data.push(n % 10);

        let len = self.data.len();
        self.i1 = (self.i1 + 1 + d1) % len;
        self.i2 = (self.i2 + 1 + d2) % len;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, &s) in self.data.iter().enumerate() {
            write!(
                f,
                "{}",
                if i == self.i1 {
                    "("
                } else if i == self.i2 {
                    "["
                } else {
                    " "
                }
            )?;
            write!(f, "{}", s)?;
            write!(
                f,
                "{}",
                if i == self.i1 {
                    ")"
                } else if i == self.i2 {
                    "]"
                } else {
                    " "
                }
            )?;
        }
        Ok(())
    }
}

fn main() {
    let n = 260321;
    let input = vec![3, 7];
    let output_len = 10;

    let skip = n;

    let data = input;
    let i1 = 0;
    let i2 = 1;

    let mut state = State { data, i1, i2 };
    //println!("{}", state);
    while state.data.len() < skip + output_len {
        state.next();
        //println!("{}", state);
    }

    for d in state.data.iter().skip(skip).take(output_len) {
        print!("{}", d);
    }
    println!();
}
