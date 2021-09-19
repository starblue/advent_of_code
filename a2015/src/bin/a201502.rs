use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::do_parse;
use nom::many1;
use nom::map_res;
use nom::named;

struct Present {
    length: usize,
    width: usize,
    height: usize,
}
impl Present {
    fn volume(&self) -> usize {
        self.length * self.width * self.height
    }
    fn paper_area(&self) -> usize {
        let area0 = self.length * self.width;
        let area1 = self.width * self.height;
        let area2 = self.height * self.length;
        let min_area = area0.min(area1).min(area2);

        2 * (area0 + area1 + area2) + min_area
    }
    fn ribbon_length(&self) -> usize {
        let perimeter0 = 2 * self.length + 2 * self.width;
        let perimeter1 = 2 * self.width + 2 * self.height;
        let perimeter2 = 2 * self.height + 2 * self.length;
        let min_perimeter = perimeter0.min(perimeter1).min(perimeter2);

        min_perimeter + self.volume()
    }
}

named!(uint<&str, usize>,
    map_res!(digit1, FromStr::from_str)
);

named!(present<&str, Present>,
    do_parse!(
        length: uint >>
        char!('x') >>
        width: uint >>
        char!('x') >>
        height: uint >>
        line_ending >> (Present { length, width, height })
    )
);

named!(input<&str, Vec<Present>>,
    many1!(present)
);

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;

    let result_a = input.iter().map(|p| p.paper_area()).sum::<usize>();

    let result_b = input.iter().map(|p| p.ribbon_length()).sum::<usize>();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
