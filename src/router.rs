use crate::channel::{BufReceiver, BufSender};
use intcode::program::{ProgReceiver, ProgSender};
use std::collections::HashMap;

pub struct Router {
    output_receivers: HashMap<usize, BufReceiver>,
    input_senders: HashMap<usize, BufSender>,
    out_size: usize,
    in_size: usize,
}

impl Router {
    pub fn new(cats: Vec<(BufSender, BufReceiver)>, out_size: usize, in_size: usize) -> Self {
        let mut output_receivers = HashMap::new();
        let mut input_senders = HashMap::new();
        for (in_s, mut out_r) in cats {
            let address = out_r.get().unwrap() as usize;
            input_senders.insert(address, in_s);
            output_receivers.insert(address, out_r);
        }
        Router {
            output_receivers,
            input_senders,
            out_size,
            in_size,
        }
    }

    pub fn start(&mut self) {
        loop {
            for (sender, out) in self.output_receivers.iter_mut() {
                if let Some(address) = out.get() {
                    let address = address as usize;
                    let x = out.get().unwrap();
                    let y = out.get().unwrap();
                    let p = vec![x, y];
                    println!("address {} sent {},{} to {}", sender, x, y, address);
                    let dest = self.input_senders.get_mut(&address).unwrap();
                    dest.put(x);
                    dest.put(y);
                }
            }
        }
    }
}
