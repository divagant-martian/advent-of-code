use crate::program::Program;
use itertools::Itertools;

pub fn run_solution(data: &Vec<i32>, debug: bool) {
    let perms = (0..5).permutations(5);
    let mut input = vec![];
    let mut output = vec![0];
    let mut max_perm = vec![-1; 5];
    let mut max = 0;
    for permutation in perms {
        for phase in &permutation {
            let last_out = output.pop().expect("last amplifier had no output");
            input.push(last_out);
            input.push(*phase);
            let mut program = Program::new(&data, &mut input, &mut output);
            if debug {
                program.run_debug_mode();
            } else {
                program.run();
            }
        }
        let perm_out = output.pop().expect("throusers without output");
        output.push(0);
        if perm_out > max {
            max = perm_out;
            max_perm = permutation;
        }
    }
    println!("max: {} produced by: {:?}", max, max_perm);
}
