use std::collections::HashSet;

/// Integer or internal disjoint sets
#[derive(Clone, Debug)]
pub struct IntDisjointSets {
    reprs: Vec<usize>,
    sizes: Vec<usize>,
    set_count: usize,
}
impl IntDisjointSets {
    pub fn new() -> Self {
        IntDisjointSets {
            reprs: Vec::new(),
            sizes: Vec::new(),
            set_count: 0,
        }
    }
    /// Adds an element which is its own equivalence class and returns its id.
    pub fn add(&mut self) -> usize {
        let id = self.reprs.len();
        self.reprs.push(id);
        self.sizes.push(1);
        self.set_count += 1;
        id
    }
    /// Returns the representative for the equivalence class of an element.
    ///
    /// Does not do path compression to avoid a mutable reference.
    pub fn find(&self, id: usize) -> usize {
        let mut i = id;
        while self.reprs[i] != i {
            i = self.reprs[i];
        }
        i
    }
    /// Returns the representative for the equivalence class of an element.
    ///
    /// Does path compression during the lookup.
    pub fn find_update(&mut self, id: usize) -> usize {
        let mut i = id;
        while self.reprs[i] != i {
            let prev = i;
            i = self.reprs[i];
            self.reprs[prev] = self.reprs[i];
        }
        i
    }
    /// Combines the equivalence classes of two elements into one.
    pub fn union(&mut self, i: usize, j: usize) {
        let i_repr = self.find_update(i);
        let j_repr = self.find_update(j);
        if i_repr != j_repr {
            let i_size = self.sizes[i_repr];
            let j_size = self.sizes[j_repr];
            if i_size < j_size {
                self.reprs[j_repr] = i;
                self.sizes[i_repr] += j_size;
            } else {
                self.reprs[i_repr] = j;
                self.sizes[j_repr] += i_size;
            }
            self.set_count -= 1;
        }
    }
    // Return the size of a disjoint set.
    pub fn set_size(&self, i: usize) -> usize {
        let ri = self.find(i);
        self.sizes[ri]
    }
    // Returns the representatives of disjoint sets.
    pub fn set_reprs(&self) -> HashSet<usize> {
        let mut result = HashSet::new();
        for &r in &self.reprs {
            let r = self.find(r);
            result.insert(r);
        }
        result
    }
    /// Returns the number of disjoint sets.
    pub fn set_count(&self) -> usize {
        self.set_count
    }
}
impl Default for IntDisjointSets {
    fn default() -> Self {
        Self::new()
    }
}
