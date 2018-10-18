/// [Probabilities for Sums in Rolling Cubic Dice](https://www.codewars.com/kata/probabilities-for-sums-in-rolling-cubic-dice/train/rust)

fn rolldice_sum_prob2(sum: i32, dice_amount: i32) -> f64 {
    let n = dice_sum_count(sum, dice_amount);
    n as f64 / 6.0f64.powf(dice_amount as f64)
}

fn dice_sum_count(sum: i32, dice_amount: i32) -> i32 {
    let mut result;
    if dice_amount <= 0 {
        return 0;
    }
    if dice_amount == 1 {
        return if sum > 0 && sum < 7 { 1 } else { 0 };
    }
    result = dice_sum_count(sum - 6, dice_amount - 1);
    let order = result > 0;
    for num in 1..6 {
        let select_number = if order { 6 - num } else { num };
        let r = dice_sum_count(sum - select_number, dice_amount - 1);
        if r <= 0 {
            break;
        }
        result += r;
    }

    result
}

// #![allow(unused_parens)]
fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    if sum < dice_amount || sum > 6 * dice_amount {
        0f64
    } else if dice_amount == 0 {
        1f64
    } else {
        (1..7)
            .map(|d| rolldice_sum_prob(sum - d, dice_amount - 1))
            .sum::<f64>()
            / 6f64
    }
}

fn assert_fuzzy_eq(actual: f64, expected: f64, eps: f64) {
    assert!(
        (actual - expected).abs() < eps,
        format!("Expected {}, but got {}", expected, actual)
    );
}

#[test]
fn returns_expected() {
    assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
}

#[test]
fn returns_expected2() {
    assert_fuzzy_eq(rolldice_sum_prob2(11, 2), 0.055555555555, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob2(8, 2), 0.13888888889, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob2(8, 3), 0.0972222222222, 1e-10);
}
