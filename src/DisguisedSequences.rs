fn v1(n: i32, p: i32) -> i64 {
    // your code
}
fn u1(n: i32, p: i32) -> i64 {
    // your code
}

fn dotest1(n: i32, p: i32, exp: i64) -> () {
    assert_eq!(exp, v1(n, p))
}
fn dotest2(n: i32, p: i32, exp: i64) -> () {
    assert_eq!(exp, u1(n, p))
}

#[test]
fn basics_v1() {
    dotest1(5, 65, 715);
    dotest1(9, 12, 228);
    dotest1(13, 104, 2808);
}
#[test]
fn basics_u1() {
    dotest2(10, 106, 1166);
    dotest2(17, 32, 576);
}
