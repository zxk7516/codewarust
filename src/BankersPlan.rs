/// [link](https://www.codewars.com/kata/bankers-plan/train/rust)

fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    let mut r = true;
    let mut f = f0 as f64;
    let mut c = c0 as f64;
    let mut n = n;
    while n > 0 {
        n = n - 1;
        f = f + f * p / 100.0 - c;
        c = c + c * i / 100.0;
        if f < 0.0 {
            r = false;
            break;
        }
    }
    r
}

fn testequal(f0: i32, p: f64, c0: i32, n: i32, i: f64, exp: bool) -> () {
    assert_eq!(exp, fortune(f0, p, c0, n, i))
}

#[test]
fn basics() {
    testequal(100000, 1.0, 2000, 15, 1.0, true);
    testequal(100000, 1.0, 9185, 12, 1.0, false);
    testequal(100000000, 1.0, 100000, 50, 1.0, true);
    testequal(100000000, 1.5, 10000000, 50, 1.0, false);
}
