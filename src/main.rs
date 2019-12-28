mod channel;
mod router;

use intcode::get_data_from_path;
use intcode::program::{ProgReceiver, ProgSender};
use std::thread;

fn main() {
    let in_size = 2;
    let out_size = 3;
    let default_in = Some(888);
    let default_out = None;
    // computer 1
    let (mut in_s_1, mut in_r_1) = channel::buf_channel(1, in_size, default_in);
    let (mut out_s_1, mut out_r_1) = channel::buf_channel(1, out_size, default_out);

    // computer 2
    let (mut in_s_2, mut in_r_2) = channel::buf_channel(2, in_size, default_in);
    let (mut out_s_2, mut out_r_2) = channel::buf_channel(2, out_size, default_out);

    // computer 3
    let (mut in_s_3, mut in_r_3) = channel::buf_channel(8, in_size, default_in);
    let (mut out_s_3, mut out_r_3) = channel::buf_channel(8, out_size, default_out);

    // send some messages
    out_s_1.put_package(vec![8, 1, 8]); // 1 sends (1,3) to 3
    out_s_2.put_package(vec![2, 2, 2]); // 2 sends (2,2) to 2
    out_s_3.put_package(vec![2, 8, 2]); // 3 sends (3,2) to 2

    // create the router
    thread::spawn(move || {
        let channels = vec![
            (in_s_1, out_r_1), // computer 1
            (in_s_2, out_r_2), // computer 2
            (in_s_3, out_r_3), // computer 3
        ];

        let mut router = router::Router::new(channels, out_size, in_size);
        router.start();
    });

    thread::sleep_ms(1000);

    // receive the messages
    println!("1 received {:?}", in_r_1.get());
    println!("1 received {:?}", in_r_1.get());
    println!("1 received {:?}", in_r_1.get());
    println!("1 received {:?}", in_r_1.get());
    println!("1 received {:?}", in_r_1.get());
    println!("1 received {:?}", in_r_1.get());
    println!("1 received {:?}", in_r_1.get());

    println!("2 received {:?}", in_r_2.get());
    println!("2 received {:?}", in_r_2.get());
    println!("2 received {:?}", in_r_2.get());
    println!("2 received {:?}", in_r_2.get());
    println!("2 received {:?}", in_r_2.get());
    println!("2 received {:?}", in_r_2.get());
    println!("2 received {:?}", in_r_2.get());

    println!("3 received {:?}", in_r_3.get());
    println!("3 received {:?}", in_r_3.get());
    println!("3 received {:?}", in_r_3.get());
    println!("3 received {:?}", in_r_3.get());
    println!("3 received {:?}", in_r_3.get());
    println!("3 received {:?}", in_r_3.get());
    println!("3 received {:?}", in_r_3.get());
}
