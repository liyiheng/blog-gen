fn persistence(num: u64) -> u64 {
    let (_, d) = go(num, 0);
    d
}

fn go(mut n: u64, d: u64) -> (u64, u64) {
    if n < 10 {
        return (n, d);
    }
    let mut v = 1;
    while n >= 10 {
        v *= n % 10;
        n /= 10;
    }
    go(n * v, d + 1)
}

//#[test]
fn sample_tests() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
}
