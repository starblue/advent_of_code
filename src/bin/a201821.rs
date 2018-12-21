use std::collections::HashSet;
// use std::fmt;
// use std::io;
// use std::io::Read;
// use std::iter::repeat;
// use std::str::FromStr;

// use nom::*;

// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
// enum Op {
//     ADDI,
//     ADDR,
//     MULI,
//     MULR,
//     BANI,
//     BANR,
//     BORI,
//     BORR,
//     SETI,
//     SETR,
//     GTIR,
//     GTRI,
//     GTRR,
//     EQIR,
//     EQRI,
//     EQRR,
// }

// impl Op {
//     fn exec(&self, a: i64, b: i64, c: i64, regs: &mut Regs) {
//         let rs = &mut regs.0;
//         let au = a as usize;
//         let bu = b as usize;
//         let cu = c as usize;
//         match self {
//             Op::ADDI => rs[cu] = rs[au] + b,
//             Op::ADDR => rs[cu] = rs[au] + rs[bu],
//             Op::MULI => rs[cu] = rs[au] * b,
//             Op::MULR => rs[cu] = rs[au] * rs[bu],
//             Op::BANI => rs[cu] = rs[au] & b,
//             Op::BANR => rs[cu] = rs[au] & rs[bu],
//             Op::BORI => rs[cu] = rs[au] | b,
//             Op::BORR => rs[cu] = rs[au] | rs[bu],
//             Op::SETI => rs[cu] = a,
//             Op::SETR => rs[cu] = rs[au],
//             Op::GTIR => rs[cu] = if a > rs[bu] { 1 } else { 0 },
//             Op::GTRI => rs[cu] = if rs[au] > b { 1 } else { 0 },
//             Op::GTRR => rs[cu] = if rs[au] > rs[bu] { 1 } else { 0 },
//             Op::EQIR => rs[cu] = if a == rs[bu] { 1 } else { 0 },
//             Op::EQRI => rs[cu] = if rs[au] == b { 1 } else { 0 },
//             Op::EQRR => rs[cu] = if rs[au] == rs[bu] { 1 } else { 0 },
//         }
//     }
// }

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct Regs(Vec<i64>);

// impl Regs {
//     fn new() -> Regs {
//         Regs(repeat(0).take(6).collect::<Vec<_>>())
//     }
//     fn get(&self, n: i64) -> i64 {
//         self.0[n as usize]
//     }
//     fn set(&mut self, n: i64, v: i64) {
//         self.0[n as usize] = v
//     }
// }

// impl fmt::Display for Regs {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut sep = "";
//         write!(f, "[")?;
//         for (i, r) in self.0.iter().enumerate() {
//             write!(f, "{}r{}: {}", sep, i, r)?;
//             sep = ", ";
//         }
//         write!(f, "]")?;
//         Ok(())
//     }
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
// struct Instr {
//     op: Op,
//     a: i64,
//     b: i64,
//     c: i64,
// }

// impl Instr {
//     fn exec(&self, regs: &mut Regs) {
//         let op = self.op;
//         let a = self.a;
//         let b = self.b;
//         let c = self.c;
//         op.exec(a, b, c, regs);
//     }
// }

// impl fmt::Display for Instr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?} {} {} {}", self.op, self.a, self.b, self.c)
//     }
// }

// #[derive(Clone, Debug)]
// enum Error {}

// named!(int64<&str, i64>,
//     map_res!(digit, FromStr::from_str)
// );

// named!(ip_decl<&str, i64>,
//     do_parse!(
//         tag_s!("#ip ") >>
//         ip: int64 >>
//             (ip)
//     )
// );

// named!(op<&str, Op>,
//     alt!(
//         value!(Op::ADDI, tag_s!("addi")) |
//         value!(Op::ADDR, tag_s!("addr")) |
//         value!(Op::MULI, tag_s!("muli")) |
//         value!(Op::MULR, tag_s!("mulr")) |
//         value!(Op::BANI, tag_s!("bani")) |
//         value!(Op::BANR, tag_s!("banr")) |
//         value!(Op::BORI, tag_s!("bori")) |
//         value!(Op::BORR, tag_s!("borr")) |
//         value!(Op::SETI, tag_s!("seti")) |
//         value!(Op::SETR, tag_s!("setr")) |
//         value!(Op::GTIR, tag_s!("gtir")) |
//         value!(Op::GTRI, tag_s!("gtri")) |
//         value!(Op::GTRR, tag_s!("gtrr")) |
//         value!(Op::EQIR, tag_s!("eqir")) |
//         value!(Op::EQRI, tag_s!("eqri")) |
//         value!(Op::EQRR, tag_s!("eqrr"))
//     )
// );

// named!(instr<&str, Instr>,
//     do_parse!(
//         op: op >>
//         tag_s!(" ") >>
//         a: int64 >>
//         tag_s!(" ") >>
//         b: int64 >>
//         tag_s!(" ") >>
//         c: int64 >>
//             (Instr{op, a, b, c})
//     )
// );

// named!(input<&str, (i64, Vec<Instr>)>,
//     do_parse!(
//         ip: ip_decl >>
//         line_ending >>
//         instrs: many1!(
//             do_parse!(instr: instr >> line_ending >> (instr))
//         ) >>
//             ((ip, instrs))
//     )
// );

// fn run(ip_reg: i64, instrs: &Vec<Instr>, regs: &mut Regs) {
//     let mut ip = 0;
//     while 0 <= ip && ip < instrs.len() as i64 {
//         regs.set(ip_reg, ip);
//         let instr = instrs[ip as usize];
//         if instr.op == Op::EQRR && instr.b == 0 {
//             // put the right value to terminate into r0
//             regs.set(0, instr.a);
//         }
//         instr.exec(regs);
//         println!("{}: {} {}", ip, instr, regs);
//         ip = regs.get(ip_reg);
//         ip += 1;
//     }
// }

fn compute_first_r5() -> i64 {
    compute_r5(true)
}
fn compute_last_r5() -> i64 {
    compute_r5(false)
}
fn compute_r5(first: bool) -> i64 {
    // reverse engineering of the input file
    let mut stop_values = HashSet::new();
    let mut last_stop_value = std::i64::MAX;
    let mut r2;
    let mut r4;
    let mut r5;
    loop {
        r5 = 123;
        r5 &= 456;
        if r5 == 72 {
            break;
        }
    }
    r5 = 0;
    loop {
        r4 = r5 | 65536;
        r5 = 8858047;
        loop {
            r2 = r4 & 255;
            r5 += r2;
            r5 &= 16777215; // 2^24-1 = 0xffffff
            r5 *= 65899;
            r5 &= 16777215; // 2^24-1 = 0xffffff

            if 256 > r4 {
                break;
            }
            r2 = 0;
            while (r2 + 1) * 256 <= r4 {
                r2 += 1;
            }
            r4 = r2
        }
        // Here the program stops if r5 == r0.
        // We want the first value of r5 for part 1,
        // and the last before values start repeating for part 2.
        if first {
            return r5;
        } else {
            if stop_values.contains(&r5) {
                // value in r5 was seen before, return previous
                return last_stop_value;
            }
            last_stop_value = r5;
            stop_values.insert(r5);
        }
    }
}

fn main() {
    // let mut input_data = String::new();
    // io::stdin()
    //     .read_to_string(&mut input_data)
    //     .expect("I/O error");

    // // make nom happy
    // input_data.push_str("\n");

    // // parse input
    // let result = input(&input_data);
    // //println!("{:?}", result);

    // let data = result.unwrap().1;

    // let ip_reg = data.0;
    // let instrs = data.1;

    // for i in 0..instrs.len() {
    //     println!("{}: {}", i, instrs[i]);
    // }
    // println!();

    println!("1: {}", compute_first_r5());
    println!("2: {}", compute_last_r5());
}
