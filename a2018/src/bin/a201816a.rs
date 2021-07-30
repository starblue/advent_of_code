use std::io;
use std::io::Read;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    let (samples, _instrs) = result.unwrap().1;

    let mut count = 0;
    for s in samples {
        let mut op_count = 0;
        for op in &OPS {
            let a = s.instr.0[1];
            let b = s.instr.0[2];
            let c = s.instr.0[3];
            let mut regs = s.before.clone();
            op.exec(a, b, c, &mut regs);
            if regs == s.after {
                op_count += 1;
            }
        }
        if op_count >= 3 {
            count += 1;
        }
    }

    println!("{}", count);
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
