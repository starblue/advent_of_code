use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weight: i64,
    subprogram_names: Vec<String>,
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.weight)?;
        let mut sep = " -> ";
        for n in &self.subprogram_names {
            write!(f, "{}{}", sep, n)?;
            sep = ", ";
        }
        Ok(())
    }
}

fn name(i: &str) -> IResult<&str, String> {
    map(recognize(alpha1), String::from)(i)
}

fn uint(i: &str) -> IResult<&str, i64> {
    map_res(recognize(digit1), FromStr::from_str)(i)
}

fn subprogram_names_nonempty(i: &str) -> IResult<&str, Vec<String>> {
    let (i, _) = tag(" -> ")(i)?;
    let (i, names) = separated_list1(tag(", "), name)(i)?;
    Ok((i, names))
}
fn subprogram_names_empty(i: &str) -> IResult<&str, Vec<String>> {
    Ok((i, Vec::new()))
}
fn subprogram_names(i: &str) -> IResult<&str, Vec<String>> {
    alt((subprogram_names_nonempty, subprogram_names_empty))(i)
}

fn program(i: &str) -> IResult<&str, Program> {
    let (i, n) = name(i)?;
    let (i, _) = tag(" (")(i)?;
    let (i, weight) = uint(i)?;
    let (i, _) = tag(")")(i)?;
    let (i, subprogram_names) = subprogram_names(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Program {
            name: n,
            weight,
            subprogram_names,
        },
    ))
}

fn input(i: &str) -> IResult<&str, Vec<Program>> {
    many1(program)(i)
}

#[derive(Clone, Debug)]
struct SubtreeInfo {
    /// The total weight of the subtree.
    total_weight: i64,
    /// The corrected weight of the program with the wrong weight
    /// if it is in the subtree, otherwise `None`.
    corrected_weight: Option<i64>,
}

fn subtree_info(
    programs: &HashMap<String, Program>,
    subtree_infos: &mut HashMap<String, SubtreeInfo>,
    name: &str,
) -> SubtreeInfo {
    if let Some(info) = subtree_infos.get(name) {
        info.clone()
    } else {
        let p = programs[name].clone();
        let mut weight_names = HashMap::new();
        let mut total_weight = p.weight;
        for n in &p.subprogram_names {
            let info = subtree_info(programs, subtree_infos, n);
            total_weight += info.total_weight;

            let entry = weight_names
                .entry(info.total_weight)
                .or_insert_with(Vec::new);
            entry.push(n.to_string());
        }
        let corrected_weight = {
            if weight_names.len() > 1 {
                // p is unbalanced, find subprogram with wrong weight
                let (wrong_weight, wrong_weight_names) =
                    weight_names.iter().find(|(_w, ns)| ns.len() == 1).unwrap();
                let wrong_weight_name = &wrong_weight_names[0];
                let wrong_weight_info = subtree_info(programs, subtree_infos, wrong_weight_name);
                if let Some(cw) = wrong_weight_info.corrected_weight {
                    // The wrong weight is deeper inside the subtree, just copy it.
                    Some(cw)
                } else {
                    // The wrong weight is at an immediate subprogram.
                    let (right_weight, _) =
                        weight_names.iter().find(|(_w, ns)| ns.len() > 1).unwrap();
                    let wrong_weight_program = &programs[wrong_weight_name];
                    let error = wrong_weight - right_weight;
                    Some(wrong_weight_program.weight - error)
                }
            } else {
                None
            }
        };
        let info = SubtreeInfo {
            total_weight,
            corrected_weight,
        };
        subtree_infos.insert(name.to_string(), info.clone());
        info
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let input = result.unwrap().1;
    // for program in &input {
    //     println!("{}", program);
    // }

    let mut parents = HashSet::new();
    let mut children = HashSet::new();
    for program in &input {
        parents.insert(program.name.clone());
        for n in &program.subprogram_names {
            children.insert(n.clone());
        }
    }
    let diff = parents.difference(&children).cloned().collect::<Vec<_>>();
    let root_program = &diff[0];
    let result_a = root_program;

    let mut programs = HashMap::new();
    for p in &input {
        programs.insert(p.name.clone(), p.clone());
    }
    let mut subtree_infos = HashMap::new();
    let info = subtree_info(&programs, &mut subtree_infos, root_program);
    let result_b = info.corrected_weight.unwrap();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}
