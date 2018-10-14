fn parse(code: &str) -> Vec<i32> {
    let mut r = vec![];
    let mut sum = 0;
    for i in code.chars() {
        match i {
            'i' => {sum=sum+1;},
            'd' => {sum=sum-1;},
            's' => {sum=sum*sum},
            'o' => {r.push(sum);},
            _ => {},
        }
    }
    r
}

#[test]
fn sample_tests() {
    assert_eq!(parse("iiisdoso"),
        vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"),
        vec![8, 64, 3600]);
}