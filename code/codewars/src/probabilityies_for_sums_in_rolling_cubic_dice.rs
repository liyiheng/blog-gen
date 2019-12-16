fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    count(sum, dice_amount) as f64 / 6_i32.pow(dice_amount as u32) as f64
}

fn count(sum: i32, dice_amount: i32) -> i32 {
    if sum <= 0 {
        return 0;
    }
    if dice_amount == 1 {
        if sum > 6 {
            return 0;
        } else {
            return 1;
        }
    }
    (1..7)
        .into_iter()
        .fold(0, |acc, i| acc + count(sum - i, dice_amount - 1))
}

fn assert_fuzzy_eq(actual: f64, expected: f64, eps: f64) {
    assert!(
        (actual - expected).abs() < eps,
        format!("Expected {}, but got {}", expected, actual)
    );
}

// #[test]
fn returns_expected() {
    assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
}
