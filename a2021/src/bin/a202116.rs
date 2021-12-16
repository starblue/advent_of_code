use std::io;
use std::io::Read;

use nom::bits::bits;
use nom::bits::complete::take;
use nom::character::complete::one_of;
use nom::combinator::map_opt;
use nom::multi::many1;
use nom::IResult;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TryFromPacketTypeError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum PacketType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}
impl PacketType {
    fn value(&self, packets: &[Packet]) -> i64 {
        match self {
            PacketType::Sum => packets.iter().map(|p| p.value()).sum::<i64>(),
            PacketType::Product => packets.iter().map(|p| p.value()).product::<i64>(),
            PacketType::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
            PacketType::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
            PacketType::Literal => {
                panic!("unexpected literal packet");
            }
            PacketType::GreaterThan => {
                if packets[0].value() > packets[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::LessThan => {
                if packets[0].value() < packets[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketType::EqualTo => {
                if packets[0].value() == packets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}
impl TryFrom<u8> for PacketType {
    type Error = TryFromPacketTypeError;
    fn try_from(type_id: u8) -> Result<Self, Self::Error> {
        match type_id {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::Minimum),
            3 => Ok(PacketType::Maximum),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GreaterThan),
            6 => Ok(PacketType::LessThan),
            7 => Ok(PacketType::EqualTo),
            _ => Err(TryFromPacketTypeError),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Contents {
    Literal {
        value: i64,
    },
    Operator {
        length: Length,
        packets: Vec<Packet>,
    },
}
impl Contents {
    fn value(&self, packet_type: PacketType) -> i64 {
        match self {
            Contents::Literal { value } => *value,
            Contents::Operator { packets, .. } => packet_type.value(packets),
        }
    }
    fn subpackets(&self) -> &[Packet] {
        match self {
            Contents::Literal { .. } => &[],
            Contents::Operator { packets, .. } => packets,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Length {
    TotalBits(usize),
    PacketCount(usize),
}
impl Length {
    fn is_reached(&self, bit_len: usize, packet_count: usize) -> bool {
        match self {
            Length::TotalBits(n) => bit_len >= *n,
            Length::PacketCount(n) => packet_count >= *n,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Packet {
    version: u32,
    packet_type: PacketType,
    contents: Contents,
}
impl Packet {
    fn value(&self) -> i64 {
        self.contents.value(self.packet_type)
    }
    fn subpackets(&self) -> &[Packet] {
        self.contents.subpackets()
    }
    fn version_sum(&self) -> u32 {
        self.version
            + self
                .subpackets()
                .iter()
                .map(|p| p.version_sum())
                .sum::<u32>()
    }
}

fn nibble(i: &str) -> IResult<&str, u8> {
    let (i, d) = map_opt(one_of("0123456789ABCDEF"), |c: char| c.to_digit(16))(i)?;
    Ok((i, u8::try_from(d).unwrap()))
}

fn byte(i: &str) -> IResult<&str, u8> {
    let (i, nibble1) = nibble(i)?;
    let (i, nibble0) = nibble(i)?;
    Ok((i, (nibble1 << 4) | nibble0))
}

fn parse(i: &str) -> IResult<&str, Vec<u8>> {
    many1(byte)(i)
}

fn packet_type(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketType> {
    let (i, type_id) = take(3_usize)(i)?;
    Ok((i, <PacketType as TryFrom<u8>>::try_from(type_id).unwrap()))
}

fn length_total_bits(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Length, usize)> {
    let (i, len) = take(15_usize)(i)?;
    Ok((i, (Length::TotalBits(len), 16)))
}
fn length_packet_count(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Length, usize)> {
    let (i, len) = take(11_usize)(i)?;
    Ok((i, (Length::PacketCount(len), 12)))
}
fn length(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Length, usize)> {
    let (i, b): (_, u8) = take(1_usize)(i)?;
    if b == 0 {
        length_total_bits(i)
    } else {
        length_packet_count(i)
    }
}

fn literal_group_cont(i: (&[u8], usize)) -> IResult<(&[u8], usize), bool> {
    let (i, b): (_, u8) = take(1_usize)(i)?;
    Ok((i, b == 1))
}

fn literal_group(i: (&[u8], usize)) -> IResult<(&[u8], usize), (bool, i64)> {
    let (i, cont) = literal_group_cont(i)?;
    let (i, bits) = take(4_usize)(i)?;
    Ok((i, (cont, bits)))
}

fn contents_literal(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Contents, usize)> {
    let mut bit_len = 0;
    let mut input = i;
    let mut value = 0;
    loop {
        let (i, (cont, d)) = literal_group(input)?;
        input = i;
        bit_len += 5;

        value = 16 * value + d;
        if !cont {
            break;
        }
    }
    Ok((input, (Contents::Literal { value }, bit_len)))
}
fn contents_operator(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Contents, usize)> {
    let (i, (length, len_bit_len)) = length(i)?;

    let mut bit_len = 0;
    let mut input = i;
    let mut packets = Vec::new();
    loop {
        let (i, (p, bl)) = packet(input)?;
        input = i;
        bit_len += bl;
        packets.push(p);

        if length.is_reached(bit_len, packets.len()) {
            break;
        }
    }
    Ok((
        input,
        (
            Contents::Operator { length, packets },
            len_bit_len + bit_len,
        ),
    ))
}

fn packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), (Packet, usize)> {
    let (i, version) = take(3_usize)(i)?;
    let (i, packet_type) = packet_type(i)?;
    let (i, (contents, bit_len)) = {
        if packet_type == PacketType::Literal {
            contents_literal(i)?
        } else {
            contents_operator(i)?
        }
    };
    Ok((
        i,
        (
            Packet {
                version,
                packet_type,
                contents,
            },
            3 + 3 + bit_len,
        ),
    ))
}

fn parse_data(i: &[u8]) -> IResult<&[u8], Packet> {
    let (i, (packet, _len)) = bits(packet)(i)?;
    Ok((i, packet))
}

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_to_string(&mut input_data)
        .expect("I/O error");

    // parse input
    let result = parse(&input_data);
    //println!("{:?}", result);

    let bytes = result.unwrap().1;
    // for b in &bytes {
    //     print!("{:02X}", b);
    // }
    // println!();

    let result = parse_data(&bytes);
    // println!("{:?}", result);

    let packet = result.unwrap().1;

    let result_a = packet.version_sum();

    let result_b = packet.value();

    println!("a: {}", result_a);
    println!("b: {}", result_b);
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::parse_data;
    use crate::Contents::Literal;
    use crate::Contents::Operator;
    use crate::Length::PacketCount;
    use crate::Length::TotalBits;
    use crate::Packet;
    use crate::PacketType;

    #[test]
    fn test_example_1() {
        let bytes = parse("D2FE28").unwrap().1;
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 6,
                    packet_type: PacketType::Literal,
                    contents: Literal { value: 2021 }
                }
            )),
            parse_data(&bytes)
        );
    }

    #[test]
    fn test_example_2() {
        let bytes = parse("38006F45291200").unwrap().1;
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 1,
                    packet_type: PacketType::LessThan,
                    contents: Operator {
                        length: TotalBits(27),
                        packets: vec![
                            Packet {
                                version: 6,
                                packet_type: PacketType::Literal,
                                contents: Literal { value: 10 }
                            },
                            Packet {
                                version: 2,
                                packet_type: PacketType::Literal,
                                contents: Literal { value: 20 }
                            }
                        ]
                    }
                }
            )),
            parse_data(&bytes)
        );
    }

    #[test]
    fn test_example_3() {
        let bytes = parse("EE00D40C823060").unwrap().1;
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 7,
                    packet_type: PacketType::Maximum,
                    contents: Operator {
                        length: PacketCount(3),
                        packets: vec![
                            Packet {
                                version: 2,
                                packet_type: PacketType::Literal,
                                contents: Literal { value: 1 }
                            },
                            Packet {
                                version: 4,
                                packet_type: PacketType::Literal,
                                contents: Literal { value: 2 }
                            },
                            Packet {
                                version: 1,
                                packet_type: PacketType::Literal,
                                contents: Literal { value: 3 }
                            }
                        ]
                    }
                }
            )),
            parse_data(&bytes)
        );
    }

    #[test]
    fn test_example_4() {
        let bytes = parse("8A004A801A8002F478").unwrap().1;
        let parse_result = parse_data(&bytes);
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 4,
                    packet_type: PacketType::Minimum,
                    contents: Operator {
                        length: PacketCount(1),
                        packets: vec![Packet {
                            version: 1,
                            packet_type: PacketType::Minimum,
                            contents: Operator {
                                length: PacketCount(1),
                                packets: vec![Packet {
                                    version: 5,
                                    packet_type: PacketType::Minimum,
                                    contents: Operator {
                                        length: TotalBits(11),
                                        packets: vec![Packet {
                                            version: 6,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 15 }
                                        }]
                                    }
                                }]
                            }
                        }]
                    }
                }
            )),
            parse_result
        );
        let packet = parse_result.unwrap().1;
        assert_eq!(16, packet.version_sum());
    }

    #[test]
    fn test_example_5() {
        let bytes = parse("620080001611562C8802118E34").unwrap().1;
        let parse_result = parse_data(&bytes);
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 3,
                    packet_type: PacketType::Sum,
                    contents: Operator {
                        length: PacketCount(2),
                        packets: vec![
                            Packet {
                                version: 0,
                                packet_type: PacketType::Sum,
                                contents: Operator {
                                    length: TotalBits(22),
                                    packets: vec![
                                        Packet {
                                            version: 0,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 10 }
                                        },
                                        Packet {
                                            version: 5,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 11 }
                                        }
                                    ]
                                }
                            },
                            Packet {
                                version: 1,
                                packet_type: PacketType::Sum,
                                contents: Operator {
                                    length: PacketCount(2),
                                    packets: vec![
                                        Packet {
                                            version: 0,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 12 }
                                        },
                                        Packet {
                                            version: 3,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 13 }
                                        }
                                    ]
                                }
                            }
                        ]
                    }
                }
            )),
            parse_result
        );
        let packet = parse_result.unwrap().1;
        assert_eq!(12, packet.version_sum());
    }

    #[test]
    fn test_example_6() {
        let bytes = parse("C0015000016115A2E0802F182340").unwrap().1;
        let parse_result = parse_data(&bytes);
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 6,
                    packet_type: PacketType::Sum,
                    contents: Operator {
                        length: TotalBits(84),
                        packets: vec![
                            Packet {
                                version: 0,
                                packet_type: PacketType::Sum,
                                contents: Operator {
                                    length: TotalBits(22),
                                    packets: vec![
                                        Packet {
                                            version: 0,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 10 }
                                        },
                                        Packet {
                                            version: 6,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 11 }
                                        }
                                    ]
                                }
                            },
                            Packet {
                                version: 4,
                                packet_type: PacketType::Sum,
                                contents: Operator {
                                    length: PacketCount(2),
                                    packets: vec![
                                        Packet {
                                            version: 7,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 12 }
                                        },
                                        Packet {
                                            version: 0,
                                            packet_type: PacketType::Literal,
                                            contents: Literal { value: 13 }
                                        }
                                    ]
                                }
                            }
                        ]
                    }
                }
            )),
            parse_result
        );
        let packet = parse_result.unwrap().1;
        assert_eq!(23, packet.version_sum());
    }

    #[test]
    fn test_example_7() {
        let bytes = parse("A0016C880162017C3686B18A3D4780").unwrap().1;
        let parse_result = parse_data(&bytes);
        assert_eq!(
            Ok((
                &vec![][..],
                Packet {
                    version: 5,
                    packet_type: PacketType::Sum,
                    contents: Operator {
                        length: TotalBits(91),
                        packets: vec![Packet {
                            version: 1,
                            packet_type: PacketType::Sum,
                            contents: Operator {
                                length: PacketCount(1),
                                packets: vec![Packet {
                                    version: 3,
                                    packet_type: PacketType::Sum,
                                    contents: Operator {
                                        length: PacketCount(5),
                                        packets: vec![
                                            Packet {
                                                version: 7,
                                                packet_type: PacketType::Literal,
                                                contents: Literal { value: 6 }
                                            },
                                            Packet {
                                                version: 6,
                                                packet_type: PacketType::Literal,
                                                contents: Literal { value: 6 }
                                            },
                                            Packet {
                                                version: 5,
                                                packet_type: PacketType::Literal,
                                                contents: Literal { value: 12 }
                                            },
                                            Packet {
                                                version: 2,
                                                packet_type: PacketType::Literal,
                                                contents: Literal { value: 15 }
                                            },
                                            Packet {
                                                version: 2,
                                                packet_type: PacketType::Literal,
                                                contents: Literal { value: 15 }
                                            }
                                        ]
                                    }
                                }]
                            }
                        }]
                    }
                }
            )),
            parse_result
        );
        let packet = parse_result.unwrap().1;
        assert_eq!(31, packet.version_sum());
    }

    #[test]
    fn test_value_1() {
        let bytes = parse("C200B40A82").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(3, packet.value());
    }
    #[test]
    fn test_value_2() {
        let bytes = parse("04005AC33890").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(54, packet.value());
    }
    #[test]
    fn test_value_3() {
        let bytes = parse("880086C3E88112").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(7, packet.value());
    }
    #[test]
    fn test_value_4() {
        let bytes = parse("CE00C43D881120").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(9, packet.value());
    }
    #[test]
    fn test_value_5() {
        let bytes = parse("D8005AC2A8F0").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(1, packet.value());
    }
    #[test]
    fn test_value_6() {
        let bytes = parse("F600BC2D8F").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(0, packet.value());
    }
    #[test]
    fn test_value_7() {
        let bytes = parse("9C005AC2F8F0").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(0, packet.value());
    }
    #[test]
    fn test_value_8() {
        let bytes = parse("9C0141080250320F1802104A08").unwrap().1;
        let parse_result = parse_data(&bytes);
        let packet = parse_result.unwrap().1;
        assert_eq!(1, packet.value());
    }
}
