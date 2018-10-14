fn john(n: i32) -> Vec<i32> {
    let mut r = vec![];
    let mut r_ann = vec![];
    for i in 0..n {
        if i < 2 {
            r.push(0i32);
            r_ann.push(1i32);
        }else{
            let r_i_1 = r[(i-1) as usize];
            let rann_i_1  = r_ann[(i-1) as usize];
            r.push(i-r_ann[r_i_1 as usize]);
            r_ann.push(i-r[rann_i_1 as usize]);
        }
    }
    r
}
fn ann(n: i32) -> Vec<i32> {
    let mut r = vec![];
    let mut r_ann = vec![];
    for i in 0..n {
        if i < 2 {
            r.push(0i32);
            r_ann.push(1i32);
        }else{
            let r_i_1 = r[(i-1) as usize];
            let rann_i_1  = r_ann[(i-1) as usize];
            r.push(i-r_ann[r_i_1 as usize]);
            r_ann.push(i-r[rann_i_1 as usize]);
        }
    }
    r_ann
}
fn sum_john(n: i32) -> i32 {
    john(n).iter().fold(0, |acc, i| acc + i)
}
fn sum_ann(n: i32) -> i32 {
    ann(n).iter().fold(0, |acc, i| acc + i)
}

fn test_john(n: i32, exp: Vec<i32>) -> () {
    assert_eq!(john(n), exp)
}
fn test_ann(n: i32, exp: Vec<i32>) -> () {
    assert_eq!(ann(n), exp)
}
fn test_sum_john(n: i32, exp: i32) -> () {
    assert_eq!(sum_john(n), exp)
}
fn test_sum_ann(n: i32, exp: i32) -> () {
    assert_eq!(sum_ann(n), exp)
}

#[test]
fn test_test_john() {
    test_john(11, vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]);
    test_john(14, vec![0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 8]);
}
#[test]
fn test_test_ann() {
    test_ann(6, vec![1, 1, 2, 2, 3, 3]);
    test_ann(15, vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6, 7, 8, 8, 9]);
}
#[test]
fn test_test_sum_john() {
    test_sum_john(75, 1720);
    test_sum_john(78, 1861);
}
#[test]
fn test_test_sum_ann() {
    test_sum_ann(115, 4070);
    test_sum_ann(150, 6930);
}
