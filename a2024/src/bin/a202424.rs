use core::fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::satisfy;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn name(i: &str) -> IResult<&str, String> {
    let (i, cs) = recognize(many1(satisfy(|c| c.is_ascii_alphanumeric())))(i)?;
    Ok((i, cs.to_string()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct WireValue(bool);
impl fmt::Display for WireValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            true => write!(f, "1"),
            false => write!(f, "0"),
        }
    }
}

fn wire_value(i: &str) -> IResult<&str, WireValue> {
    alt((
        value(WireValue(true), tag("1")),
        value(WireValue(false), tag("0")),
    ))(i)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Wire {
    name: String,
    value: WireValue,
}
impl fmt::Display for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

fn wire(i: &str) -> IResult<&str, Wire> {
    let (i, name) = name(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, value) = wire_value(i)?;
    Ok((i, Wire { name, value }))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}
impl GateType {
    fn apply(&self, value0: bool, value1: bool) -> bool {
        match self {
            GateType::And => value0 && value1,
            GateType::Or => value0 || value1,
            GateType::Xor => value0 ^ value1,
        }
    }
}
impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GateType::And => write!(f, "AND"),
            GateType::Or => write!(f, "OR"),
            GateType::Xor => write!(f, "XOR"),
        }
    }
}

fn gate_type(i: &str) -> IResult<&str, GateType> {
    alt((
        value(GateType::And, tag("AND")),
        value(GateType::Or, tag("OR")),
        value(GateType::Xor, tag("XOR")),
    ))(i)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Gate {
    gate_type: GateType,
    inputs: [String; 2],
    output: String,
}
impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.inputs[0], self.gate_type, self.inputs[1], self.output,
        )
    }
}

fn gate(i: &str) -> IResult<&str, Gate> {
    let (i, input0) = name(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, gate_type) = gate_type(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, input1) = name(i)?;
    let (i, _) = tag(" -> ")(i)?;
    let (i, output) = name(i)?;

    let inputs = [input0, input1];
    Ok((
        i,
        Gate {
            gate_type,
            inputs,
            output,
        },
    ))
}

#[derive(Clone, Debug)]
struct Input {
    wires: Vec<Wire>,
    gates: Vec<Gate>,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for wire in &self.wires {
            writeln!(f, "{}", wire)?;
        }
        writeln!(f)?;
        for gate in &self.gates {
            writeln!(f, "{}", gate)?;
        }
        Ok(())
    }
}

fn input(i: &str) -> IResult<&str, Input> {
    let (i, wires) = separated_list1(line_ending, wire)(i)?;
    let (i, _) = line_ending(i)?;
    let (i, _) = line_ending(i)?;
    let (i, gates) = separated_list1(line_ending, gate)(i)?;
    Ok((i, Input { wires, gates }))
}

fn main() -> Result<()> {
    let input_data = io::read_to_string(io::stdin())?;

    // parse input
    let result = input(&input_data).map_err(|e| e.to_owned())?;

    let input = result.1;
    // println!("{}", input);

    // The gates indexed by outputs for easy access.
    let gates = input
        .gates
        .iter()
        .map(|gate| (gate.output.clone(), gate))
        .collect::<HashMap<_, _>>();

    // For each input a list of dependent outputs.
    let mut dependencies = HashMap::new();
    for gate in &input.gates {
        let input0 = gate.inputs[0].clone();
        let entry0 = dependencies.entry(input0).or_insert(Vec::new());
        entry0.push(gate.output.clone());

        let input1 = gate.inputs[1].clone();
        let entry1 = dependencies.entry(input1).or_insert(Vec::new());
        entry1.push(gate.output.clone());
    }

    let mut values = HashMap::new();
    let mut candidates = HashSet::new();
    for wire in &input.wires {
        values.insert(wire.name.clone(), wire.value.0);

        if let Some(ds) = dependencies.get(&wire.name) {
            for name in ds {
                candidates.insert(name.clone());
            }
        }
    }
    while !candidates.is_empty() {
        let mut new_candidates = HashSet::new();
        for name in &candidates {
            if let Some(gate) = gates.get(name) {
                // Try to compute the value.
                if let Some(value0) = values.get(&gate.inputs[0]) {
                    if let Some(value1) = values.get(&gate.inputs[1]) {
                        let value = gate.gate_type.apply(*value0, *value1);
                        values.insert(name.clone(), value);

                        if let Some(ds) = dependencies.get(name) {
                            for new_name in ds {
                                new_candidates.insert(new_name.clone());
                            }
                        }
                    }
                }
            }
        }
        candidates = new_candidates;
    }
    let mut z_values = values
        .iter()
        .filter_map(|(name, value)| {
            if name.starts_with("z") {
                Some((name, value))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    z_values.sort();

    let mut number = 0_i64;
    let mut pow2 = 1;
    for (i, (name, &value)) in z_values.into_iter().enumerate() {
        let expected_name = format!("z{:02}", i);
        if name != &expected_name {
            return Err(format!("output {expected_name} not set").into());
        }

        if value {
            number += pow2;
        }
        pow2 *= 2;
    }
    let result1 = number;

    let result2 = "gjc,gvm,qjj,qsb,wmp,z17,z26,z39";

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);

    Ok(())
}

/*

x00 XOR y00 -> z00
x00 AND y00 -> wbd

x01 XOR y01 -> hfr
x01 AND y01 -> ktq
hfr XOR wbd -> z01
hfr AND wbd -> fhk
fhk OR ktq -> mrk

x02 XOR y02 -> bfd
x02 AND y02 -> sfw
bfd XOR mrk -> z02
bfd AND mrk -> hwt
sfw OR hwt -> qkc

x03 XOR y03 -> nmb
x03 AND y03 -> wqk
nmb XOR qkc -> z03
nmb AND qkc -> gdm
gdm OR wqk -> www

x04 XOR y04 -> cwr
x04 AND y04 -> qvr
cwr XOR www -> z04
cwr AND www -> cqn
cqn OR qvr -> hrk

x05 XOR y05 -> sck
x05 AND y05 -> mbj
sck XOR hrk -> z05
sck AND hrk -> jwf
mbj OR jwf -> bfg

x06 XOR y06 -> hht
x06 AND y06 -> qdd
hht XOR bfg -> z06
hht AND bfg -> tfp
qdd OR tfp -> scb

x07 XOR y07 -> wkm
x07 AND y07 -> mbb
scb XOR wkm -> z07
scb AND wkm -> qnk
qnk OR mbb -> fmv

x08 XOR y08 -> sdc
x08 AND y08 -> khp
fmv XOR sdc -> z08
fmv AND sdc -> jhv
khp OR jhv -> rhc

x09 XOR y09 -> qhr
x09 AND y09 -> csw
rhc XOR qhr -> z09
rhc AND qhr -> qwj
qwj OR csw -> trw

x10 XOR y10 -> fvg
x10 AND y10 -> fsw
fvg XOR trw -> z10
fvg AND trw -> bng
bng OR fsw -> jdm

x11 XOR y11 -> gjc *
x11 AND y11 -> qjj *
qjj XOR jdm -> z11
qjj AND jdm -> ckv
ckv OR gjc -> sfm

x12 XOR y12 -> dnj
x12 AND y12 -> rvj
dnj XOR sfm -> z12
dnj AND sfm -> pjk
rvj OR pjk -> mmk

x13 XOR y13 -> mrw
x13 AND y13 -> pgd
mmk XOR mrw -> z13
mmk AND mrw -> qgn
pgd OR qgn -> gvp

x14 AND y14 -> kmw
x14 XOR y14 -> jdk
gvp XOR jdk -> z14
gvp AND jdk -> ffb
kmw OR ffb -> ptj

x15 XOR y15 -> jgs
x15 AND y15 -> rcr
jgs XOR ptj -> z15
jgs AND ptj -> wkv
rcr OR wkv -> nsf

x16 XOR y16 -> vwv
x16 AND y16 -> twg
nsf XOR vwv -> z16
nsf AND vwv -> dmw
dmw OR twg -> pvh

x17 XOR y17 -> rqq
x17 AND y17 -> pqv
pvh XOR rqq -> wmp *
pvh AND rqq -> ffg
pqv OR ffg -> z17 *

x18 XOR y18 -> vfq
x18 AND y18 -> vvq
vfq XOR wmp -> z18
vfq AND wmp -> dbn
dbn OR vvq -> vcv

x19 XOR y19 -> mpm
x19 AND y19 -> qhw
mpm XOR vcv -> z19
mpm AND vcv -> cqc
cqc OR qhw -> drq

x20 XOR y20 -> wrs
x20 AND y20 -> svn
drq XOR wrs -> z20
drq AND wrs -> fvh
fvh OR svn -> vws

x21 XOR y21 -> hmg
x21 AND y21 -> mnt
hmg XOR vws -> z21
hmg AND vws -> pmj
mnt OR pmj -> gss

x22 XOR y22 -> vrw
x22 AND y22 -> hsd
gss XOR vrw -> z22
gss AND vrw -> bdv
hsd OR bdv -> gdw

x23 XOR y23 -> wtc
x23 AND y23 -> gnv
gdw XOR wtc -> z23
gdw AND wtc -> mjd
mjd OR gnv -> psq

x24 XOR y24 -> jhj
x24 AND y24 -> ngs
jhj XOR psq -> z24
jhj AND psq -> ptp
ngs OR ptp -> knj

x25 XOR y25 -> nvk
x25 AND y25 -> fvp
knj XOR nvk -> z25
knj AND nvk -> pjr
fvp OR pjr -> qgs

x26 XOR y26 -> kfq
x26 AND y26 -> vfk
kfq XOR qgs -> gvm *
kfq AND qgs -> z26 *
vfk OR gvm -> vfs

x27 XOR y27 -> nrn
x27 AND y27 -> vcw
nrn XOR vfs -> z27
nrn AND vfs -> bjd
bjd OR vcw -> hvw

x28 XOR y28 -> cpd
x28 AND y28 -> kvv
cpd XOR hvw -> z28
cpd AND hvw -> ngp
ngp OR kvv -> swr

x29 XOR y29 -> pjv
x29 AND y29 -> crg
swr XOR pjv -> z29
swr AND pjv -> kpw
crg OR kpw -> dgk

x30 XOR y30 -> jgk
x30 AND y30 -> fnf
dgk XOR jgk -> z30
dgk AND jgk -> mfg
fnf OR mfg -> htv

x31 XOR y31 -> nrj
x31 AND y31 -> rfn
htv XOR nrj -> z31
htv AND nrj -> sgs
rfn OR sgs -> ncd

x32 XOR y32 -> jrp
x32 AND y32 -> fmk
ncd XOR jrp -> z32
ncd AND jrp -> grc
fmk OR grc -> dqg

x33 XOR y33 -> fsq
x33 AND y33 -> hrv
dqg XOR fsq -> z33
dqg AND fsq -> cps
hrv OR cps -> ptb

x34 XOR y34 -> nhv
x34 AND y34 -> qcw
nhv XOR ptb -> z34
nhv AND ptb -> spr
spr OR qcw -> wnv

x35 XOR y35 -> cpb
x35 AND y35 -> bkk
cpb XOR wnv -> z35
cpb AND wnv -> gnp
bkk OR gnp -> bgv

x36 XOR y36 -> vcq
x36 AND y36 -> rbf
bgv XOR vcq -> z36
bgv AND vcq -> nfm
nfm OR rbf -> pbm

x37 XOR y37 -> knp
x37 AND y37 -> mbq
knp XOR pbm -> z37
knp AND pbm -> spn
spn OR mbq -> rjs

x38 XOR y38 -> cwp
x38 AND y38 -> cdq
cwp XOR rjs -> z38
cwp AND rjs -> mcb
mcb OR cdq -> hkg

x39 XOR y39 -> sbq
x39 AND y39 -> z39 *
sbq XOR hkg -> qsb *
sbq AND hkg -> vsm
qsb OR vsm -> chr

x40 XOR y40 -> wsw
x40 AND y40 -> jfc
chr XOR wsw -> z40
chr AND wsw -> fct
jfc OR fct -> gfc

x41 XOR y41 -> wdd
x41 AND y41 -> rbk
gfc XOR wdd -> z41
gfc AND wdd -> ckj
ckj OR rbk -> ntc

x42 XOR y42 -> wkt
x42 AND y42 -> wvw
wkt XOR ntc -> z42
wkt AND ntc -> sqj
sqj OR wvw -> tnh

x43 XOR y43 -> ggm
x43 AND y43 -> rwv
ggm XOR tnh -> z43
ggm AND tnh -> dmf
rwv OR dmf -> ctm

x44 XOR y44 -> dmn
x44 AND y44 -> mhg
ctm XOR dmn -> z44
ctm AND dmn -> bnt
bnt OR mhg -> z45

*/
