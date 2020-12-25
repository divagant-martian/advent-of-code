const P: usize = 20201227;

fn find_loop_size(final_val: usize) -> usize {
    let mut val = 1;
    const SUBJECT: usize = 7;

    for loop_size in 1..P {
        val *= SUBJECT;
        val = val.rem_euclid(P);
        if val == final_val {
            return loop_size;
        }
    }

    unreachable!()
}

fn do_loop(subject: usize, loop_size: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val *= subject;
        val = val.rem_euclid(P);
    }
    val
}

fn main() {
    let first_key = std::env::args()
        .nth(1)
        .expect("Provide the first public key (a number)")
        .parse()
        .expect("Failed to parse first number");
    let second_key = std::env::args()
        .nth(2)
        .expect("Provide the first public key (a number)")
        .parse()
        .expect("Failed to parse second number");
    let n = find_loop_size(first_key);
    let encription_key = do_loop(second_key, n);

    println!("Encription key is {}", encription_key);
}
