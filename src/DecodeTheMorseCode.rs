// Preloaded:
//
use std::collections::HashMap;
struct MorseDecoder {
    morse_code: HashMap<String, String>,
}
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

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

    fn decode_morse(&self, encoded: &str) -> String {
        let mut space = true;
        encoded
            .split(" ")
            .into_iter()
            .fold("".to_string(), |acc, code| {
                if code == "" {
                    if space {
                        space = false;
                        return acc + " ";
                    } else {
                        return acc;
                    }
                } else {
                    space = true;
                    acc + self.morse_code.get(code).unwrap_or(&"".to_string())
                }
            }).trim().to_string()
    }
}

#[test]
fn test_hey_jude() {
    let decoder = MorseDecoder::new();
    assert_eq!(
        decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
        "HEY JUDE"
    );
}
