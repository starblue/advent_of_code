use std::fmt;

struct State {
    data: Vec<usize>,
    i1: usize,
    i2: usize,
    needle: Vec<usize>,
    found_pos: Option<usize>,
}

impl State {
    fn next(&mut self) {
        let d1 = self.data[self.i1];
        let d2 = self.data[self.i2];
        let n = d1 + d2;
        if n >= 10 {
            self.push(1);
        }
        self.push(n % 10);

        let len = self.data.len();
        self.i1 = (self.i1 + 1 + d1) % len;
        self.i2 = (self.i2 + 1 + d2) % len;
    }
    fn push(&mut self, d: usize) {
        self.data.push(d);
        if self.found_pos == None && self.data.ends_with(&self.needle) {
            self.found_pos = Some(self.data.len() - self.needle.len());
        }
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
    let needle = vec![2, 6, 0, 3, 2, 1];
    let input = vec![3, 7];

    let data = input.clone();
    let i1 = 0;
    let i2 = 1;

    let mut state = State {
        data,
        i1,
        i2,
        needle,
        found_pos: None,
    };
    //println!("{}", state);
    while state.found_pos == None {
        state.next();
        //println!("{}", state);
    }

    println!("{}", state.found_pos.unwrap());
}
