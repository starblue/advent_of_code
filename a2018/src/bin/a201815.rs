use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::rc::Rc;

use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::combinator::value;
use nom::multi::many1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::Point2d;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UnitType {
    Goblin,
    Elf,
}
impl UnitType {
    fn to_char(&self) -> char {
        match self {
            UnitType::Goblin => 'G',
            UnitType::Elf => 'E',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Square {
    Open,
    Wall,
    Unit(UnitType),
}
impl Square {
    fn to_char(&self) -> char {
        match self {
            Square::Open => '.',
            Square::Wall => '#',
            Square::Unit(unit_type) => unit_type.to_char(),
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn reading_order_cmp(p0: &Point2d, p1: &Point2d) -> Ordering {
    p0.y().cmp(&p1.y()).then(p0.x().cmp(&p1.x()))
}

fn attack_order_cmp(u0: &Rc<RefCell<Unit>>, u1: &Rc<RefCell<Unit>>) -> Ordering {
    let u0 = u0.borrow();
    let u1 = u1.borrow();
    u0.hit_points
        .cmp(&u1.hit_points)
        .then(reading_order_cmp(&u0.position, &u1.position))
}

#[derive(Clone, Copy, Debug)]
struct Unit {
    unit_type: UnitType,
    position: Point2d,
    hit_points: i64,
}

#[derive(Clone, Copy, Debug)]
struct Config {
    elf_attack_power: i64,
    elves_may_die: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            elf_attack_power: 3,
            elves_may_die: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Err {
    ElfDied,
}

#[derive(Clone, Debug)]
struct State {
    config: Config,
    map: Array2d<i64, Square>,
    units: HashMap<Point2d, Rc<RefCell<Unit>>>,
}
impl State {
    fn from_map(map: Array2d<i64, Square>, config: Config) -> State {
        let mut units = HashMap::new();
        for position in map.bbox().iter() {
            let square = map[position];
            if let Square::Unit(unit_type) = square {
                let unit = Unit {
                    unit_type,
                    position,
                    hit_points: 200,
                };
                units.insert(position, Rc::new(RefCell::new(unit)));
            }
        }
        State { map, units, config }
    }
    pub fn get_unit(&self, p: Point2d) -> Option<Rc<RefCell<Unit>>> {
        self.units.get(&p).cloned()
    }
    pub fn turns(&self) -> Vec<Rc<RefCell<Unit>>> {
        let mut units = self.units.values().cloned().collect::<Vec<_>>();
        units.sort_by(|u0, u1| {
            let u0 = u0.borrow();
            let u1 = u1.borrow();
            reading_order_cmp(&u0.position, &u1.position)
        });
        units
    }
    pub fn target_positions(&self, unit: &Rc<RefCell<Unit>>) -> HashSet<Point2d> {
        let mut result = HashSet::new();
        let unit = unit.borrow();
        for u in self.units.values() {
            let u = u.borrow();
            if u.unit_type != unit.unit_type {
                result.insert(u.position);
            }
        }
        result
    }
    pub fn is_open_position(&self, p: Point2d) -> bool {
        self.map.get(p) == Some(&Square::Open)
    }
    pub fn is_target_position(&self, u: &Rc<RefCell<Unit>>, p: Point2d) -> bool {
        if let Some(&Square::Unit(unit_type)) = self.map.get(p) {
            unit_type != u.borrow().unit_type
        } else {
            false
        }
    }
    pub fn move_unit(&mut self, unit: &Rc<RefCell<Unit>>, p: Point2d) {
        self.units.remove(&unit.borrow().position);

        let u = &mut unit.borrow_mut();
        let unit_position = &mut u.position;
        assert_eq!(1, unit_position.distance_l1(p));
        self.map[*unit_position] = Square::Open;
        *unit_position = p;
        let unit_type = u.unit_type;
        self.map[p] = Square::Unit(unit_type);

        self.units.insert(p, unit.clone());
    }
    pub fn attack(&mut self, unit: &Rc<RefCell<Unit>>, p: Point2d) -> Result<(), Err> {
        assert_eq!(1, unit.borrow().position.distance_l1(p));
        assert!(self.is_target_position(unit, p));

        let target = self.units[&p].clone();
        let mut target = target.borrow_mut();
        target.hit_points -= self.attack_power(unit);
        if target.hit_points <= 0 {
            self.units.remove(&p);
            self.map[p] = Square::Open;
            if target.unit_type == UnitType::Elf && !self.config.elves_may_die {
                return Err(Err::ElfDied);
            }
        }
        Ok(())
    }
    pub fn attack_power(&self, unit: &Rc<RefCell<Unit>>) -> i64 {
        if unit.borrow().unit_type == UnitType::Elf {
            self.config.elf_attack_power
        } else {
            3
        }
    }
    pub fn hit_points(&self) -> i64 {
        self.units
            .values()
            .map(|u| u.borrow().hit_points)
            .sum::<i64>()
    }
    pub fn run(&mut self) -> Result<i64, Err> {
        let mut round = 0;
        let full_rounds;
        'outer: loop {
            round += 1;

            // play the units in turn order
            for u in self.turns() {
                if u.borrow().hit_points > 0 {
                    let target_positions = self.target_positions(&u);
                    if target_positions.is_empty() {
                        full_rounds = round - 1;
                        break 'outer;
                    }

                    let pu = u.borrow().position;

                    let mut target_neighbor_positions = HashSet::new();
                    for tp in target_positions {
                        for p1 in tp.neighbors_l1() {
                            if self.is_open_position(p1) || p1 == pu {
                                target_neighbor_positions.insert(p1);
                            }
                        }
                    }

                    if !target_neighbor_positions.contains(&pu) {
                        // we are not yet in an attack position

                        // do a breadth-first search until a target neighbor is reached
                        // or all reachable positions are exhausted
                        let mut positions = HashSet::new();
                        let mut distances = HashMap::new();
                        positions.insert(pu);
                        let mut distance = 0;
                        distances.insert(pu, distance);
                        while !positions.is_empty()
                            && positions.is_disjoint(&target_neighbor_positions)
                        {
                            distance += 1;
                            let mut new_positions = HashSet::new();
                            for p in &positions {
                                for p1 in p.neighbors_l1() {
                                    if self.is_open_position(p1) && distances.get(&p1) == None {
                                        new_positions.insert(p1);
                                        distances.insert(p1, distance);
                                    }
                                }
                            }
                            positions = new_positions;
                        }
                        if positions.is_empty() {
                            continue;
                        }

                        // find target position to move towards
                        let mut target_positions = Vec::new();
                        for &p in &positions {
                            if target_neighbor_positions.contains(&p) {
                                target_positions.push(p);
                            }
                        }
                        target_positions.sort_by(reading_order_cmp);
                        let target_position = target_positions[0];

                        let mut positions = HashSet::new();
                        positions.insert(target_position);
                        while distance > 1 {
                            let mut new_positions = HashSet::new();
                            distance -= 1;
                            for p in &positions {
                                for np in p.neighbors_l1() {
                                    if let Some(&d) = distances.get(&np) {
                                        if d == distance {
                                            new_positions.insert(np);
                                        }
                                    }
                                }
                            }
                            positions = new_positions;
                        }
                        let mut positions = positions.into_iter().collect::<Vec<_>>();
                        positions.sort_by(reading_order_cmp);

                        self.move_unit(&u, positions[0]);
                    }

                    // unit position after possible move
                    let pu = u.borrow().position;
                    if target_neighbor_positions.contains(&pu) {
                        let mut targets = Vec::new();
                        for np in pu.neighbors_l1() {
                            if self.is_target_position(&u, np) {
                                if let Some(u) = self.get_unit(np) {
                                    targets.push(u);
                                }
                            }
                        }
                        targets.sort_by(attack_order_cmp);
                        let target_position = targets[0].borrow().position;
                        self.attack(&u, target_position)?;
                    }
                }
            }
        }
        Ok(full_rounds * self.hit_points())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.map.bbox().y_range() {
            for x in self.map.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            write!(f, "   ")?;
            let mut sep = "";
            for x in self.map.bbox().x_range() {
                if let Some(u) = self.units.get(&p2d(x, y)) {
                    let u = u.borrow();
                    write!(f, "{}{}({})", sep, u.unit_type.to_char(), u.hit_points)?;
                    sep = ", ";
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn square(i: &str) -> IResult<&str, Square> {
    let p0 = value(Square::Open, char('.'));
    let p1 = value(Square::Wall, char('#'));
    let p2 = value(Square::Unit(UnitType::Goblin), char('G'));
    let p3 = value(Square::Unit(UnitType::Elf), char('E'));
    alt((p0, p1, p2, p3))(i)
}

fn line(i: &str) -> IResult<&str, Vec<Square>> {
    let (i, line) = many1(square)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, line))
}

fn lines(i: &str) -> IResult<&str, Vec<Vec<Square>>> {
    many1(line)(i)
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = lines(&input_data);
    //println!("{:?}", result);

    let map = Array2d::from_vec(result.unwrap().1);

    let mut state = State::from_map(map.clone(), Config::default());
    let result_a = state.run().unwrap();

    let mut eap = 3;
    let result = loop {
        let config = Config {
            elf_attack_power: eap,
            elves_may_die: false,
        };
        let mut state = State::from_map(map.clone(), config);
        if let Ok(result) = state.run() {
            break result;
        }
        eap += 1;
    };

    let result_b = result;
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
