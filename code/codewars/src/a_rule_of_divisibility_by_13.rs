const DIGITS: [i64; 6] = [1, 10, 9, 12, 3, 4];

fn go(mut n: i64, prev: i64) -> i64 {
    let mut v = vec![];
    while n >= 10 {
        v.push(n % 10);
        n /= 10;
    }
    v.push(n);
    if *v.last().unwrap() == 0 {
        v.pop();
    }
    let mut prod = 0;
    for (i, d) in v.into_iter().enumerate() {
        prod += DIGITS[i % 6] * d;
    }
    if prod == prev {
        prod
    } else {
        go(prod, prod)
    }
}

fn thirt(n: i64) -> i64 {
    go(n, 0)
}
fn testequal(n: i64, exp: i64) -> () {
    assert_eq!(exp, thirt(n))
}

//#[test]
fn basics_thirt() {
    testequal(8529, 79);
    testequal(85299258, 31);
    testequal(5634, 57);
}
