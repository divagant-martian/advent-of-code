use crate::payload::{Op, Payload};
use crate::Error;

use super::counting_iter::{CountingIter, CountingIterator};

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub version: u8,
    pub payload: Payload,
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        let mut sum = self.version as usize;
        if let Payload::Recur { ref packets, .. } = self.payload {
            for p in packets {
                sum += p.version_sum();
            }
        }
        sum
    }

    pub fn eval(&self) -> usize {
        match &self.payload {
            Payload::Recur { op, packets } => match op {
                Op::Sum => packets.iter().map(Packet::eval).sum(),
                Op::Prod => packets.iter().map(Packet::eval).product(),
                Op::Min => packets.iter().map(Packet::eval).min().unwrap(),
                Op::Max => packets.iter().map(Packet::eval).max().unwrap(),
                Op::GreT => (packets[0].eval() > packets[1].eval()) as usize,
                Op::LessT => (packets[0].eval() < packets[1].eval()) as usize,
                Op::Eq => (packets[0].eval() == packets[1].eval()) as usize,
            },
            Payload::Base(n) => *n,
        }
    }
}

impl std::str::FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode_inner(&mut CountingIter::new(
            s.chars().flat_map(crate::hexa::to_binary),
        ))
    }
}

impl Packet {
    pub fn decode_inner(bits: &mut impl CountingIterator<bool>) -> Result<Packet, Error> {
        let mut version = 0;
        for _ in 0..3 {
            version *= 2;
            if bits.next().ok_or("Missing bit in version")? {
                version += 1;
            }
        }

        let mut kind = 0;
        for _ in 0..3 {
            kind *= 2;
            if bits.next().ok_or("Missing bit in type id")? {
                kind += 1;
            }
        }

        let payload = Payload::new(kind, bits)?;
        Ok(Packet { version, payload })
    }
}

#[cfg(test)]
mod tests {
    use crate::payload::Op;

    use super::*;

    #[test]
    fn test_decide_binary() {
        let packet: Packet = "D2FE28".parse().unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 6,
                payload: Payload::Base(2021)
            }
        );
        assert_eq!(packet.version_sum(), 6);
    }

    #[test]
    fn test_recursive() {
        let packet: Packet = "38006F45291200".parse().unwrap();
        assert_eq!(
            packet,
            Packet {
                version: 1,
                payload: Payload::Recur {
                    op: Op::LessT,
                    packets: vec![
                        Packet {
                            version: 6,
                            payload: Payload::Base(10)
                        },
                        Packet {
                            version: 2,
                            payload: Payload::Base(20)
                        }
                    ]
                }
            }
        );

        assert_eq!(packet.version_sum(), 9);
    }

    #[test]
    fn test_eval() {
        for (input, eval_result) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            assert_eq!(input.parse::<Packet>().unwrap().eval(), eval_result);
        }
    }
}
