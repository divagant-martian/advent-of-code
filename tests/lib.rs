extern crate intcode;
use intcode::program::{Int, Program};
use intcode::{get_data_from_path, get_data_from_str};

use intcode::{solution_7a, solution_7b};

#[test]
fn test_02() {
    let tests: Vec<(&str, &[Int])> = vec![
        (
            "1,9,10,3,2,3,11,0,99,30,40,50",
            &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ),
        ("1,0,0,0,99", &[2, 0, 0, 0, 99]),
        ("2,3,0,3,99", &[2, 3, 0, 6, 99]),
        ("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801]),
        ("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99]),
    ];
    for (mem_in, mem_out) in tests {
        let data = get_data_from_str(mem_in);
        let mut input = vec![];
        let mut output = vec![];
        let mut prog = Program::new(&data, &mut input, &mut output);
        prog.run();

        assert_eq!(prog.peak_mem(), mem_out);
    }
}

#[test]
fn test_02a_final() {
    let mut data = get_data_from_path("data/day02_final.txt");
    data[1] = 12;
    data[2] = 2;
    let mut input = vec![];
    let mut output = vec![];
    let mut prog = Program::new(&data, &mut input, &mut output);
    prog.run();

    assert_eq!(prog.peak_mem()[0], 3306701);
}

#[test]
fn test_02b_final() {
    let mut data = get_data_from_path("data/day02_final.txt");
    data[1] = 76;
    data[2] = 21;
    let mut input = vec![];
    let mut output = vec![];
    let mut prog = Program::new(&data, &mut input, &mut output);
    prog.run();

    assert_eq!(prog.peak_mem()[0], 19690720);
}

#[test]
fn test_05a() {
    let data = get_data_from_str("1002,4,3,4,33");
    let mut input = vec![];
    let mut output = vec![];
    let mut prog = Program::new(&data, &mut input, &mut output);
    prog.run();

    assert_eq!(prog.peak_mem(), &[1002, 4, 3, 4, 99]);
}

#[test]
fn test_05a_final() {
    let data = get_data_from_path("data/day05_final.txt");
    let mut input = vec![1];
    let mut output = vec![];
    Program::new(&data, &mut input, &mut output).run();
    assert_eq!(output, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 7286649]);
}

#[test]
fn test_05b() {
    let tests = vec![
        // consider if the input is 8; output 1 (if it is) or 0 (if it is not)
        ( "3,9,8,9,10,9,4,9,99,-1,8", 8, 1 ),
        ( "3,9,8,9,10,9,4,9,99,-1,8", 7, 0 ),
        // consider if input < 8; output 1 (if it is) or 0 (if it is not)
        ( "3,9,7,9,10,9,4,9,99,-1,8", 7, 1 ),
        ( "3,9,7,9,10,9,4,9,99,-1,8", 8, 0 ),
        // consider if input == 8; output 1 (if it is) or 0 (if it is not)
        ( "3,3,1108,-1,8,3,4,3,99", 8, 1 ),
        ( "3,3,1108,-1,8,3,4,3,99", 5, 0 ),
        // consider if input < 8; output 1 (if it is) or 0 (if it is not)
        ( "3,3,1107,-1,8,3,4,3,99", 7, 1 ),
        ( "3,3,1107,-1,8,3,4,3,99", 8, 0),
        // output 0 if the input was zero or 1 if the input was non-zero
        ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0),
        ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 56, 1),
        // output 999 if input < 8; 1000 if input == 8, 1001 if input > 8
        ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7, 999),
        ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8, 1000),
        ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 9, 1001)];

    for (input_str, single_input, single_output) in tests {
        let data = get_data_from_str(input_str);
        let mut input = vec![single_input];
        let mut output = vec![];
        Program::new(&data, &mut input, &mut output).run();
        assert_eq!(&output, &[single_output]);
    }
}

#[test]
fn test_05b_final() {
    let data = get_data_from_path("data/day05_final.txt");
    let mut input = vec![5];
    let mut output = vec![];
    Program::new(&data, &mut input, &mut output).run();
    assert_eq!(output, vec![15724522]);
}

#[test]
fn test_07a() {
    let tests = vec![
        (
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
            [4, 3, 2, 1, 0],
            43210,
        ),
        (
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
            [0, 1, 2, 3, 4],
            54321,
        ),
        (
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
            [1, 0, 4, 3, 2],
            65210,
        ),
    ];

    for (input_str, max_phase, max) in tests {
        let data = get_data_from_str(input_str);
        let (p_max, p_max_perm) = solution_7a::run_solution(&data, false);
        assert_eq!(p_max, max);
        assert_eq!(p_max_perm, max_phase);
    }
}

#[test]
fn test_07a_final() {
    let data = get_data_from_path("data/day07_final.txt");
    let (p_max, p_max_perm) = solution_7a::run_solution(&data, false);
    assert_eq!(p_max, 92663);
    assert_eq!(p_max_perm, vec![3, 1, 4, 2, 0]);
}

#[test]
fn test_07b() {
    let tests = vec![
        (
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            139629729,
        ),
        (
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
            18216,
        ),
    ];

    for (input_str, max) in tests {
        let data = get_data_from_str(input_str);
        let p_max = solution_7b::run_solution(data, false);
        assert_eq!(p_max, max);
    }
}

#[test]
fn test_07b_final() {
    let data = get_data_from_path("data/day07_final.txt");
    let p_max = solution_7b::run_solution(data, false);
    assert_eq!(p_max, 14365052);
}

#[test]
fn test_09a() {
    let tests: Vec<(&str, &[Int])> = vec![
        (
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            &[
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
        ),
        ("1102,34915192,34915192,7,4,7,99,0", &[1219070632396864]),
        ("104,1125899906842624,99", &[1125899906842624]),
    ];
    for (input_str, expected_out) in tests {
        let data = get_data_from_str(input_str);
        let mut input = vec![];
        let mut output = vec![];
        Program::new(&data, &mut input, &mut output).run();
        assert_eq!(output, expected_out);
    }
}
