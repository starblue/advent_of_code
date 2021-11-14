const SIZE: usize = 256;
const ROUND_COUNT: usize = 64;

pub struct KnotHashState {
    ns: Vec<u8>,
    pos: usize,
    skip: usize,
}
impl KnotHashState {
    pub fn new() -> Self {
        KnotHashState {
            ns: (0..=255).collect::<Vec<u8>>(),
            pos: 0,
            skip: 0,
        }
    }
    pub fn round(&mut self, lens: &[usize]) {
        for len in lens {
            let mut p1 = self.pos;
            let mut p2 = (self.pos + len - 1) % SIZE;
            let mut rest = len / 2;
            while rest > 0 {
                self.ns.swap(p1, p2);

                p1 = (p1 + 1) % SIZE;
                p2 = (p2 + SIZE - 1) % SIZE;
                rest -= 1;
            }
            self.pos = (self.pos + len + self.skip) % SIZE;
            self.skip = (self.skip + 1) % SIZE;
        }
    }
    /// Returns the check value for day 10 part 1.
    pub fn check_value(&self) -> usize {
        usize::from(self.ns[0]) * usize::from(self.ns[1])
    }
    fn dense_hash(&self) -> Vec<u8> {
        self.ns
            .chunks(16)
            .map(|c| c.iter().fold(0, |a, b| a ^ b))
            .collect::<Vec<u8>>()
    }
}
impl Default for KnotHashState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn knot_hash(input: &[u8]) -> Vec<u8> {
    let mut input = input.iter().map(|&b| b as usize).collect::<Vec<usize>>();
    input.append(&mut vec![17, 31, 73, 47, 23]);

    let mut state = KnotHashState::new();
    for _ in 0..ROUND_COUNT {
        state.round(&input);
    }
    state.dense_hash()
}
