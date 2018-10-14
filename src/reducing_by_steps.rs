fn som(x: i64, y: i64) -> i64 {
    x + y
}
fn maxi(x: i64, y: i64) -> i64 {
    if x > y {
        x
    } else {
        y
    }
}
fn mini(x: i64, y: i64) -> i64 {
    if x < y {
        x
    } else {
        y
    }
}
fn gcdi(m: i64, n: i64) -> i64 {
    let a = m.abs();
    let b = n.abs();
    let mut m = maxi(a, b);
    let mut n = mini(a, b);
    let mut z = 0;
    while n != 0 {
        z = m %n;
        m = n;
        n = z;
    }
    m
}
fn lcmu(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    a * b / gcdi(a, b)
}

// first parameter: dots have to be replaced by function of two variables
fn oper_array(f: fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
    let mut r = vec![];
    let mut acc = init;
    for i in a {
        acc = f(acc, *i);
        r.push(acc);
    }
    r
}

fn testing_som(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(som, a, 0), exp);
}
fn testing_lcmu(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(lcmu, a, a[0]), exp);
}
fn testing_gcdi(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(gcdi, a, a[0]), exp);
}
fn testing_maxi(a: &[i64], exp: &Vec<i64>) -> () {
    assert_eq!(&oper_array(maxi, a, a[0]), exp);
}

#[test]
fn basics_som() {
    testing_som(&[18, 69, -90, -78, 65, 40], &vec![18, 87, -3, -81, -16, 24]);
}
#[test]
fn basics_lcmu() {
    testing_lcmu(
        &[18, 69, -90, -78, 65, 40],
        &vec![18, 414, 2070, 26910, 26910, 107640],
    );
}
#[test]
fn basics_maxi() {
    testing_maxi(&[18, 69, -90, -78, 65, 40], &vec![18, 69, 69, 69, 69, 69]);
}
#[test]
fn basics_gcdi() {
    testing_gcdi(&[18, 69, -90, -78, 65, 40], &vec![18, 3, 3, 3, 1, 1]);
}
