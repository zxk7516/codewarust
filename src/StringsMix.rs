/// [link](https://www.codewars.com/kata/5629db57620258aa9d000014/train/rust)
use std::collections::HashMap;

fn mix(s1: &str, s2: &str) -> String {
    let s1 = s1.clone().to_lowercase();
    let s2 = s2.clone().to_lowercase();
    let mut m = HashMap::new();
    s1.chars().for_each(|c| {
        if c.is_alphabetic() {
            let counter = m.entry(c).or_insert((0, 0));
            (*counter).0 += 1;
        }
    });
    s2.chars().for_each(|c| {
        if c.is_alphabetic() {
            let counter = m.entry(c).or_insert((0, 0));
            (*counter).1 += 1;
        }
    });
    let mut r = vec![];
    for (c, (i, j)) in m {
        if i > 1 && j > 1 {
            let sub = (i as i32) - (j as i32);
            r.push( &format!(
                "{}:{}",
                match sub {
                    sub if sub > 0 => "1",
                    sub if sub < 0 => "2",
                    _ => "=",
                },
                c.to_string().repeat(max(i, j))
            ));
        }
    }

    r.join("/")
}
fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {
    testing(
        "Are they here",
        "yes, they are here",
        "2:eeeee/2:yy/=:hh/=:rr",
    );
    testing(
        "looping is fun but dangerous",
        "less dangerous than coding",
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
    );
    testing(
        " In many languages",
        " there's a pair of functions",
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
    );
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing(
        "A generation must confront the looming ",
        "codewarrs",
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
    );
}
