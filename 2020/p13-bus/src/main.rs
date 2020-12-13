fn gcd(
    a: isize,
    b: isize,
) -> (
    isize, /*gcd*/
    isize, /* coef1 */
    isize, /* coef 2 */
) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r.div_euclid(r);
        let mid_r = old_r;
        old_r = r;
        r = mid_r - quotient * r;

        let mid_s = old_s;
        old_s = s;
        s = mid_s - quotient * s;

        let mid_t = old_t;
        old_t = t;
        t = mid_t - quotient * t;
    }
    (old_r, old_s, old_t)
}

type V = Vec<isize>;

fn main() {
    let data = "19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13";

    let chris = "19,x,x,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,821,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,463,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";
    // let data = "17,x,13,19";
    let (n_i, a_i): (V, V) = chris
        .split(',')
        .enumerate()
        .filter(|(i, n)| *n != "x")
        .map(|(i, n)| {
            let n = n.parse::<isize>().unwrap();
            (n, n - i as isize)
        })
        .unzip();

    let N: isize = n_i.iter().product();
    let N_i: V = n_i.iter().map(|n| N / n).collect();

    let x: isize = n_i
        .into_iter()
        .zip(N_i)
        .zip(a_i)
        .map(|((n_small, n_big), a)| {
            let (_gcd, _m_small, m_big) = gcd(n_small, n_big);
            a * n_big * m_big
        })
        .sum();

    println!("Hello, world! {:?}, {:?}", x, x.rem_euclid(N));
}
