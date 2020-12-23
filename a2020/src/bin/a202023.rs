use core::fmt;

#[derive(Clone, Debug)]
struct Links {
    /// The label of the cup before this one
    prev: usize,
    /// The label of the cup after this one
    next: usize,
}
#[derive(Clone, Debug)]
/// A circular doubly linked list representing the cups.
///
/// The list is represented by a Vec indexed by the cup label.
/// This allows to get to a cup with a specific label,
/// to go a step clockwise in the circle,
/// to delete an element and to add an element each in O(1).
struct Circle {
    min: usize,
    max: usize,
    links: Vec<Links>,
    current: usize,
}
impl Circle {
    /// Create a circle from a vector of cups.
    ///
    /// The first cup in the vector becomes the current cup.
    fn from(cups: &[usize]) -> Circle {
        let &min = cups.iter().min().unwrap();
        let &max = cups.iter().max().unwrap();
        let links = (0..=max)
            .map(|i| Links { prev: i, next: i })
            .collect::<Vec<_>>();
        let current = cups[0];
        let mut circle = Circle {
            min,
            max,
            links,
            current,
        };
        for &cup in &cups[1..] {
            circle.push(cup);
        }
        circle
    }

    /// Add a cup clockwise from a given cup
    fn push_next_to(&mut self, dest: usize, cup: usize) {
        let prev = dest;
        let next = self.links[prev].next;
        let new = cup;

        // hook up the new element
        self.links[new].prev = prev;
        self.links[new].next = next;

        // link the new element into the list
        self.links[prev].next = new;
        self.links[next].prev = new;
    }

    /// Add a cup at the end
    ///
    /// That is, add it just before the current cup in the circle.
    fn push(&mut self, cup: usize) {
        let tail = self.links[self.current].prev;
        self.push_next_to(tail, cup);
    }

    /// Removes and returns the cup after the current cup
    ///
    /// The circle must contain at least two elements.
    fn pop_next(&mut self) -> usize {
        let head = self.current;
        let popd = self.links[head].next;
        let next = self.links[popd].next;

        assert!(head != popd);

        // unlink the removed element
        self.links[head].next = next;
        self.links[next].prev = head;

        // Return the popped value
        popd
    }

    /// Move current cup to next cup in clockwise direction
    fn next(&mut self) {
        self.current = self.links[self.current].next;
    }

    /// Do a move in the game
    fn do_move(&mut self) {
        // pick up three cups clockwise of the current cup
        let mut moved = Vec::new();
        for _ in 0..3 {
            moved.push(self.pop_next());
        }

        // select destination cup
        let mut dest = self.current;
        loop {
            // go one label down
            // wrap around from min to max element without underflow
            dest = if dest > self.min { dest - 1 } else { self.max };

            if !moved.contains(&dest) {
                break;
            }
        }

        // place moved cups clockwise of the destination cup
        for cup in moved {
            self.push_next_to(dest, cup);
            dest = cup;
        }

        // select new current cup
        self.next();
    }

    fn solution_a(&self) -> String {
        let mut s = String::new();
        let mut cup = 1;
        loop {
            cup = self.links[cup].next;
            if cup == 1 {
                break;
            }
            s.push_str(&format!("{}", cup));
        }
        s
    }

    fn solution_b(&self) -> usize {
        let cup0 = 1;
        let cup1 = self.links[cup0].next;
        let cup2 = self.links[cup1].next;

        cup1 * cup2
    }
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cup = self.current;
        let mut sep = "[";
        loop {
            write!(f, "{}{}", sep, cup)?;
            sep = ", ";

            cup = self.links[cup].next;
            if cup == self.current {
                break;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    //let (input, n) = (vec![3, 8, 9, 1, 2, 5, 4, 6, 7], 10);
    let (input, n) = (vec![4, 6, 7, 5, 2, 8, 1, 9, 3], 100);

    let mut circle = Circle::from(&input);
    for _ in 0..n {
        circle.do_move();
    }
    println!("a: {}", circle.solution_a());

    // add cups to get a million
    let max_cup = 1_000_000;
    let mut cups = input.clone();
    let &max_input = input.iter().max().unwrap();
    for cup in (max_input + 1)..=max_cup {
        cups.push(cup);
    }

    let n = 10_000_000;
    let mut circle = Circle::from(&cups);
    for _ in 0..n {
        circle.do_move();
    }
    println!("b: {}", circle.solution_b());
}
