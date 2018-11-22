pub fn highlight(code: &str) -> String {
  if code.len() <= 0 {
    return "".to_string();
  }
  let mut maps = vec![];
  maps.push(code.chars().next().unwrap().to_string());
  let mut len = 1;
  code.chars().skip(1).for_each(|c| {
    if c.is_numeric() && maps[len - 1].chars().next().unwrap().is_numeric() {
      maps[len - 1].push(c);
    } else if c == maps[len - 1].chars().next().unwrap() {
      maps[len - 1].push(c)
    } else {
      maps.push(c.to_string());
      len = len + 1;
    }
  });
  let mut result = String::with_capacity(len * 38);
  maps.iter().for_each(|s| {
    let c = s.chars().next().unwrap();
    if c.is_numeric() {
      result.push_str(&format!("<span style=\"color: orange\">{}</span>", s));
    } else if c == 'F' {
      result.push_str(&format!("<span style=\"color: pink\">{}</span>", s));
    } else if c == 'L' {
      result.push_str(&format!("<span style=\"color: red\">{}</span>", s));
    } else if c == 'R' {
      result.push_str(&format!("<span style=\"color: green\">{}</span>", s));
    } else if c == '(' || c == ')' {
      result.push_str(s);
    } else {
    }
  });
  result
}

#[cfg(test)]
macro_rules! assert_highlight {
  ($code:expr , $expected:expr $(,)*) => {{
    let actual = highlight($code);
    let expected = $expected;
    println!("Code without syntax highlighting: {}", $code);
    println!("Your code with syntax highlighting: {}", actual);
    println!("Expected syntax highlighting: {}", expected);
    assert_eq!(actual, expected);
  }};
}

#[test]
fn examples_in_description() {
  assert_highlight!(
    "F3RF5LF7",
    "<span style=\"color: pink\">F</span><span style=\"color: orange\">3</span><span style=\"color: green\">R</span><span style=\"color: pink\">F</span><span style=\"color: orange\">5</span><span style=\"color: red\">L</span><span style=\"color: pink\">F</span><span style=\"color: orange\">7</span>",
  );
  assert_highlight!(
    "FFFR345F2LL",
    "<span style=\"color: pink\">FFF</span><span style=\"color: green\">R</span><span style=\"color: orange\">345</span><span style=\"color: pink\">F</span><span style=\"color: orange\">2</span><span style=\"color: red\">LL</span>",
  );
}
