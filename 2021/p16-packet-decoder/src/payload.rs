use crate::counting_iter::CountingIterator;
use crate::decode::Packet;
use crate::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Sum,
    Prod,
    Min,
    Max,
    GreT,
    LessT,
    Eq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Payload {
    Recur { op: Op, packets: Vec<Packet> },
    Base(usize),
}

impl Payload {
    pub fn base(bits: &mut impl CountingIterator<bool>) -> Result<usize, Error> {
        let mut payload = 0;
        while let Some(more_packets) = bits.next() {
            for _ in 0..4 {
                payload *= 2;
                if bits.next().ok_or("Missing bit in payload group")? {
                    payload += 1;
                }
            }
            if !more_packets {
                break;
            }
        }
        Ok(payload)
    }

    pub fn recur(bits: &mut impl CountingIterator<bool>) -> Result<Vec<Packet>, Error> {
        let length_type_id = bits.next().ok_or("Missing length_type_id")?;

        let packets = if length_type_id {
            let mut subpacket_count = 0;
            for _ in 0..11 {
                subpacket_count *= 2;
                if bits.next().ok_or("Missing bit in subpacket count")? {
                    subpacket_count += 1;
                }
            }
            let mut packets = Vec::with_capacity(subpacket_count);
            while packets.len() < subpacket_count {
                packets.push(Packet::decode_inner(bits)?);
            }

            packets
        } else {
            let mut total_length = 0;
            for _ in 0..15 {
                total_length *= 2;
                if bits.next().ok_or("Missing bit in total length")? {
                    total_length += 1;
                }
            }
            let currently_used = bits.calls();

            let mut packets = Vec::new();
            while bits.calls() < currently_used + total_length {
                packets.push(Packet::decode_inner(bits)?);
            }

            packets
        };
        Ok(packets)
    }

    pub fn new(kind: u8, bits: &mut impl CountingIterator<bool>) -> Result<Self, Error> {
        match kind {
            4 => Ok(Payload::Base(Payload::base(bits)?)),
            other => {
                let packets = Payload::recur(bits)?;
                Ok(Payload::Recur {
                    packets,
                    op: match other {
                        0 => Op::Sum,
                        1 => Op::Prod,
                        2 => Op::Min,
                        3 => Op::Max,
                        5 => Op::GreT,
                        6 => Op::LessT,
                        7 => Op::Eq,
                        _ => return Err("Unexpected operation type"),
                    },
                })
            }
        }
    }
}
