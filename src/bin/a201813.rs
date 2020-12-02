use std::fmt;
use std::io;
use std::io::Read;

use nom::alt;
use nom::char;
use nom::do_parse;
use nom::line_ending;
use nom::many1;
use nom::named;
use nom::value;

use twodim::p2d;
use twodim::v2d;
use twodim::Array2d;
use twodim::Vec2d;

#[derive(Clone, Copy, Debug)]
enum Track {
    Empty,
    PathNS,
    PathEW,
    CurveUp,
    CurveDn,
    Crossing,
}
impl Track {
    fn to_char(&self) -> char {
        match self {
            Track::Empty => ' ',
            Track::PathNS => '|',
            Track::PathEW => '-',
            Track::CurveUp => '/',
            Track::CurveDn => '\\',
            Track::Crossing => '+',
        }
    }
}
impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    E,
    N,
    W,
    S,
}
impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::E => Direction::N,
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::E => Direction::S,
            Direction::N => Direction::E,
            Direction::W => Direction::N,
            Direction::S => Direction::W,
        }
    }
    fn to_v2d(&self) -> Vec2d {
        match self {
            Direction::E => v2d(1, 0),
            Direction::N => v2d(0, -1),
            Direction::W => v2d(-1, 0),
            Direction::S => v2d(0, 1),
        }
    }
    fn to_track(&self) -> Track {
        match self {
            Direction::E | Direction::W => Track::PathEW,
            Direction::N | Direction::S => Track::PathNS,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Direction::E => '>',
            Direction::N => '^',
            Direction::W => '<',
            Direction::S => 'v',
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}
impl Turn {
    fn apply(&self, dir: Direction) -> Direction {
        match self {
            Turn::Left => dir.left(),
            Turn::Straight => dir,
            Turn::Right => dir.right(),
        }
    }
    fn next_x_turn(&self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Cart {
    dir: Direction,
    x_turn: Turn,
}
impl Cart {
    fn new(dir: Direction) -> Cart {
        Cart {
            dir,
            x_turn: Turn::Left,
        }
    }
    fn turn(&mut self, track: Track) {
        match track {
            Track::CurveUp => {
                self.dir = match self.dir {
                    Direction::E => Direction::N,
                    Direction::N => Direction::E,
                    Direction::W => Direction::S,
                    Direction::S => Direction::W,
                };
            }
            Track::CurveDn => {
                self.dir = match self.dir {
                    Direction::E => Direction::S,
                    Direction::N => Direction::W,
                    Direction::W => Direction::N,
                    Direction::S => Direction::E,
                };
            }
            Track::Crossing => {
                self.dir = self.x_turn.apply(self.dir);
                self.x_turn = self.x_turn.next_x_turn();
            }
            _ => (),
        }
    }
    fn to_char(&self) -> char {
        self.dir.to_char()
    }
}

#[derive(Clone, Copy, Debug)]
struct Square {
    track: Track,
    cart: Option<Cart>,
}
impl Square {
    fn to_char(&self) -> char {
        if let Some(cart) = self.cart {
            cart.to_char()
        } else {
            self.track.to_char()
        }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Clone, Debug)]
struct State {
    map: Array2d<Square>,
}
impl State {
    fn from_vec(v: Vec<Vec<Square>>) -> State {
        let map = Array2d::from_vec(v);
        State { map }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.map.bounds().y_range() {
            for x in self.map.bounds().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

named!(track<&str, Track>,
    alt!(
        value!(Track::Empty, char!(' ')) |
        value!(Track::PathNS, char!('|')) |
        value!(Track::PathEW, char!('-')) |
        value!(Track::CurveUp, char!('/')) |
        value!(Track::CurveDn, char!('\\')) |
        value!(Track::Crossing, char!('+'))
    )
);

named!(cart<&str, Cart>,
    do_parse!(
        dir: alt!(
            value!(Direction::E, char!('>')) |
            value!(Direction::N, char!('^')) |
            value!(Direction::W, char!('<')) |
            value!(Direction::S, char!('v'))
        ) >> (Cart::new(dir))
    )
);

named!(square<&str, Square>,
    alt!(
        do_parse!(
            track: track >> (Square { track, cart: None } )
        ) |
        do_parse!(
            cart: cart >> ({
                let track = cart.dir.to_track();
                Square { track, cart: Some(cart) }})
        )
    )
);

named!(
    line<&str, Vec<Square>>,
    many1!(square)
);

named!(
    lines<&str, Vec<Vec<Square>>>,
    many1!(
        do_parse!(
            line: line >>
            line_ending >> (line)
        )
    )
);

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push_str("\n");

    // parse input
    let result = lines(&input_data);
    //println!("{:?}", result);

    let initial_state = State::from_vec(result.unwrap().1);
    //println!("{}", initial_state);

    let mut state = initial_state;
    let mut first_crash_pos = None;
    let last_cart_pos;
    loop {
        //println!();
        //println!("{}", state);

        // collect cart positions in specified row order
        let mut cart_positions = Vec::new();
        for y in state.map.bounds().y_range() {
            for x in state.map.bounds().x_range() {
                let p = p2d(x, y);
                let square = state.map[p];
                if let Some(_cart) = square.cart {
                    // collect cart
                    cart_positions.push(p);
                }
            }
        }

        if cart_positions.len() <= 1 {
            last_cart_pos = cart_positions[0];
            break;
        }

        // move carts
        for p0 in cart_positions {
            if let Some(cart) = state.map[p0].cart {
                // cart is still here, not annihilated by a crash

                // new position of cart
                let v = cart.dir.to_v2d();
                let p1 = p0 + v;
                //println!("{:?}: {:?} -> {:?}", cart, p0, p1);

                if let Some(_other_cart) = state.map[p1].cart {
                    // there is already another cart there, crash!
                    if first_crash_pos == None {
                        // record first crash position
                        first_crash_pos = Some(p1);
                    }
                    // remove carts
                    state.map[p0].cart = None;
                    state.map[p1].cart = None;
                } else {
                    // move cart
                    let mut cart = cart;
                    state.map[p0].cart = None;
                    cart.turn(state.map[p1].track);
                    state.map[p1].cart = Some(cart);
                }
            }
        }
    }

    let pa = first_crash_pos.unwrap();
    let pb = last_cart_pos;
    let result_a = format!("{},{}", pa.x, pa.y);
    let result_b = format!("{},{}", pb.x, pb.y);
    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
