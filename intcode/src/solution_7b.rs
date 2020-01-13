use crate::program::{Int, ProgReceiver, ProgSender, Program};
use itertools::Itertools;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

impl ProgSender for Sender<Int> {
    fn put(&mut self, num: Int) {
        self.send(num).unwrap();
    }
}

impl ProgReceiver for Receiver<Int> {
    fn get(&mut self) -> Option<Int> {
        self.recv().ok()
    }
}

fn try_config(data: &Vec<Int>, phases: Vec<Int>) -> Option<Int> {
    let (sender_a, receiver_b) = channel();
    let (sender_b, receiver_c) = channel();
    let (sender_c, receiver_d) = channel();
    let (sender_d, receiver_e) = channel();
    let (sender_e, receiver_a) = channel();
    sender_e.send(phases[0]).unwrap(); //send to a
    sender_e.send(0).unwrap(); //send to a
    sender_a.send(phases[1]).unwrap();
    sender_b.send(phases[2]).unwrap();
    sender_c.send(phases[3]).unwrap();
    sender_d.send(phases[4]).unwrap();
    let mut a = Program::new(data, receiver_a, sender_a);
    let mut b = Program::new(data, receiver_b, sender_b);
    let mut c = Program::new(data, receiver_c, sender_c);
    let mut d = Program::new(data, receiver_d, sender_d);
    let mut e = Program::new(data, receiver_e, sender_e);
    let thread_a = thread::spawn(move || {
        a.run();
        a.peak_input().recv().unwrap()
    });
    let thread_b = thread::spawn(move || {
        b.run();
    });
    let thread_c = thread::spawn(move || {
        c.run();
    });
    let thread_d = thread::spawn(move || {
        d.run();
    });
    let thread_e = thread::spawn(move || {
        e.run();
    });
    if thread_e.join().is_ok()
        && thread_d.join().is_ok()
        && thread_c.join().is_ok()
        && thread_b.join().is_ok()
    {
        thread_a.join().ok()
    } else {
        None
    }
}

pub fn run_solution(data: Vec<Int>, _debug: bool) -> Int {
    let mut max = 0;
    for phase_setting in (5..=9).permutations(5) {
        max = try_config(&data, phase_setting).unwrap().max(max);
    }
    println!("{:?}", max);
    max
}
