use std::str::FromStr;

use super::*;

const INPUT_A: &str = "[[[[[9,8],1],2],3],4]";
const INPUT_B: &str = "[[1,2],[[3,4],5]]";
const INPUT_C: &str = "[[6,[5,[4,[3,2]]]],1]";

const A: &[Num] = &[
    Num {
        n: 9,
        role: Role::Left,
        depth: 5,
    },
    Num {
        n: 8,
        role: Role::Right,
        depth: 5,
    },
    Num {
        n: 1,
        role: Role::Right,
        depth: 4,
    },
    Num {
        n: 2,
        role: Role::Right,
        depth: 3,
    },
    Num {
        n: 3,
        role: Role::Right,
        depth: 2,
    },
    Num {
        n: 4,
        role: Role::Right,
        depth: 1,
    },
];

const B: &[Num] = &[
    Num {
        n: 1,
        role: Role::Left,
        depth: 2,
    },
    Num {
        n: 2,
        role: Role::Right,
        depth: 2,
    },
    Num {
        n: 3,
        role: Role::Left,
        depth: 3,
    },
    Num {
        n: 4,
        role: Role::Right,
        depth: 3,
    },
    Num {
        n: 5,
        role: Role::Right,
        depth: 2,
    },
];

#[test]
fn test_parse() {
    for (input, expected) in [(INPUT_A, A), (INPUT_B, B)] {
        assert_eq!(
            NumTree::from_str(input),
            Ok(NumTree {
                inner: expected.to_vec()
            })
        )
    }
}

#[test]
fn test_decode_encode_is_identity() {
    for input in [INPUT_A, INPUT_B, INPUT_C] {
        assert_eq!(
            NumTree::from_str(input).expect("input is ok").to_string(),
            input
        )
    }
}

#[test]
fn test_add() {
    let a = NumTree::from_str("[1,2]").unwrap();
    let b = NumTree::from_str("[[3,4],5]").unwrap();
    let r = NumTree::from_str("[[1,2],[[3,4],5]]").unwrap();
    assert_eq!(a + b, r)
}

#[test]
fn test_explode() {
    for (input, exploded) in [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
    ] {
        let mut input = NumTree::from_str(input).unwrap();
        input.explode_once();
        assert_eq!(input.to_string(), exploded);
    }
}

#[test]
fn test_split() {
    let mut input = NumTree::from_str("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    input.explode_once();
    println!("{:?}", input);
    assert_eq!(&input.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");
    input.split_once();
    assert_eq!(&input.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
}

#[test]
fn test_reduce() {
    let mut input = NumTree::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    input.reduce();
    assert_eq!(&input.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
}
#[test]
fn test_magnitude() {
    for (input, m) in [
        ("[9,1]", 29),
        ("[1,9]", 21),
        ("[[9,1],[1,9]]", 129),
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        (
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        ),
    ] {
        assert_eq!(NumTree::from_str(input).unwrap().checksum(), m);
    }
}

#[test]
fn test_homework() {
    let input = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";
    let expected = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";
    let expected_num = NumTree::from_str(expected).unwrap();
    let final_num = input
        .trim()
        .lines()
        .map(|l| NumTree::from_str(l.trim()).unwrap())
        .reduce(|num_a, num_b| {
            let mut n = num_a + num_b;
            n.reduce();
            n
        })
        .unwrap();

    assert_eq!(expected_num, final_num);
    // assert_eq!(expected, final_num.to_string());
    assert_eq!(final_num.checksum(), 4140);
}
