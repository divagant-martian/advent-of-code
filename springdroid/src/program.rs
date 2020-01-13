use intcode::program::{Int, Program as VM};
use std::collections::HashMap;

pub const SPEED: &str = "WALK\n";

pub enum Operator {
    And,
    Or,
    Not,
}

impl Operator {
    pub fn eval(&self, x: bool, y: bool) -> bool {
        match self {
            Operator::And => x && y,
            Operator::Or => x || y,
            Operator::Not => !x,
        }
    }

    pub fn to_string(&self) -> String {
        let rep = match self {
            Operator::And => "AND",
            Operator::Or => "OR",
            Operator::Not => "NOT",
        };
        String::from(rep)
    }
}

pub struct Statement {
    op: Operator,
    r: char,
    w: char,
}

impl Statement {
    pub fn to_string(&self) -> String {
        let mut rep = self.op.to_string();
        rep.push(' ');
        rep.push(self.r);
        rep.push(' ');
        rep.push(self.w);
        rep.push('\n');
        rep
    }
}

pub struct Program {
    statements: Vec<Statement>,
    truth_table: HashMap<Vec<bool>, bool>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program {
            statements,
            truth_table: HashMap::new(),
        }
    }

    pub fn test_script(intp: &Vec<Int>, script: &String, output: &mut Vec<Int>) {
        let mut input = script.as_bytes().iter().rev().map(|&c| c as Int).collect();
        let mut prog = VM::new(intp, &mut input, output);
        prog.run();
    }
    pub fn test(&self, intp: &Vec<Int>) -> Result<Int, (Vec<bool>, bool)> {
        let input = self.springscript();
        let mut output = vec![];
        Program::test_script(intp, &input, &mut output);
        let last = output.pop().unwrap();
        if last > 256 {
            return Ok(last);
        }
        let test_case = gen_test_from_failure(&output);
        Err(test_case)
    }

    pub fn springscript(&self) -> String {
        self.statements
            .iter()
            .map(Statement::to_string)
            .collect::<String>()
            + SPEED
    }
}

pub fn gen_test_from_failure(output: &Vec<Int>) -> (Vec<bool>, bool) {
    let landscape = output
        .into_iter()
        .map(|&i| i as u8 as char)
        .collect::<String>();
    println!("{}", landscape);
    let landscape = landscape
        .lines()
        .last()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    // for w in landscape.windows(4) {
    //     let scenario = w.iter().collect::<String>();
    //     if scenario.contains('@') {
    //         println!("{}", scenario.replace("@", "."));
    //         let mut s = scenario
    //             .char_indices()
    //             .fold(String::from("("), |acc, (i, c)| {
    //                 let mut var = if c == '#' {
    //                     String::new()
    //                 } else {
    //                     String::from("not ")
    //                 };
    //                 var += match i {
    //                     0 => "A",
    //                     1 => "B",
    //                     2 => "C",
    //                     3 => "D",
    //                     _ => unreachable!(),
    //                 };
    //                 if i != 3 {
    //                     acc + " " + &var + " and"
    //                 } else {
    //                     acc + " " + &var + " ) => "
    //                 }
    //             });
    //         //
    //         if scenario.chars().last().unwrap() == '#' {
    //             s += "true";
    //         } else {
    //             s += "false";
    //         }
    //         // );
    //         println!("({}) and", s);
    //     }
    // }
    // ##@#
    // #@##
    // @###
    (vec![], false)
}
