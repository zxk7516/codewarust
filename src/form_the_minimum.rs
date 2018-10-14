/// [link](https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust)

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    let mut r = 0;
    let mut p = 0;
    for i in digits {
        if p != i {
            r = r * 10 + i;
        }
        p = i;
    }
    r
}

fn min_value2(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    
    digits.into_iter().fold(0, |acc, x| acc * 10 + x)
}

#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
