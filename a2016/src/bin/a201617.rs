use lowdim::bb2d;
use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    pos: Point2d,
    path: String,
}
impl Node {
    fn hash(&self, passcode: &str) -> Vec<bool> {
        let md5_input = passcode.to_string() + &self.path;
        let md5_input = md5_input.as_bytes();
        let digest = md5::compute(md5_input);
        let s = format!("{:x}", digest);
        s.chars().take(4).map(|c| c >= 'b').collect::<Vec<bool>>()
    }
}

fn main() {
    let input = "qtetzkpl";

    let bbox = bb2d(0..4, 0..4);
    let moves: [(&str, Vec2d<i64>); 4] = [
        ("U", v2d(0, 1)),
        ("D", v2d(0, -1)),
        ("L", v2d(-1, 0)),
        ("R", v2d(1, 0)),
    ];

    let start_node = Node {
        pos: p2d(0, 3),
        path: String::new(),
    };
    let target_pos = p2d(3, 0);
    let successors = |n: &Node| {
        moves
            .iter()
            .zip(n.hash(input))
            .filter_map(|((c, v), b)| {
                let pos = n.pos + v;
                if bbox.contains(pos) && b {
                    let path = n.path.clone() + c;
                    Some((Node { pos, path }, 1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    let heuristic = |n: &Node| n.pos.distance_l1(target_pos);
    let success = |n: &Node| n.pos == target_pos;

    let search_result = astar(&start_node, successors, heuristic, success);
    let (path, _cost) = search_result.unwrap();
    let result_a = &path.last().unwrap().path;

    // Do a depth-first search with explicit stack through all possible paths.
    let mut stack = vec![start_node];
    let mut max_path_len = 0;
    while let Some(node) = stack.pop() {
        if success(&node) {
            // We reached the target room, record path length and finish.
            max_path_len = max_path_len.max(node.path.len());
        } else {
            for (n, _) in successors(&node) {
                stack.push(n);
            }
        }
    }
    let result_b = max_path_len;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
