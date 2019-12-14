// Doddy
fn zeros_x(n: u64) -> u64 {
    let mut five = 0;
    let mut two = 0;
    'outer: for mut i in 1..n + 1 {
        loop {
            let is_even = i % 2 == 0;
            let is_five = i % 5 == 0;
            if !is_five && !is_even {
                continue 'outer;
            }
            if is_even {
                two += 1;
                i /= 2;
            }
            if is_five {
                five += 1;
                i /= 5;
            }
        }
    }
    two.min(five)
}

fn zeros(n: u64) -> u64 {
    match n {
        0 | 1 | 2 | 3 | 4 => 0,
        _ => n / 5 + zeros(n / 5),
    }
}

#[test]
fn sample_tests() {
    assert_eq!(zeros(0), 0);
    assert_eq!(zeros(6), 1);
    assert_eq!(zeros(14), 2);
    assert_eq!(zeros(30), 7);
    assert_eq!(zeros(1000), 249);
    assert_eq!(zeros(100000), 24999);
    assert_eq!(zeros(1000000000), 249999998);
}
