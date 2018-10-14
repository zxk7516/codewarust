fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {

    a.into_iter().filter(|s:&T| !b.contains(s)).collect()
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
    assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
    assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
}
