fn going(n: i32) -> f64 {
    match n {
        1 => 1.0,
        _ => 1.0 + going(n - 1) / n as f64,
    }
}

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-6;
    let inrange = if expected == 0.0 {
        (actual.abs() <= merr)
    } else {
        ((actual - expected).abs() / expected <= merr)
    };
    if inrange == false {
        println!(
            "Expected value must be near: {:e} but was:{:e}",
            expected, actual
        );
    } else {
        //println!("....... GOOD\n");
    }
    assert_eq!(true, inrange);
}

fn dotest(n: i32, exp: f64) -> () {
    assert_fuzzy_equals(going(n), exp);
}

//#[test]
fn basics_going() {
    dotest(5, 1.275);
    dotest(6, 1.2125);
    dotest(7, 1.173214);
    dotest(8, 1.146651);
}
