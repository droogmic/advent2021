use bitvec::prelude::*;
use take_until::TakeUntilExt;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
enum PacketType {
    Literal,
    Operator,
}

impl std::convert::TryFrom<u8> for PacketType {
    type Error = ParseError;
    fn try_from(type_id: u8) -> Result<Self, Self::Error> {
        Ok(match type_id {
            4 => Self::Literal,
            _ => Self::Operator,
        })
    }
}

type PacketBits = BitSlice<Lsb0, usize>;

#[derive(Debug)]
pub struct Header {
    version: u8,
    packet_type: PacketType,
}

impl std::convert::TryFrom<&PacketBits> for Header {
    type Error = ParseError;
    fn try_from(header_bits: &PacketBits) -> Result<Self, Self::Error> {
        assert_eq!(header_bits.len(), 6);
        Ok(Self {
            version: header_bits[0..3].load::<u8>(),
            packet_type: header_bits[4..6].load::<u8>().try_into()?,
        })
    }
}

#[derive(Debug)]
enum PacketLength {
    Bits(usize),
    Packets(usize),
}

impl std::convert::TryFrom<&PacketBits> for PacketLength {
    type Error = ParseError;
    fn try_from(bits: &PacketBits) -> Result<Self, Self::Error> {
        Ok(match bits[0] {
            false => Self::Bits(
                bits.get(1..=15)
                    .ok_or_else(|| ParseError::Str("not enough bits".to_owned()))?
                    .load::<usize>(),
            ),
            true => Self::Packets(
                bits.get(1..=11)
                    .ok_or_else(|| ParseError::Str("not enough bits".to_owned()))?
                    .load::<usize>(),
            ),
        })
    }
}

#[derive(Debug)]
pub enum PacketData {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    data: PacketData,
}

fn recursive_parse(bits: &PacketBits) -> Result<(Packet, &PacketBits), ParseError> {
    let Header {
        version,
        packet_type,
    } = bits[0..6].try_into()?;
    let data = match packet_type {
        PacketType::Literal => {
            // the binary number is padded with leading zeroes
            // until its length is a multiple of four bits,
            // and then it is broken into groups of four bits
            let literal_bits: BitVec<Lsb0, usize> = bitvec![];
            for group in bits[6..]
                .chunks_exact(5)
                // each group is prefixed by a 1 bit except the last group,
                // which is prefixed by a 0 bit
                .take_until(|literal_part| literal_part.first().unwrap() == false)
            {
                literal_bits.extend_from_bitslice(&group[1..5])
            }
            PacketData::Literal(literal_bits.load::<usize>())
        }
        PacketType::Operator => {
            // let length: PacketLength = bits.try_into()?;
            let packets = vec![];
            match bits[6..].try_into()? {
                PacketLength::Bits(bit_count) => {
                    let next_packet = recursive_parse()
                    packets
                },
                PacketLength::Packets(packet_count) => unreachable!(),
            }
            PacketData::Operator()
        }
    };
    Ok((Packet { version, data }, bits))
}

impl std::convert::TryFrom<&PacketBits> for Packet {
    type Error = ParseError;
    fn try_from(bits: &PacketBits) -> Result<Self, Self::Error> {
        let Header {
            version,
            packet_type,
        } = bits[0..6].try_into()?;
        let data = match packet_type {
            PacketType::Literal => {
                // the binary number is padded with leading zeroes
                // until its length is a multiple of four bits,
                // and then it is broken into groups of four bits
                let literal_bits: BitVec<Lsb0, usize> = bitvec![];
                for group in bits[6..]
                    .chunks_exact(5)
                    // each group is prefixed by a 1 bit except the last group,
                    // which is prefixed by a 0 bit
                    .take_until(|literal_part| literal_part.first().unwrap() == false)
                {
                    literal_bits.extend_from_bitslice(&group[1..5])
                }
                PacketData::Literal(literal_bits.load::<usize>())
            }
            PacketType::Operator => {
                let length: PacketLength = bits.try_into()?;
                PacketData::Operator(vec![])
            }
        };
        Ok(Packet { version, data })
    }
}

pub fn parse(input: &str) -> ParseResult<Packet> {
    Ok(Packet {})
}

pub fn part1(something: &Packet) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub fn part2(something: &Packet) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<Packet, usize> = Day {
    title: "TITLE",
    display: (
        "Foobar foobar foobar {answer}",
        "Foobar foobar foobar {answer}",
    ),
    calc: DayCalc {
        parse: parse,
        part1,
        part2,
    },
    example: include_str!("../examples/day00.txt"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input;
    use test_log::test;

    #[test]
    fn test_example_part1() {
        let something = parse(DAY.example).unwrap();
        let result = play(&something);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_example_part2() {
        let something = parse(DAY.example).unwrap();
        let result = play(&something);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(0)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "-1");
        assert_eq!(part2(&something).answer.to_string(), "-1");
    }
}
