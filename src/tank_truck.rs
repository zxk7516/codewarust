/// [link](https://www.codewars.com/kata/tank-truck/train/rust)
fn tank_vol(h: i32, d: i32, vt: i32) -> i32 {
    
    0
}

fn dotest(h: i32, d: i32, vt: i32, exp: i32) -> () {
    assert_eq!(tank_vol(h, d, vt), exp)
}
#[test]
fn basics_tank_vol() {
    dotest(5, 7, 3848, 2940);
    dotest(5, 7, 3848, 2940);
    dotest(2, 7, 3848, 907);
    dotest(2, 8, 5026, 982);
    dotest(4, 9, 6361, 2731);
}