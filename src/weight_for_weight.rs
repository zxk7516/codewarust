fn order_weight(s: &str) -> String {
    let mut weight_str: Vec<&str> = s.split(' ').collect();
    weight_str.sort_by(|a,b| a.cmp(b));
    weight_str.sort_by(|a,b| num_count(a).cmp(&num_count(b)));

    weight_str.join(" ")
}

fn num_count(s: &str) -> i32 {
    s.chars()
        .map(|c: char| c.to_digit(10).unwrap())
        .fold(0, |acc, n| acc + (n as i32))
}

fn testing(s: &str, exp: &str) -> () {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
