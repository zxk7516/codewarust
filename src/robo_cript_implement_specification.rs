#[derive(Debug)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

pub fn execute(code: &str) -> String {
  let mut d = Direction::Right;

  let mut code2 = String::with_capacity(code.len());
  let mut number_str_count = 0;
  let mut m_c = ' ';
  for c in code.chars() {
    if c.is_numeric() {
      number_str_count = number_str_count * 10 + c.to_string().parse::<i32>().unwrap();
    } else {
      if number_str_count > 0 {
        for _ in 0..(number_str_count - 1) {
          code2.push(m_c);
        }
        number_str_count = 0;
      }
      m_c = c;
      code2.push(c);
    }
  }
  if number_str_count > 0 {
    for _ in 0..(number_str_count - 1) {
      code2.push(m_c);
    }
  }

  let mut lines = vec![];
  let mut y = 0 as usize;
  let mut x = 0 as usize;
  lines.push(String::from("*"));

  for c in code2.chars() {
    match c {
      'F' => match d {
        Direction::Left => {
          if x == 0 {
            for i in 0..lines.len() {
              if i == y {
                lines[i].insert(0, '*')
              } else {
                lines[i].insert(0, ' ')
              }
            }
          } else {
            x = x - 1;
            lines[y] = lines[y]
              .chars()
              .enumerate()
              .map(|(i, c)| if i == x { '*' } else { c })
              .collect::<String>();
          }
        }
        Direction::Down => {
          y = y + 1;
          if y >= lines.len() {
            lines.push(format!("{}{}", " ".repeat(x), "*"))
          } else {
            if x >= lines[y].len() {
              for _ in 0..(x - lines[y].len()) {
                lines[y].push(' ');
              }
              lines[y].push('*');
            } else {
              lines[y] = lines[y]
                .chars()
                .enumerate()
                .map(|(i, c)| if i == x { '*' } else { c })
                .collect::<String>();
            }
          }
        }
        Direction::Right => {
          x = x + 1;
          lines[y].push('*');
        }
        Direction::Up => {
          if y == 0 {
            lines.insert(0, format!("{}{}", " ".repeat(x), "*"));
          } else {
            y = y - 1;
            if x >= lines[y].len() {
              for _ in 0..(x - lines[y].len()) {
                lines[y].push(' ');
              }
              lines[y].push('*');
            } else {
              lines[y] = lines[y]
                .chars()
                .enumerate()
                .map(|(i, c)| if i == x { '*' } else { c })
                .collect::<String>();
            }
          }
        }
      },
      'L' => {
        d = match d {
          Direction::Left => Direction::Down,
          Direction::Down => Direction::Right,
          Direction::Right => Direction::Up,
          Direction::Up => Direction::Left,
        }
      }
      'R' => {
        d = match d {
          Direction::Left => Direction::Up,
          Direction::Up => Direction::Right,
          Direction::Right => Direction::Down,
          Direction::Down => Direction::Left,
        }
      }
      c if c.is_numeric() => {}
      _ => {}
    }
  }

  let max_len = lines
    .iter()
    .fold(0, |acc, s| if acc < s.len() { s.len() } else { acc });
  for i in 0..lines.len() {
    if lines[i].len() < max_len {
      let spaces = max_len - lines[i].len();
      for _ in 0..spaces {
        lines[i].push(' ');
      }
    }
  }
  lines.join("\r\n")
}

use std::cmp::{max, min};

pub fn execute2(code: &str) -> String {
  if code.is_empty() {
    return "*".to_string();
  }

  const DIRECTIONS: [[i64; 2]; 4] = [[0, 1], [1, -1], [0, -1], [1, 1]];

  let mut coordinates: Vec<[i64; 2]> = vec![[0, 0]];

  let mut cur: [i64; 2] = [0, 0];

  let mut chars = code.chars();
  let mut edge = [0; 4]; // left, right, top, bottom

  {
    let mut cmd = chars.next().unwrap();
    let mut times = 0i32;
    let mut direct_idx = 0i32;

    let mut do_cmd = |c: char, mut t: i32| {
      t = max(t, 1);
      match c {
        'F' => {
          let d_idx = (((direct_idx % 4) + 4) % 4) as usize;

          for _ in 0..t {
            cur[DIRECTIONS[d_idx][0] as usize] += DIRECTIONS[d_idx][1];
            coordinates.push(cur.clone());
          }

          edge[0] = min(edge[0], cur[0]);
          edge[1] = max(edge[1], cur[0]);
          edge[2] = min(edge[2], cur[1]);
          edge[3] = max(edge[3], cur[1]);
        }
        'R' => direct_idx += t,
        'L' => direct_idx -= t,
        _ => {}
      }
    };

    chars.for_each(|ch| match ch {
      'F' | 'L' | 'R' => {
        do_cmd(cmd, times);
        cmd = ch;
        times = 0;
      }
      _ => {
        times = times * 10 + ch.to_digit(10).unwrap() as i32;
      }
    });

    do_cmd(cmd, times);
  }

  let width = ((edge[1] - edge[0]) + 1) as usize;
  let height = ((edge[3] - edge[2]) + 1) as usize;

  let mut char_coords: Vec<Vec<char>> = Vec::with_capacity(height);
  for _ in 0..height {
    char_coords.push(vec![' '; width]);
  }

  coordinates
    .into_iter()
    .map(|coord| [(coord[0] - edge[0]) as usize, (coord[1] - edge[2]) as usize])
    .for_each(|coord| {
      char_coords[height - 1 - coord[1]][coord[0]] = '*';
    });

  char_coords
    .into_iter()
    .map(|chars| chars.into_iter().collect::<String>())
    .collect::<Vec<String>>()
    .join("\r\n")
}
#[cfg(test)]
macro_rules! expect_equal {
  ($actual:expr, $expected:expr $(,)*) => {{
    let actual = $actual;
    let expected = $expected;

    assert_eq!(
      actual, expected,
      "\ngot:\n{}\n\nexpected:\n{}\n",
      actual, expected
    );
  }};
}

#[test]
fn examples_in_description() {
  expect_equal!(execute(""), "*");
  expect_equal!(execute("FFFFF"), "******");
  expect_equal!(
    execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
    "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
  );

  expect_equal!(
    execute("LFFFFFRFFFRFFFRFFFFFFF"),
    "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
  );
  expect_equal!(
    execute("LF5RF3RF3RF7"),
    "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
  );
}
