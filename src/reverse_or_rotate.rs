/// [codeward](https://www.codewars.com/kata/reverse-or-rotate/train/rust)
fn revrot(s: &str, c: usize) -> String {
    const RADIX: u32 = 10;
    let len = s.len();
    if c == 0 || len == 0 {
        return "".to_string();
    }
    let chunk_num = len / c;
    let mut r = String::with_capacity(chunk_num * c);
    for i in 0..chunk_num {
        if s[i * c..(i + 1) * c]
            .chars()
            .map(|c| c.to_digit(RADIX).unwrap() )
            .sum::<u32>()
            % 2
            == 0
        {
            for cc in s[i * c..(i + 1) * c].chars().rev() {
                r.push(cc);
            }

            
        } else {
            for cc in s[i * c + 1..(i + 1) * c].chars() {
                r.push(cc);
            }
            r.push(s.chars().nth(i * c).unwrap());
        }
    }

    r
}

fn revrot2(s: &str, c: usize) -> String {
    let mut result = String::with_capacity(s.len());
    if c > 0 {
        for i in 0 .. s.len() / c {
            let i = c * i;
            result += &transform(&s[i..i+c]);
        }
    }
    result
}

fn transform(s: &str) -> String {
    let sum: u32 = s.chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|x| x.pow(3))
        .sum();
    if sum % 2 == 0 {
        s.chars().rev().collect()
    } else {
        String::from(&s[1..]) + &s[0..1]
    }
}

fn testing(s: &str, c: usize, exp: &str) -> () {
    assert_eq!(&revrot(s, c), exp)
}

#[test]
fn basics_revrot() {
    testing("1234", 0, "");
    testing("", 0, "");
    testing("1234", 5, "");
    let s = "733049910872815764";
    testing(s, 5, "330479108928157");
    let s = "73304991087281576455176044327690580265896";
    testing(s, 8, "1994033775182780067155464327690480265895");
}
