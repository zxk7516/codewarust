fn get_middle(s: &str) -> &str {
    
    let len = s.len();
    let start = (len - 1) / 2;
    let end = start + (if len % 2 == 0 { 2 } else { 1 });
    &s[start..end]
}

#[test]
fn example_tests() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}
