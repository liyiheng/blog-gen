fn dig_pow(n: i64, p: i32) -> i64 {
    let zero = '0' as i64;
    let sum = n
        .to_string()
        .chars()
        .map(|c| c as i64 - zero)
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + v.pow(p as u32 + i as u32));
    if sum % n == 0 {
        sum / n
    } else {
        -1
    }
}

fn dotest(n: i64, p: i32, exp: i64) -> () {
    println!(" n: {:?};", n);
    println!("p: {:?};", p);
    let ans = dig_pow(n, p);
    println!(" actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!(" {};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
}

//#[test]
fn basic_tests() {
    dotest(89, 1, 1);
    dotest(92, 1, -1);
    dotest(46288, 3, 51);
}
