use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Read;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use lowdim::v3d;
use lowdim::Point3d;
use lowdim::Vec3d;
use lowdim::Vector;

fn fmt_v3d(f: &mut fmt::Formatter<'_>, v: Vec3d) -> fmt::Result {
    write!(f, "<{},{},{}>", v.x(), v.y(), v.z())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Particle {
    id: usize,
    p: Point3d,
    v: Vec3d,
    a: Vec3d,
}
impl Particle {
    fn step(&self) -> Particle {
        let id = self.id;
        let a = self.a;
        let v = self.v + a;
        let p = self.p + v;
        Particle { id, p, v, a }
    }
    fn can_collide(&self, other: &Particle) -> bool {
        let sa = (self.a - other.a).signum();
        let sv = (self.v - other.v).signum();
        let sp = (self.p - other.p).signum();
        // Check for each coordinate that particles are in different positions
        // and cannot come closer together, hence cannot collide.
        let cant_collide_x =
            // The particles are not in the same position.
            sp.x() != 0
            // The relative velocity cannot change the signum of the relative position,
            // so the relative position cannot become zero.
            && ((sa.x() == 0 && sv.x() == 0) || sv.x() == sp.x())
            // The relative acceleration cannot change the signum of the relative velocity.
            && (sa.x() == 0 || sa.x() == sv.x());
        let cant_collide_y = sp.y() != 0
            && ((sa.y() == 0 && sv.y() == 0) || sv.y() == sp.y())
            && (sa.y() == 0 || sa.y() == sv.y());
        let cant_collide_z = sp.z() != 0
            && ((sa.z() == 0 && sv.z() == 0) || sv.z() == sp.z())
            && (sa.z() == 0 || sa.z() == sv.z());
        let cant_collide = cant_collide_x || cant_collide_y || cant_collide_z;
        !cant_collide
    }
}
impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "p=")?;
        fmt_v3d(f, self.p.to_vec())?;
        write!(f, ", v=")?;
        fmt_v3d(f, self.v)?;
        write!(f, ", a=")?;
        fmt_v3d(f, self.a)?;
        Ok(())
    }
}

fn int(i: &str) -> IResult<&str, i64> {
    map_res(
        recognize(tuple((opt(char('-')), digit1))),
        FromStr::from_str,
    )(i)
}

fn vec3d(i: &str) -> IResult<&str, Vec3d> {
    let (i, _) = tag("<")(i)?;
    let (i, x) = int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, z) = int(i)?;
    let (i, _) = tag(">")(i)?;
    Ok((i, v3d(x, y, z)))
}

fn point3d(i: &str) -> IResult<&str, Point3d> {
    let (i, v) = vec3d(i)?;
    Ok((i, v.into()))
}

fn particle(i: &str) -> IResult<&str, Particle> {
    let (i, _) = tag("p=")(i)?;
    let (i, p) = point3d(i)?;
    let (i, _) = tag(", v=")(i)?;
    let (i, v) = vec3d(i)?;
    let (i, _) = tag(", a=")(i)?;
    let (i, a) = vec3d(i)?;
    let (i, _) = line_ending(i)?;
    // The id will be set later.
    let id = 0;
    Ok((i, Particle { id, p, v, a }))
}

fn input(i: &str) -> IResult<&str, Vec<Particle>> {
    many1(particle)(i)
}

fn norm2(v: Vec3d) -> i64 {
    v.x() * v.x() + v.y() * v.y() + v.z() * v.z()
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // make nom happy
    input_data.push('\n');

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let mut input = result.unwrap().1;
    // for particle in &input {
    //     println!("{}", particle);
    // }

    for (id, particle) in input.iter_mut().enumerate() {
        particle.id = id;
    }

    // The particle with the smallest acceleration stays closest to the origin.
    // We use the square of the Euclidean norm for comparison.
    let result_a = input
        .iter()
        .enumerate()
        .min_by_key(|(_id, particle)| norm2(particle.a))
        .unwrap()
        .0;

    let mut particles = input;
    loop {
        let mut some_can_collide = false;
        'outer: for i in 0..(particles.len() - 1) {
            for j in (i + 1)..particles.len() {
                let pi = &particles[i];
                let pj = &particles[j];
                if pi.can_collide(pj) {
                    some_can_collide = true;
                    break 'outer;
                }
            }
        }
        if !some_can_collide {
            break;
        }

        let mut positions = HashMap::new();
        for particle in &particles {
            let entry = positions.entry(particle.p).or_insert_with(Vec::new);
            entry.push(particle);
        }

        particles = particles
            .iter()
            .filter(|particle| positions[&particle.p].len() == 1)
            .map(Particle::step)
            .collect::<Vec<_>>();
    }
    let result_b = particles.len();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
