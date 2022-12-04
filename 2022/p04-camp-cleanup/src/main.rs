use std::ops::RangeInclusive;

fn main() {
    let file_name = std::env::args().nth(1).expect("Provide a file name");
    let file_contents = std::fs::read_to_string(file_name).expect("File exists");
    let data = parse_data(&file_contents);
    dbg!(problem_2(data));
}

fn problem_1<T>(data: T) -> usize
where
    T: Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)>,
{
    data.filter(|(ra, rb)| {
        let ra0 = ra.start();
        let ra1 = ra.end();
        let rb0 = rb.start();
        let rb1 = rb.end();
        (ra.contains(&rb0) && ra.contains(&rb1)) || (rb.contains(&ra0) && rb.contains(&ra1))
    })
    .count()
}

fn overlaps(ra: &RangeInclusive<u32>, rb: &RangeInclusive<u32>) -> bool {
    let ra0 = ra.start();
    let ra1 = ra.end();
    let rb0 = rb.start();
    let rb1 = rb.end();
    ra.contains(&rb0) || ra.contains(&rb1) || rb.contains(&ra0) || rb.contains(&ra1)
}

fn problem_2<T>(data: T) -> usize
where
    T: Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)>,
{
    data.filter(|(ra, rb)| overlaps(ra, rb) || overlaps(rb, ra))
        .count()
}

fn parse_data(data: &str) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    data.lines().map(|l| {
        let mut parts = l
            .split(|c| c == '-' || c == ',')
            .map(|p| p.parse::<u32>().unwrap());
        (
            parts.next().unwrap()..=parts.next().unwrap(),
            parts.next().unwrap()..=parts.next().unwrap(),
        )
    })
}
