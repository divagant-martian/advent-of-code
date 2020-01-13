use intcode::get_data_from_path;
use intcode::program::Program;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let path = "./data/input.txt";
    let path = "./data/chris.txt";
    // let x_start = 424 - 2;
    let x_start = 1020;
    // let y_start = 964 - 3;
    let y_start = 1650;
    let grid_size = 100;
    let mut pic = HashMap::new();
    for x in x_start..x_start + grid_size {
        for y in y_start..y_start + grid_size {
            let (insendr, inrecvr) = channel();
            let (outsendr, outrcvr) = channel();
            thread::spawn(move || {
                insendr.send(1 * x);
                insendr.send(1 * y);
                let data = get_data_from_path(path);
                let mut prog = Program::new(&data, inrecvr, outsendr);
                prog.run();
            });
            let b = outrcvr.recv().expect("droid did not answer");
            pic.insert((x, y), b);
        }
    }
    let mut count = 0;
    for y in y_start..y_start + grid_size {
        let mut row = format!("y {:04}", y);
        // let mut is_min = false;
        // let mut last = 3;
        for x in x_start..x_start + grid_size {
            match pic.get(&(x, y)).expect("pos not found") {
                1 => {
                    // if is_min {
                    // row.push('-');
                    // } else {
                    // is_min = true;
                    row.push('#');
                    // }
                    // last = 1;
                    count += 1;
                }
                _ => {
                    // if last == 1 {
                    // row.pop();
                    // row.push('#');
                    // row.push('.');
                    // } else {
                    row.push('.');
                    // }
                    // last = 0;
                }
            }
        }
        println!("{}", row);
    }
    println!(
        "from x:{} and y:{} it works on {} points",
        x_start, y_start, count
    );
    // println!("Pic: {:?}", pic);
}
