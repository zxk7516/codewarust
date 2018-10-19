/// [Decode the Morse code, advanced](https://www.codewars.com/kata/decode-the-morse-code-advanced/train/rust)
use std::collections::HashMap;
use std::collections::HashSet;

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    fn new() -> Self {
        let mut morse_code = HashMap::new();
        let codes = vec![
            (".-", "A"),
            ("-...", "B"),
            ("-.-.", "C"),
            ("-..", "D"),
            (".", "E"),
            ("..-.", "F"),
            ("--.", "G"),
            ("....", "H"),
            ("..", "I"),
            (".---", "J"),
            ("-.-", "K"),
            (".-..", "L"),
            ("--", "M"),
            ("-.", "N"),
            ("---", "O"),
            (".--.", "P"),
            ("--.-", "Q"),
            (".-.", "R"),
            ("...", "S"),
            ("-", "T"),
            ("..-", "U"),
            ("...-", "V"),
            (".--", "W"),
            ("-..-", "X"),
            ("-.--", "Y"),
            ("--..", "Z"),
            (".----", "1"),
            ("..---", "2"),
            ("...--", "3"),
            ("....-", "4"),
            (".....", "5"),
            ("-....", "6"),
            ("--...", "7"),
            ("---..", "8"),
            ("----.", "9"),
            ("-----", "0"),
            ("..--..", "?"),
            ("-..-.", "/"),
            ("-.--.-", "()"),
            ("-....-", "-"),
            (".-.-.-", "."),
        ];
        for (a, b) in codes {
            morse_code.insert(a.to_string(), b.to_string());
        }
        MorseDecoder { morse_code }
    }
    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|x| {
                x.split(' ')
                    .filter_map(|y| self.morse_code.get(y))
                    .cloned()
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn decode_bits(&self, encoded: &str) -> String {
        if encoded.len() <= 0 {
            return "".to_string();
        }
        let encoded = encoded.trim_matches(|c: char| c == '0');
        let repeat = self.guess_repeat(encoded) as usize;
        println!("{}, {}", repeat, encoded);
        let dash = "1".repeat(repeat * 3 );
        let dot = "1".repeat(repeat);
        let white3 = "0".repeat(repeat * 7);
        let white = "0".repeat(repeat * 3);
        let split = "0".repeat(repeat);

        encoded
            .trim()
            .replace(&dash, "-")
            .replace(&dot, ".")
            .replace(&white3, "   ")
            .replace(&white, " ")
            .replace(&split, "")
    }

    pub fn guess_repeat(&self, encoded: &str) -> i32 {
        let mut repeat = 1;
        let mut repeat_1 = 0;
        let mut repeat_0 = vec![];
        let c2 = encoded.trim_matches(|c: char| c == '0');
        let mut c_c = c2.chars().nth(0).unwrap();
        let mut it = c2.chars().skip(1);
        while let Some(c) = it.next() {
            if c == c_c {
                repeat = repeat + 1;
            } else {
                if c == '1' {
                    if repeat % 3 != 0 {
                        return repeat;
                    }
                    if repeat_1 == 0 {
                        repeat_1 = repeat;
                    } else if repeat != repeat_1 {
                        return if repeat > repeat_1 { repeat_1 } else { repeat };
                    }
                } else if c == '0' {
                    if repeat % 3 != 0 && repeat != 7 {
                        return repeat;
                    } else {
                        if repeat_0.len() == 0 {
                            repeat_0.push(repeat);
                        } else if repeat_0.len() == 1 {
                            let mut repeat_0_0 = repeat_0[0];
                            let mut m = repeat_0_0 % repeat;;
                            while m > 0 {
                                m = repeat_0_0 % repeat;
                                repeat_0_0 = repeat;
                                repeat = m;
                            }
                            return repeat;
                        }
                    }
                }
                repeat = 1;
                c_c = c;
            }
        }
        repeat
    }
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn examples() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
}
