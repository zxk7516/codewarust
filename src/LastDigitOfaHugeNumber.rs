fn last_digit(list: &[u64]) -> u64 {
    if list.len() == 0 {
        return 1;
    }
    if list.len() == 1 {
        return list[0] % 10;
    }
    let pow_0 = check_zero(&list[1..]);
    if pow_0 == 0 {
        return 1;
    }
    if pow_0 == 1 {
        return list[0];
    }
    let n = list[0] % 10;
    if n == 0 || n == 1 || n == 6 || n == 5 {
        return n;
    }
    let circle = match n {
        2 => vec![2, 4, 8, 6],
        3 => vec![3, 9, 7, 1],
        4 => vec![4, 6],
        7 => vec![7, 9, 3, 1],
        8 => vec![8, 4, 2, 6],
        9 => vec![9, 1],

        _ => vec![],
    };

    if circle.len() == 2 {
        return circle[( (pow_0-1) % 2) as usize];
    }
    if circle.len() == 4 {
        println!("{:?} : {}", circle, check_four(&list[1..])  -1);
        return circle[(check_four(&list[1..])  -1) as usize];
    }

    0
}

fn check_zero(afs: &[u64]) -> u64 {
    afs.iter()
        .rev()
        .fold(1, |acc, &b| if acc == 0 { 1 } else { b })
}


fn check_four(afs: &[u64]) -> u64 {
    if afs[0] % 2 == 1 {
        if afs[0] % 4 == 1 {
            return 1;
        }else{
            return [1,3][ (check_zero(&afs[1..])%2) as usize]
        }

    }else{
        if afs[0] % 4 == 0 {
            return 4;
        }else{
            return if check_zero(&afs[1..]) >=2  {4}else{2};
        }
    }

}

#[test]
fn basic_tests() {
    let tests = vec![
        (vec![], 1),
        (vec![0, 0], 1),
        (vec![0, 0, 0], 0),
        (vec![1, 2], 1),
        (vec![3, 4, 5], 1),
        (vec![4, 3, 6], 4),
        (vec![7, 6, 21], 1),
        (vec![12, 30, 21], 6),
        (vec![2, 2, 2, 0], 4),
        (vec![937640, 767456, 981242], 0),
        (vec![123232, 694022, 140249], 6),
        (vec![499942, 898102, 846073], 6),
    ];

    for test in tests {
        println!("{}{}", last_digit(&test.0), test.1);
        assert_eq!(last_digit(&test.0), test.1);
    }
}
