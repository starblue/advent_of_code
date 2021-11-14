use std::collections::HashMap;

use lowdim::p2d;
use lowdim::v2d;

use util::knot_hash;
use util::DisjointSets;

fn main() {
    let input = "wenycdww";

    let mut count = 0;
    for i in 0..128 {
        let s = format!("{}-{}", input, i);
        let hash = knot_hash(s.as_bytes());
        for b in hash {
            count += b.count_ones();
        }
    }
    let result_a = count;

    // Internal disjoint set representatives for points in a region.
    let mut reprs = HashMap::new();
    // Regions are represented as disjoint sets.
    let mut regions = DisjointSets::new();

    for i in 0..128 {
        let s = format!("{}-{}", input, i);
        let hash = knot_hash(s.as_bytes());
        for j in 0..128 {
            let b = hash[usize::try_from(j / 8).unwrap()];
            // The bit index is most significant bit first.
            let bit_index = 7 - (j % 8);
            if (b & (1 << bit_index)) != 0 {
                let p = p2d(i, j);
                let id = regions.add();
                reprs.insert(p, id);

                let p_left = p - v2d(1, 0);
                if let Some(id_left) = reprs.get(&p_left) {
                    regions.union(id, *id_left);
                }
                let p_up = p - v2d(0, 1);
                if let Some(id_up) = reprs.get(&p_up) {
                    regions.union(id, *id_up);
                }
            }
        }
    }
    let result_b = regions.set_reprs().len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
