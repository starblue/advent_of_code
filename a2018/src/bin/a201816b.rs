use std::collections::hash_set::HashSet;
use std::io;
use std::io::Read;
use std::iter::repeat;
use std::iter::repeat_with;
use std::str::FromStr;

use nom::char;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::do_parse;
use nom::many1;
use nom::map_res;
use nom::named;
use nom::tag;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    ADDI,
    ADDR,
    MULI,
    MULR,
    BANI,
    BANR,
    BORI,
    BORR,
    SETI,
    SETR,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

const OPS: [Op; 16] = [
    Op::ADDI,
    Op::ADDR,
    Op::MULI,
    Op::MULR,
    Op::BANI,
    Op::BANR,
    Op::BORI,
    Op::BORR,
    Op::SETI,
    Op::SETR,
    Op::GTIR,
    Op::GTRI,
    Op::GTRR,
    Op::EQIR,
    Op::EQRI,
    Op::EQRR,
];

impl Op {
    fn exec(self: Op, a: i64, b: i64, c: i64, regs: &mut Regs) {
        let rs = &mut regs.0;
        let au = a as usize;
        let bu = b as usize;
        let cu = c as usize;
        match self {
            Op::ADDI => rs[cu] = rs[au] + b,
            Op::ADDR => rs[cu] = rs[au] + rs[bu],
            Op::MULI => rs[cu] = rs[au] * b,
            Op::MULR => rs[cu] = rs[au] * rs[bu],
            Op::BANI => rs[cu] = rs[au] & b,
            Op::BANR => rs[cu] = rs[au] & rs[bu],
            Op::BORI => rs[cu] = rs[au] | b,
            Op::BORR => rs[cu] = rs[au] | rs[bu],
            Op::SETI => rs[cu] = a,
            Op::SETR => rs[cu] = rs[au],
            Op::GTIR => rs[cu] = if a > rs[bu] { 1 } else { 0 },
            Op::GTRI => rs[cu] = if rs[au] > b { 1 } else { 0 },
            Op::GTRR => rs[cu] = if rs[au] > rs[bu] { 1 } else { 0 },
            Op::EQIR => rs[cu] = if a == rs[bu] { 1 } else { 0 },
            Op::EQRI => rs[cu] = if rs[au] == b { 1 } else { 0 },
            Op::EQRR => rs[cu] = if rs[au] == rs[bu] { 1 } else { 0 },
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

named!(int64<&str, i64>,
    map_res!(digit1, FromStr::from_str)
);

named!(regs<&str, Regs>,
    do_parse!(
        multispace0 >>
        char!('[') >>
        multispace0 >>
        r0: int64 >>
        multispace0 >>
        tag!(",") >>
        multispace0 >>
        r1: int64 >>
        multispace0 >>
        tag!(",") >>
        multispace0 >>
        r2: int64 >>
        multispace0 >>
        tag!(",") >>
        multispace0 >>
        r3: int64 >>
        multispace0 >>
        char!(']') >>
            (Regs(vec![r0, r1, r2, r3]))
    )
);

named!(instr<&str, Instr>,
    do_parse!(
        opc: int64 >>
        tag!(" ") >>
        a: int64 >>
        tag!(" ") >>
        b: int64 >>
        tag!(" ") >>
        c: int64 >>
            (Instr(vec![opc, a, b, c]))
    )
);

named!(sample<&str, Sample>,
    do_parse!(
        multispace0 >>
        tag!("Before:") >>
        multispace0 >>
        before: regs >> line_ending >>
        instr: instr >> line_ending >>
        multispace0 >>
        tag!("After:") >>
        multispace0 >>
        after: regs >> line_ending >>
            (Sample { before, instr, after })
    )
);

named!(input<&str, (Vec<Sample>, Vec<Instr>)>,
    do_parse!(
        samples: many1!(sample) >>
        many1!(line_ending) >>
        instrs: many1!(
            do_parse!(instr: instr >> line_ending >> (instr))
        ) >>
            ((samples, instrs))
    )
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
        assert_eq!(instr("0 1 2 3\n"), Ok(("\n", Instr(vec![0, 1, 2, 3]))));
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
