mod channel;
mod router;

use intcode::get_data_from_path;
use intcode::program::{Int, ProgReceiver, ProgSender, Program};
use std::thread;

fn main() {
    const in_size: usize = 2;
    const out_size: usize = 3;
    const default_in: Option<Int> = Some(-1);
    const default_out: Option<Int> = None;
    const n_comp: usize = 50;

    // create the cables
    let mut cats = vec![]; // pairs of in_sender, out_receiver
    let mut peripherals = vec![];

    for id in 0..n_comp {
        let id = id as Int;
        let (mut i_s, mut i_r) = channel::buf_channel(id, in_size, default_in);
        let (mut o_s, mut o_r) = channel::buf_channel(id, out_size, default_out);
        cats.push((i_s, o_r));
        peripherals.push((i_r, o_s));
    }

    // create the router by pluging the cables
    thread::spawn(move || {
        let mut router = router::Router::new(cats, out_size, in_size);
        router.start();
    });
    // create the computers pluging the other end of the cables
    let data = get_data_from_path("data/input.txt");
    let mut handles = vec![];
    for (i_r, o_s) in peripherals {
        let mut prog = Program::new(&data, i_r, o_s);
        let h = thread::spawn(move || {
            prog.run();
        });
        handles.push(h);
    }

    // wait for all the computers to finish (do they?)
    for h in handles {
        h.join().unwrap();
    }
    // receive the messages
}
