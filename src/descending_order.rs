/// ## Descending Order
/// Your task is to make a function that can take any non-negative integer as a argument and return it with its digits in descending order. Essentially, rearrange the digits to create the highest possible number.
/// ### Examples:
/// Input: `21445` Output: `54421`
/// Input: `145263` Output: `654321`
/// Input: `1254859723` Output: `9875543221`   
use std::string::String;
use std::iter::FromIterator;
fn descending_order(x: u64) -> u64 {
    let sl = &format!("{}", x)[..];
    let mut chars: Vec<char> = sl.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    match  String::from_iter(chars).parse::<u64>()
    {
        Ok(u) => u,
        _ => 0,
    }
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
