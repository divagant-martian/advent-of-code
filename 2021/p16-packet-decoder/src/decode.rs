use super::counting_iter::{CountingIter, CountingIterator};
type Error = &'static str;

pub fn to_binary(c: char) -> impl Iterator<Item = bool> {
    match c {
        '0' => [false, false, false, false].into_iter(),
        '1' => [false, false, false, true].into_iter(),
        '2' => [false, false, true, false].into_iter(),
        '3' => [false, false, true, true].into_iter(),
        '4' => [false, true, false, false].into_iter(),
        '5' => [false, true, false, true].into_iter(),
        '6' => [false, true, true, false].into_iter(),
        '7' => [false, true, true, true].into_iter(),
        '8' => [true, false, false, false].into_iter(),
        '9' => [true, false, false, true].into_iter(),
        'A' => [true, false, true, false].into_iter(),
        'B' => [true, false, true, true].into_iter(),
        'C' => [true, true, false, false].into_iter(),
        'D' => [true, true, false, true].into_iter(),
        'E' => [true, true, true, false].into_iter(),
        'F' => [true, true, true, true].into_iter(),
        _ => panic!("char is not a hexadecimal digit"),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub version: u8,
    pub payload: Payload,
}

pub fn decode(bits: &mut impl CountingIterator<bool>) -> Result<Packet, Error> {
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

#[derive(Debug, PartialEq, Eq)]
pub enum Payload {
    Recur(Vec<Packet>),
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
                packets.push(decode(bits)?);
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
                packets.push(decode(bits)?);
            }

            packets
        };
        Ok(packets)
    }

    pub fn new(kind: u8, bits: &mut impl CountingIterator<bool>) -> Result<Self, Error> {
        match kind {
            4 => Ok(Payload::Base(Payload::base(bits)?)),
            _ => Ok(Payload::Recur(Payload::recur(bits)?)),
        }
    }
}

pub fn input_iter(input: &str) -> impl CountingIterator<bool> + '_ {
    CountingIter::new(input.chars().flat_map(to_binary))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decide_binary() {
        assert_eq!(
            decode(&mut input_iter("D2FE28")),
            Ok(Packet {
                version: 6,
                payload: Payload::Base(2021)
            })
        )
    }

    #[test]
    fn test_recursive() {
        assert_eq!(
            decode(&mut input_iter("38006F45291200")),
            Ok(Packet {
                version: 1,
                payload: Payload::Recur(vec![
                    Packet {
                        version: 6,
                        payload: Payload::Base(10)
                    },
                    Packet {
                        version: 2,
                        payload: Payload::Base(20)
                    }
                ])
            })
        )
    }
}
