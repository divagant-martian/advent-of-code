mod graph;

use crate::graph::CaveSystem;

fn main() {
    let input: CaveSystem = "
        start-co
        ip-WE
        end-WE
        le-ls
        wt-zi
        end-sz
        wt-RI
        wt-sz
        zi-start
        wt-ip
        YT-sz
        RI-start
        le-end
        ip-sz
        WE-sz
        le-WE
        le-wt
        zi-ip
        RI-zi
        co-zi
        co-le
        WB-zi
        wt-WE
        co-RI
        RI-ip
    "
    .try_into()
    .expect("Input is ok");
    println!("Paths: {}", input.iter().count());
    println!("Paths: {}", input.iter().with_enough_time().count());
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_1: &str = "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ";

    const INPUT_2: &str = "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";

    const INPUT_3: &str = "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ";

    #[test]
    fn find_paths() {
        for (input, expected) in [(INPUT_1, 226), (INPUT_2, 10), (INPUT_3, 19)] {
            test_paths(input, expected);
        }
    }

    #[test]
    fn find_paths_with_time() {
        test_paths_with_time(INPUT_1, 3509);
        test_paths_with_time(INPUT_3, 103);
        test_paths_with_time(INPUT_2, 36);
    }

    fn test_paths(input: &str, expected_count: usize) {
        let input: CaveSystem = input.try_into().expect("Path is ok");
        assert_eq!(input.iter().count(), expected_count);
    }

    fn test_paths_with_time(input: &str, expected_count: usize) {
        let input: CaveSystem = input.try_into().expect("Path is ok");
        assert_eq!(input.iter().with_enough_time().count(), expected_count);
    }
}
