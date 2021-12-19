use core::fmt;

use std::collections::HashSet;
use std::io;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::IResult;

use lowdim::p3d;
use lowdim::AffineTransformation;
use lowdim::Matrix;
use lowdim::Matrix3d;
use lowdim::Point3d;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    scanner_id: i64,
    beacons: Vec<Point3d<i64>>,
}
impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "--- scanner {} ---", self.scanner_id)?;
        for b in &self.beacons {
            writeln!(f, "{},{},{}", b.x(), b.y(), b.z())?;
        }
        Ok(())
    }
}

fn scanner_id(i: &str) -> IResult<&str, i64> {
    let (i, _) = tag("--- scanner ")(i)?;
    let (i, scanner_id) = i64(i)?;
    let (i, _) = tag(" ---")(i)?;
    Ok((i, scanner_id))
}

fn beacon(i: &str) -> IResult<&str, Point3d> {
    let (i, x) = i64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, y) = i64(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, z) = i64(i)?;
    Ok((i, p3d(x, y, z)))
}

fn report(i: &str) -> IResult<&str, Report> {
    let (i, scanner_id) = scanner_id(i)?;
    let (i, _) = line_ending(i)?;
    let (i, beacons) = separated_list0(line_ending, beacon)(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Report {
            scanner_id,
            beacons,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Report>> {
    separated_list0(line_ending, report)(i)
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = input(&input_data);
    // println!("{:?}", result);

    let input = result.unwrap().1;
    // for report in &input {
    //     println!("{}", report);
    // }

    let mut rotations = Vec::new();
    for xj in 0..=2 {
        for yj in 0..=2 {
            if yj != xj {
                for zj in 0..=2 {
                    if zj != xj && zj != yj {
                        for xs in [1, -1] {
                            for ys in [1, -1] {
                                for zs in [1, -1] {
                                    let m = Matrix3d::with(|i, j| match i {
                                        0 if j == xj => xs,
                                        1 if j == yj => ys,
                                        2 if j == zj => zs,
                                        _ => 0,
                                    });
                                    if m.det() == 1 {
                                        rotations.push(m);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // for m in &rotations {
    //     println!("{:?}", m);
    // }

    // Initialize with data from first scanner
    // The transformation maps from the report coordinate system to the global one,
    // which is taken to be the one from the first report.
    let f = <AffineTransformation<i64, Matrix3d<i64>>>::identity();
    let placed_beacons = input[0]
        .beacons
        .iter()
        .map(|&p| f.apply(p))
        .collect::<HashSet<_>>();
    let mut placed_scanners = vec![(p3d(0, 0, 0), placed_beacons)];
    let mut unplaced_reports = input[1..].to_vec();
    while !unplaced_reports.is_empty() {
        let mut new_placed_scanners = Vec::new();
        let mut new_placed_ids = HashSet::new();

        for report in &unplaced_reports {
            // Try to find 12 overlapping probes with a placed scanner
            'search: for (_, placed_beacons) in &placed_scanners {
                for placed_beacon in placed_beacons {
                    for beacon in &report.beacons {
                        for rotation in &rotations {
                            // Find affine transformation using the given rotation
                            // to map the unplaced beacon from the report to the placed beacon,
                            // i.e. assume the report is for the same beacon.
                            let b = rotation * beacon.to_vec();
                            let t =
                                AffineTransformation::new(*rotation, placed_beacon.to_vec() - b);

                            // The set of beacons from this report in global coordinates
                            let new_beacons = report
                                .beacons
                                .iter()
                                .map(|&p| t.apply(p))
                                .collect::<HashSet<_>>();

                            // Check whether at least 12 beacons from the report
                            // are detected by the placed scanner,
                            // that is among the placed beacons.
                            let count = placed_beacons.intersection(&new_beacons).count();
                            if count >= 12 {
                                // Place scanner for the report.
                                let new_scanner = t.apply(p3d(0, 0, 0));
                                new_placed_scanners.push((new_scanner, new_beacons));
                                new_placed_ids.insert(report.scanner_id);
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
        placed_scanners.append(&mut new_placed_scanners);
        unplaced_reports.retain(|r| !new_placed_ids.contains(&r.scanner_id));
    }
    let beacons = placed_scanners
        .iter()
        .flat_map(|(_, beacons)| beacons)
        .copied()
        .collect::<HashSet<_>>();
    let result_a = beacons.len();

    let scanners = placed_scanners
        .iter()
        .map(|(scanner, _beacons)| scanner)
        .copied()
        .collect::<HashSet<_>>();
    let mut max_distance = 0;
    for &s0 in &scanners {
        for &s1 in &scanners {
            max_distance = max_distance.max(s0.distance_l1(s1));
        }
    }
    let result_b = max_distance;

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
