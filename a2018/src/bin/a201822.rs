use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::IResult;

use pathfinding::prelude::astar;

use lowdim::p2d;
use lowdim::v2d;
use lowdim::BBox2d;
use lowdim::Point2d;
use lowdim::Vec2d;

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn input(i: &str) -> IResult<&str, (i64, Point2d)> {
    let (i, _) = tag("depth: ")(i)?;
    let (i, depth) = int64(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = tag("target: ")(i)?;
    let (i, x) = int64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, (depth, p2d(x, y))))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum ToolState {
    Neither,
    Torch,
    ClimbingGear,
}
impl ToolState {
    fn is_allowed_for(&self, region_type: RegionType) -> bool {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        matches!(
            (region_type, self),
            (
                RegionType::Rocky,
                ToolState::Torch | ToolState::ClimbingGear
            ) | (
                RegionType::Wet,
                ToolState::Neither | ToolState::ClimbingGear
            ) | (
                RegionType::Narrow,
                ToolState::Neither | ToolState::Torch)
        )
    }
}

type Cost = i64;
const MOVE_COST: Cost = 1;
const TOOL_CHANGE_COST: Cost = 7;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node {
    location: Point2d,
    tool_state: ToolState,
}

#[derive(Clone, Copy)]
enum Action {
    Move(Vec2d),
    ToolChange(ToolState),
}
impl Action {
    fn values() -> impl Iterator<Item = Self> {
        vec![
            Action::Move(v2d(1, 0)),
            Action::Move(v2d(0, 1)),
            Action::Move(v2d(-1, 0)),
            Action::Move(v2d(0, -1)),
            Action::ToolChange(ToolState::Neither),
            Action::ToolChange(ToolState::Torch),
            Action::ToolChange(ToolState::ClimbingGear),
        ]
        .into_iter()
    }
    fn apply(&self, n: &Node) -> Node {
        let Node {
            location,
            tool_state,
        } = *n;
        match self {
            Action::Move(v) => Node {
                location: location + v,
                tool_state,
            },
            Action::ToolChange(ts) => Node {
                location,
                tool_state: *ts,
            },
        }
    }
    fn cost(&self) -> Cost {
        match self {
            Action::Move(_) => MOVE_COST,
            Action::ToolChange(_) => TOOL_CHANGE_COST,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

struct Map {
    map: HashMap<Point2d, i64>,
    target: Point2d,
    depth: i64,
}
impl Map {
    fn new(target: Point2d, depth: i64) -> Self {
        Map {
            map: HashMap::new(),
            target,
            depth,
        }
    }
    fn erosion_level(&mut self, p: Point2d) -> i64 {
        if let Some(&erosion_level) = self.map.get(&p) {
            erosion_level
        } else {
            let geologic_index = {
                if p == p2d(0, 0) {
                    0
                } else if p == self.target {
                    0
                } else if p.y() == 0 {
                    p.x() * 16807
                } else if p.x() == 0 {
                    p.y() * 48271
                } else {
                    self.erosion_level(p - v2d(1, 0)) * self.erosion_level(p - v2d(0, 1))
                }
            };
            let erosion_level = (geologic_index + self.depth) % 20183;
            self.map.insert(p, erosion_level);
            erosion_level
        }
    }

    fn risk_level(&mut self, p: Point2d) -> i64 {
        self.erosion_level(p) % 3
    }
    fn region_type(&mut self, p: Point2d) -> RegionType {
        let erosion_level = self.erosion_level(p);
        match erosion_level % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => panic!("unexpected modulus"),
        }
    }
    fn exists(&mut self, n: &Node) -> bool {
        let Node {
            location,
            tool_state,
        } = *n;
        location.x() >= 0
            && location.y() >= 0
            && tool_state.is_allowed_for(self.region_type(location))
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let (depth, target) = result.unwrap().1;
    // println!("depth: {}", depth);
    // println!("target: {:?}", target);

    let origin = p2d(0, 0);
    let domain = BBox2d::from_corners(origin, target);

    let mut map = Map::new(target, depth);

    // Part 1
    let mut sum = 0;
    for p in &domain {
        sum += map.risk_level(p);
    }
    println!("1: {}", sum);

    // Part 2
    let map = Rc::new(RefCell::new(map));
    let start_node = Node {
        location: p2d(0, 0),
        tool_state: ToolState::Torch,
    };
    let target_node = Node {
        location: target,
        tool_state: ToolState::Torch,
    };

    let successors = |n: &Node| {
        let old_n = n.clone();
        let map = &map;
        Action::values().filter_map(move |action| {
            let new_n = action.apply(&old_n);
            let mut map = map.borrow_mut();
            if new_n != old_n && map.exists(&new_n) {
                Some((new_n, action.cost()))
            } else {
                None
            }
        })
    };
    let heuristic = |n: &Node| {
        // a lower bound on the cost to reach the target

        // we move orthogonally
        MOVE_COST * n.location.distance_l1(target)
            + if n.tool_state == ToolState::Torch {
                0
            } else {
                // we need at least one tool change of 7 min
                TOOL_CHANGE_COST
            }
    };
    let success = |n: &Node| n == &target_node;
    let search_result = astar(&start_node, successors, heuristic, success);
    let (_path, cost) = search_result.unwrap();
    println!("2: {}", cost);
}
