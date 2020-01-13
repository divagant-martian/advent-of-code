mod program;

use intcode::get_data_from_path;

fn main() {
    let data = get_data_from_path("data/input.txt");
    let inst = "\
NOT A J
NOT C T
OR T J
NOT B T
OR T J
AND D J
WALK
";
    let inst = "\
NOT A J
NOT C T
OR T J
NOT B T
OR T J
AND D J
OR E T
OR H T
AND T J
RUN
";
    let inst = String::from(inst);
    let mut out = vec![];
    let r = program::Program::test_script(&data, &inst, &mut out);
    let last = out.pop().unwrap();
    if last > 256 {
        println!("{}", last);
    }
    let test_case = program::gen_test_from_failure(&mut out);

    println!("{:?}", r);
}
