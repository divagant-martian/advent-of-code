use intcode::program::{Int, ProgReceiver, ProgSender};
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
type Package = Vec<Int>;

#[derive(Debug, Clone)]
pub struct BufSender {
    s: Sender<Package>,
    q: VecDeque<Int>,
    n: usize,
}

impl ProgSender for BufSender {
    fn put(&mut self, num: Int) {
        self.q.push_back(num);
        if self.q.len() >= self.n {
            let mut p = Vec::with_capacity(self.n);
            for _ in 0..self.n {
                p.push(self.q.pop_front().unwrap());
            }
            // println!("sending {:?}", p);
            self.s.send(p).unwrap();
        }
    }
}

impl BufSender {
    pub fn new(s: Sender<Package>, buf_size: usize) -> Self {
        BufSender {
            s,
            q: VecDeque::with_capacity(buf_size),
            n: buf_size,
        }
    }
    pub fn put_package(&mut self, p: Vec<Int>) {
        assert_eq!(p.len() == self.n, true);
        self.s.send(p).unwrap();
    }
    pub fn len(&self) -> usize {
        self.q.len()
    }
}

#[derive(Debug)]
pub struct BufReceiver {
    r: Receiver<Package>,
    q: VecDeque<Int>,
    address: Option<Int>,
    on_empty: Option<Int>,
}

impl ProgReceiver for BufReceiver {
    fn get(&mut self) -> Option<Int> {
        if let Some(address) = self.address.take() {
            Some(address)
        } else if let Some(x) = self.q.pop_front() {
            Some(x)
        } else {
            match self.r.try_recv() {
                Err(TryRecvError::Empty) => self.on_empty,
                Err(TryRecvError::Disconnected) => None,
                Ok(p) => {
                    for i in p {
                        self.q.push_back(i);
                    }
                    Some(self.q.pop_front().unwrap())
                }
            }
        }
    }
}

impl BufReceiver {
    pub fn new(r: Receiver<Package>, address: Int, on_empty: Option<Int>) -> Self {
        BufReceiver {
            r,
            q: VecDeque::new(),
            address: Some(address),
            on_empty,
        }
    }
    pub fn len(&self) -> usize {
        self.q.len()
    }
}

pub fn buf_channel(
    address: Int,
    buf_size: usize,
    on_empty: Option<Int>,
) -> (BufSender, BufReceiver) {
    let (s, r) = channel();
    (
        BufSender::new(s, buf_size),
        BufReceiver::new(r, address, on_empty),
    )
}
