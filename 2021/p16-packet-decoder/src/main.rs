use crate::decode::{Packet, Payload};

mod decode;

mod counting_iter;

fn main() {
    let decoded = decode::decode("D2FE28");
    assert_eq!(
        dbg!(decoded),
        Ok(Packet {
            version: 6,
            payload: Payload::Base(2021)
        })
    )
}
