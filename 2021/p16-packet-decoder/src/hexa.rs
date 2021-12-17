pub fn to_binary(c: char) -> impl Iterator<Item = bool> {
    match c.to_ascii_uppercase() {
        '0' => [false, false, false, false].into_iter(),
        '1' => [false, false, false, true].into_iter(),
        '2' => [false, false, true, false].into_iter(),
        '3' => [false, false, true, true].into_iter(),
        '4' => [false, true, false, false].into_iter(),
        '5' => [false, true, false, true].into_iter(),
        '6' => [false, true, true, false].into_iter(),
        '7' => [false, true, true, true].into_iter(),
        '8' => [true, false, false, false].into_iter(),
        '9' => [true, false, false, true].into_iter(),
        'A' => [true, false, true, false].into_iter(),
        'B' => [true, false, true, true].into_iter(),
        'C' => [true, true, false, false].into_iter(),
        'D' => [true, true, false, true].into_iter(),
        'E' => [true, true, true, false].into_iter(),
        'F' => [true, true, true, true].into_iter(),
        _ => panic!("char is not a hexadecimal digit"),
    }
}
