use std::collections::HashSet;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::Point2d;

fn is_open(p: Point2d, n: i64) -> bool {
    let x = p.x();
    let y = p.y();
    let sum = x * x + 3 * x + 2 * x * y + y + y * y + n;
    x >= 0 && y >= 0 && sum.count_ones() % 2 == 0
}

fn main() {
    let input = 1350;

    let start_node = p2d(1, 1);
    let target_node = p2d(31, 39);
    let successors = |n: &Point2d| {
        n.neighbors_l1()
            .into_iter()
            .filter(|&p| is_open(p, input))
            .map(|p| (p, 1))
    };
    let heuristic = |n: &Point2d| n.distance_l1(target_node);
    let success = |n: &Point2d| n == &target_node;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.unwrap();
    let result_a = cost;

    let mut old = HashSet::new();
    let mut current = HashSet::new();
    current.insert(start_node);
    for _ in 0..50 {
        let mut new = HashSet::new();
        for n in &current {
            for n1 in n.neighbors_l1() {
                if is_open(n1, input) && !old.contains(&n1) && !current.contains(&n1) {
                    new.insert(n1);
                }
            }
        }
        for n in current {
            old.insert(n);
        }
        current = new;
    }
    for n in current {
        old.insert(n);
    }
    let result_b = old.len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
