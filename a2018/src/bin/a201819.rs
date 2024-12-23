use std::fmt;
use std::io;
use std::iter::repeat;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::combinator::value;
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

impl Op {
    fn exec(&self, a: i64, b: i64, c: i64, regs: &mut Regs) {
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

impl Regs {
    fn new() -> Regs {
        Regs(repeat(0).take(6).collect::<Vec<_>>())
    }
    fn get(&self, n: i64) -> i64 {
        self.0[n as usize]
    }
    fn set(&mut self, n: i64, v: i64) {
        self.0[n as usize] = v
    }
}

impl fmt::Display for Regs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sep = "";
        write!(f, "[")?;
        for (i, r) in self.0.iter().enumerate() {
            write!(f, "{}r{}: {}", sep, i, r)?;
            sep = ", ";
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Instr {
    op: Op,
    a: i64,
    b: i64,
    c: i64,
}

impl Instr {
    fn exec(&self, regs: &mut Regs) {
        let op = self.op;
        let a = self.a;
        let b = self.b;
        let c = self.c;
        op.exec(a, b, c, regs);
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.op, self.a, self.b, self.c)
    }
}

#[derive(Clone, Debug)]
enum Error {}

fn int64(i: &str) -> IResult<&str, i64> {
    map_res(digit1, FromStr::from_str)(i)
}

fn ip_decl(i: &str) -> IResult<&str, i64> {
    let (i, _) = tag("#ip ")(i)?;
    let (i, ip) = int64(i)?;
    Ok((i, ip))
}

fn op(i: &str) -> IResult<&str, Op> {
    alt((
        value(Op::Addi, tag("addi")),
        value(Op::Addr, tag("addr")),
        value(Op::Muli, tag("muli")),
        value(Op::Mulr, tag("mulr")),
        value(Op::Bani, tag("bani")),
        value(Op::Banr, tag("banr")),
        value(Op::Bori, tag("bori")),
        value(Op::Borr, tag("borr")),
        value(Op::Seti, tag("seti")),
        value(Op::Setr, tag("setr")),
        value(Op::Gtir, tag("gtir")),
        value(Op::Gtri, tag("gtri")),
        value(Op::Gtrr, tag("gtrr")),
        value(Op::Eqir, tag("eqir")),
        value(Op::Eqri, tag("eqri")),
        value(Op::Eqrr, tag("eqrr")),
    ))(i)
}

fn instr(i: &str) -> IResult<&str, Instr> {
    let (i, op) = op(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, a) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, b) = int64(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, c) = int64(i)?;
    let (i, _) = line_ending(i)?;
    Ok((i, Instr { op, a, b, c }))
}

fn input(i: &str) -> IResult<&str, (i64, Vec<Instr>)> {
    let (i, ip) = ip_decl(i)?;
    let (i, _) = line_ending(i)?;
    let (i, instrs) = many1(instr)(i)?;
    Ok((i, (ip, instrs)))
}

fn run(ip_reg: i64, instrs: &[Instr], regs: &mut Regs) {
    let mut ip = 0;
    while 0 <= ip && ip < instrs.len() as i64 {
        if ip == 3 {
            // shortcut inner loop
            let r0 = regs.get(0);
            let r1 = regs.get(1);
            let r5 = regs.get(5);
            if r5 % r1 == 0 {
                regs.set(0, r1 + r0);
            }
            regs.set(2, r5);
            ip = 8;
        }
        regs.set(ip_reg, ip);
        let instr = instrs[ip as usize];
        instr.exec(regs);
        ip = regs.get(ip_reg);
        ip += 1;
    }
}

fn main() {
    let input_data = io::read_to_string(io::stdin()).expect("I/O error");

    // parse input
    let result = input(&input_data);
    //println!("{:?}", result);

    let data = result.unwrap().1;

    let ip_reg = data.0;
    let instrs = data.1;

    let mut regs = Regs::new();
    run(ip_reg, &instrs, &mut regs);
    println!("1: {}", regs.get(0));

    let mut regs = Regs::new();
    regs.set(0, 1);
    run(ip_reg, &instrs, &mut regs);
    println!("2: {}", regs.get(0));
}

#[cfg(test)]
mod test {
    use crate::instr;
    use crate::Instr;
    use crate::Op;

    #[test]
    fn test_instr() {
        assert_eq!(
            instr("addi 1 2 3\n"),
            Ok((
                "",
                Instr {
                    op: Op::Addi,
                    a: 1,
                    b: 2,
                    c: 3
                }
            ))
        );
    }
}
