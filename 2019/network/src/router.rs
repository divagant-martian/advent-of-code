use crate::channel::{BufReceiver, BufSender};
use intcode::program::{ProgReceiver, ProgSender};
use std::collections::HashMap;
use std::thread;

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
        let mut last = None;
        let mut last_y_sent = None;
        let mut queue_had_stuff = false;
        loop {
            thread::sleep_ms(60); // let them sync
            queue_had_stuff = false;
            for (sender, out) in self.output_receivers.iter_mut() {
                // exhaust the queues
                while let Some(address) = out.get() {
                    queue_had_stuff = true;
                    let address = address as usize;
                    let x = out.get().unwrap();
                    let y = out.get().unwrap();
                    let p = vec![x, y];
                    if address == 255 {
                        // intercept the packages
                        last = Some((x, y));
                        continue;
                    }
                    let dest = self.input_senders.get_mut(&address).unwrap();
                    dest.put(x);
                    dest.put(y);
                }
            }
            if !queue_had_stuff {
                if let Some((x, y)) = last {
                    // println!("idle, sending: y={}", y);
                    let dest = self.input_senders.get_mut(&0).unwrap();
                    dest.put(x);
                    dest.put(y);
                    if let Some(ly) = last_y_sent {
                        if ly == y {
                            println!("last y sent is same as now: {}", y);
                            // abrutly stop the router
                            break;
                        }
                    }
                    last_y_sent = Some(y);
                }
            }
        }
    }
}
