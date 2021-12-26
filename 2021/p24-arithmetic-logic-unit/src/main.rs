#![recursion_limit = "524"]
use rayon::prelude::*;

fn main() {
    let mut input = [9, 9, 9, 8, 9, 6, 9, 2, 6, 9, 9, 4, 3, 9];
    input.reverse();
    println!("{:?}", program(&input));
}

fn find_largest() -> [isize; 14] {
    for v1 in (1isize..=9).rev() {
        for v2 in (1isize..=9).rev() {
            for v3 in (1isize..=9).rev() {
                for v4 in (1isize..=9).rev() {
                    for v5 in (1isize..=9).rev() {
                        for v6 in (1isize..=9).rev() {
                            for v7 in (1isize..=9).rev() {
                                for v8 in (1isize..=9).rev() {
                                    for v9 in (1isize..=9).rev() {
                                        for v10 in (1isize..=9).rev() {
                                            for v11 in (1isize..=9).rev() {
                                                for v12 in (1isize..=9).rev() {
                                                    for v13 in (1isize..=9).rev() {
                                                        if let Some(largest) = (1isize..10)
                                                            .into_par_iter()
                                                            .rev()
                                                            .map(|v14| {
                                                                [
                                                                    v1, v2, v3, v4, v5, v6, v7, v8,
                                                                    v9, v10, v11, v12, v13, v14,
                                                                ]
                                                            })
                                                            .find_any(|x| program(&x) == 0)
                                                        {
                                                            return largest;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

#[inline(never)]
#[no_mangle]
pub fn program(vec: &[isize; 14]) -> isize {
    let mut w: isize = 0;
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut z: isize = 0;
    let mut idx: usize = 0;

    macro_rules! compile {
        () => {
            return z;
        };
        (inp $var: ident; $($tail: tt)*) => {
            println!("z{} {}", idx, z);
            $var = vec[idx];
            idx += 1;


            compile!($($tail)*)
        };
        (add $var: ident $to_add: expr; $($tail: tt)*) => {
            $var += $to_add;
            compile!($($tail)*)
        };
        (mul $var: ident $to_mul: expr; $($tail: tt)*) => {
            $var *= $to_mul;
            compile!($($tail)*)
        };
        (div $var: ident $to_div: expr; $($tail: tt)*) => {
            $var /= $to_div;
            compile!($($tail)*)
        };
        (mod $var: ident $to_mod: expr; $($tail: tt)*) => {
            $var %= $to_mod;
            compile!($($tail)*)
        };
        (eql $var: ident $to_cmp: expr; $($tail: tt)*) => {
            $var = if $var == $to_cmp {
                1
            } else {
                0
            };
            compile!($($tail)*)
        };
    }
    compile! {
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 13;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 14;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 12;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 8;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 11;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 5;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x 0;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 4;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 15;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 10;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x -13;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 13;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 10;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 16;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x -9;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 5;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 11;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 6;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 1;
        add x 13;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 13;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x -14;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 6;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x -3;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 7;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x -2;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 13;
        mul y x;
        add z y;
        inp w;
        mul x 0;
        add x z;
        mod x 26;
        div z 26;
        add x -14;
        eql x w;
        eql x 0;
        mul y 0;
        add y 25;
        mul y x;
        add y 1;
        mul z y;
        mul y 0;
        add y w;
        add y 3;
        mul y x;
        add z y;
    }
}
