/// [Decode the Morse code, advanced](https://www.codewars.com/kata/decode-the-morse-code-advanced/train/rust)
use std::collections::HashMap;
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
    pub fn decode_bits(&self, encoded: &str) -> String {
        unimplemented!()
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        unimplemented!()
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
