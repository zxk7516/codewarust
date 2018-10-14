/// [Ball Upwards](https://www.codewars.com/kata/566be96bb3174e155300001b/train/rust)
fn max_ball(v0: i32) -> i32 {
    ((v0 as f64) / 3.6 / 9.81 * 10.0).round() as i32
}
fn testequal(v0: i32, exp: i32) -> () {
    println!("{}-{}", exp, max_ball(v0));
    assert_eq!(exp, max_ball(v0))
}

#[test]
fn test() {
    testequal(15, 4);
    testequal(37, 10);
    testequal(45, 13);
}
