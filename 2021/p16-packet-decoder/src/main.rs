use crate::decode::{Packet, Payload};

mod decode;

mod counting_iter;

fn main() {
    let mut iter = decode::input_iter("D2FE28");
    let decoded = decode::decode(&mut iter);
    assert_eq!(
        dbg!(decoded),
        Ok(Packet {
            version: 6,
            payload: Payload::Base(2021)
        })
    )
}
