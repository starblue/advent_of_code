use std::collections::hash_set::HashSet;
use std::io;
use std::iter::repeat;
use std::iter::repeat_with;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    Addi,
    Addr,
    Muli,
    Mulr,
    Bani,
    Banr,
    Bori,
    Borr,
    Seti,
    Setr,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

const OPS: [Op; 16] = [
    Op::Addi,
    Op::Addr,
    Op::Muli,
    Op::Mulr,
    Op::Bani,
    Op::Banr,
    Op::Bori,
    Op::Borr,
    Op::Seti,
    Op::Setr,
    Op::Gtir,
    Op::Gtri,
    Op::Gtrr,
    Op::Eqir,
    Op::Eqri,
    Op::Eqrr,
];

impl Op {
    fn exec(self: Op, a: i64, b: i64, c: i64, regs: &mut Regs) {
        let rs = &mut regs.0;
        let au = a as usize;
        let bu = b as usize;
        let cu = c as usize;
        match self {
            Op::Addi => rs[cu] = rs[au] + b,
            Op::Addr => rs[cu] = rs[au] + rs[bu],
            Op::Muli => rs[cu] = rs[au] * b,
            Op::Mulr => rs[cu] = rs[au] * rs[bu],
            Op::Bani => rs[cu] = rs[au] & b,
            Op::Banr => rs[cu] = rs[au] & rs[bu],
            Op::Bori => rs[cu] = rs[au] | b,
            Op::Borr => rs[cu] = rs[au] | rs[bu],
            Op::Seti => rs[cu] = a,
            Op::Setr => rs[cu] = rs[au],
            Op::Gtir => rs[cu] = if a > rs[bu] { 1 } else { 0 },
            Op::Gtri => rs[cu] = if rs[au] > b { 1 } else { 0 },
            Op::Gtrr => rs[cu] = if rs[au] > rs[bu] { 1 } else { 0 },
            Op::Eqir => rs[cu] = if a == rs[bu] { 1 } else { 0 },
            Op::Eqri => rs[cu] = if rs[au] == b { 1 } else { 0 },
            Op::Eqrr => rs[cu] = if rs[au] == rs[bu] { 1 } else { 0 },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Regs(Vec<i64>);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Instr(Vec<i64>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Sample {
    before: Regs,
    instr: Instr,
    after: Regs,
}

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn regs(i: &str) -> IResult<&str, Regs> {
    let (i, _) = multispace0(i)?;
    let (i, _) = char('[')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, r0) = int64(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, r1) = int64(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, r2) = int64(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, r3) = int64(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char(']')(i)?;
    Ok((i, Regs(vec![r0, r1, r2, r3])))
}

fn instr(i: &str) -> IResult<&str, Instr> {
    let (i, opc) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, b) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, c) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instr(vec![opc, a, b, c])))
}

fn sample(i: &str) -> IResult<&str, Sample> {
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("Before:")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, before) = regs(i)?;
    let (i, _) = line_ending(i)?;
    let (i, instr) = instr(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("After:")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, after) = regs(i)?;
    let (i, _) = line_ending(i)?;
    Ok((
        i,
        Sample {
            before,
            instr,
            after,
        },
    ))
}

fn input(i: &str) -> IResult<&str, (Vec<Sample>, Vec<Instr>)> {
    let (i, samples) = many1(sample)(i)?;
    let (i, _) = many1(line_ending)(i)?;
    let (i, instrs) = many1(instr)(i)?;
    Ok((i, (samples, instrs)))
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let (samples, instrs) = result.unwrap().1;

    let mut possible_decodings = repeat_with(|| OPS.iter().collect::<HashSet<_>>())
        .take(16)
        .collect::<Vec<_>>();
    for s in samples {
        for op in &OPS {
            let opc = s.instr.0[0];
            let a = s.instr.0[1];
            let b = s.instr.0[2];
            let c = s.instr.0[3];
            let mut regs = s.before.clone();
            op.exec(a, b, c, &mut regs);
            if regs != s.after {
                // this op is not a decoding of the opcode, remove it
                possible_decodings[opc as usize].remove(op);
            }
        }
    }

    // propagate constraints
    let mut decodings = repeat(None).take(16).collect::<Vec<_>>();
    let mut dirty = true;
    while dirty {
        dirty = false;
        for opc in 0..16 {
            if decodings[opc] == None && possible_decodings[opc].len() == 1 {
                let &op = possible_decodings[opc].iter().next().unwrap();
                decodings[opc] = Some(op);

                // other op codes can't decode to this op
                for opc1 in 0..16 {
                    possible_decodings[opc1].remove(op);
                }

                dirty = true;
            }
        }
    }
    let decodings = decodings.iter().flatten().collect::<Vec<_>>();

    let mut regs = Regs(repeat(0).take(4).collect::<Vec<_>>());
    for i in instrs {
        let opc = i.0[0];
        let a = i.0[1];
        let b = i.0[2];
        let c = i.0[3];
        let op = decodings[opc as usize];
        op.exec(a, b, c, &mut regs);
    }
    println!("{}", regs.0[0]);
}

#[cfg(test)]
mod test {
    use crate::instr;
    use crate::regs;
    use crate::sample;
    use crate::Instr;
    use crate::Regs;
    use crate::Sample;

    #[test]
    fn test_regs() {
        assert_eq!(regs("[0, 1, 2, 3]\n"), Ok(("\n", Regs(vec![0, 1, 2, 3]))));
    }

    #[test]
    fn test_instr() {
        assert_eq!(instr("0 1 2 3\n"), Ok(("", Instr(vec![0, 1, 2, 3]))));
    }

    #[test]
    fn test_sample() {
        let before = Regs(vec![0, 1, 2, 3]);
        let instr = Instr(vec![4, 5, 6, 7]);
        let after = Regs(vec![8, 9, 10, 11]);
        assert_eq!(
            sample("Before: [0, 1, 2, 3]\n4 5 6 7\nAfter:  [8, 9, 10, 11]\n\n"),
            Ok((
                "\n",
                Sample {
                    before,
                    instr,
                    after
                }
            ))
        );
    }
}
