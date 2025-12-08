use core::hash::Hash;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::IntDisjointSets;

#[derive(Clone, Debug)]
pub struct DisjointSets<T>
where
    T: Clone + Eq + Hash,
{
    values: Vec<T>,
    reprs: HashMap<T, usize>,
    disjoint_sets: IntDisjointSets,
}
impl<T> DisjointSets<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        DisjointSets {
            values: Vec::new(),
            reprs: HashMap::new(),
            disjoint_sets: IntDisjointSets::new(),
        }
    }
    /// Adds an element which is its own equivalence class
    pub fn add(&mut self, t: T) {
        if let Entry::Vacant(e) = self.reprs.entry(t.clone()) {
            self.values.push(t);
            let id = self.disjoint_sets.add();
            e.insert(id);
        }
    }
    pub fn contains(&self, t: &T) -> bool {
        self.reprs.contains_key(t)
    }
    /// Returns the representative for the equivalence class of an element.
    pub fn find(&self, t: &T) -> &T {
        let id = self.reprs[t];
        let repr_id = self.disjoint_sets.find(id);
        &self.values[repr_id]
    }
    /// Combines the equivalence classes of two elements into one.
    pub fn union(&mut self, t0: &T, t1: &T) {
        let id0 = self.reprs[t0];
        let id1 = self.reprs[t1];
        self.disjoint_sets.union(id0, id1);
    }
    // Return the size of a disjoint set.
    pub fn set_size(&self, t: &T) -> usize {
        let id = self.reprs[t];
        self.disjoint_sets.set_size(id)
    }
    // Returns the representatives of disjoint sets.
    pub fn set_reprs(&self) -> HashSet<&T> {
        self.values
            .iter()
            .map(|t| self.find(t))
            .collect::<HashSet<_>>()
    }
    /// Returns the number of disjoint sets.
    pub fn set_count(&self) -> usize {
        self.disjoint_sets.set_count()
    }
}
impl<T> Default for DisjointSets<T>
where
    T: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
