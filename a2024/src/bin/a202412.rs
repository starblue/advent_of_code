use core::fmt;

use std::collections::HashSet;
use std::io;

use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

use lowdim::p2d;
use lowdim::Array2d;
use lowdim::BBox2d;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Plot(char);
impl Plot {
    fn to_char(self) -> char {
        self.0
    }
}
impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn plot(i: &str) -> IResult<&str, Plot> {
    let (i, c) = satisfy(|c| c.is_ascii_alphabetic())(i)?;
    Ok((i, Plot(c)))
}

#[derive(Clone, Debug)]
struct Input {
    map: Array2d<i64, Plot>,
}
impl Input {
    fn bbox(&self) -> BBox2d {
        self.map.bbox()
    }
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in self.bbox().y_range().rev() {
            for x in self.bbox().x_range() {
                let p = p2d(x, y);
                write!(f, "{}", self.map[p])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, mut lines) = separated_list1(line_ending, many1(plot))(i)?;

    // The y coordinate increases from the bottom, i.e. here from the end.
    lines.reverse();

    let map = Array2d::from_vec(lines);
    Ok((i, Input { map }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    let bbox = input.bbox();

    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut visited = Array2d::with(bbox, |_| false);
    for p_start in bbox {
        if !visited[p_start] {
            // Map a new region.
            let plant_type = input.map[p_start];

            let mut area = 0;
            let mut edges = Vec::new();

            // Explore the area by depth-first search.
            let mut stack = vec![p_start];
            visited[p_start] = true;
            while let Some(p) = stack.pop() {
                area += 1;
                for p1 in p.neighbors_l1() {
                    if bbox.contains(&p1) {
                        if input.map[p1] == plant_type {
                            if !visited[p1] {
                                visited[p1] = true;
                                stack.push(p1);
                            }
                        } else {
                            edges.push((p, p1));
                        }
                    } else {
                        edges.push((p, p1));
                    }
                }
            }

            // Walk along the boundary to measure the perimeter and count sides.
            let mut perimeter = 0;
            let mut side_count = 0;
            let mut handled_edges = HashSet::new();
            for edge in &edges {
                if !handled_edges.contains(edge) {
                    let (mut p_in, mut p_out) = edge;

                    // The edge following a corner where we start counting sides.
                    let mut start_edge = None;
                    loop {
                        handled_edges.insert((p_in, p_out));

                        let v = (p_out - p_in).rotate_left();
                        let turned;
                        if input.map.get(p_in + v) != Some(&plant_type) {
                            // The fence turns left.
                            p_out = p_in + v;
                            turned = true;
                        } else if input.map.get(p_out + v) == Some(&plant_type) {
                            // The fence turns right.
                            p_in = p_out + v;
                            turned = true;
                        } else {
                            // The fence continues straight on.
                            p_in += v;
                            p_out += v;
                            turned = false;
                        }
                        if turned {
                            if let Some(edge) = start_edge {
                                side_count += 1;
                                if (p_in, p_out) == edge {
                                    // We are back at the start, finish.
                                    break;
                                }
                            } else {
                                // Start counting.
                                start_edge = Some((p_in, p_out));
                            }
                        }

                        if start_edge.is_some() {
                            perimeter += 1;
                        }
                    }
                }
            }
            sum1 += area * perimeter;
            sum2 += area * side_count;
        }
    }
    let result1 = sum1;
    let result2 = sum2;

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}
