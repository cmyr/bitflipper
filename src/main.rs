use std::string::FromUtf8Error;

fn main() {
    let args = std::env::args().skip(1);
    for arg in args {
        println!("###### flipping {} ######", arg);
        for (i, result) in arg.iter_flips().enumerate() {
            if let Ok(r) = result {
                println!("{}: '{}'", i, r);
            }
        }
    }
}

trait BitFlippable {
    fn iter_flips<'a>(&'a self) -> FlipIter<'a>;
}

impl BitFlippable for str {
    fn iter_flips<'a>(&'a self) -> FlipIter<'a> {
        FlipIter::new(self)
    }
}

fn flip(byte: u8, bit: u8) -> u8 {
    assert!(bit <= 8);
    let mask = 1 << bit;
    let flipped = !byte & mask;
    let rest = byte & !mask;
    flipped ^ rest
}

struct FlipIter<'a> {
    text: &'a str,
    char_idx: usize,
    bit: u8,
}

impl<'a> FlipIter<'a> {
    fn new(text: &'a str) -> Self {
        FlipIter {
            text,
            char_idx: 0,
            bit: 0,
        }
    }
}

impl<'a> Iterator for FlipIter<'a> {
    type Item = Result<String, FromUtf8Error>;

    fn next(&mut self) -> Option<Result<String, FromUtf8Error>> {
        if self.char_idx == self.text.len() {
            return None;
        }
        let new_byte = flip(self.text.as_bytes()[self.char_idx], self.bit);
        let this_char = self.char_idx;
        self.bit += 1;

        if self.bit == 8 {
            self.bit = 0;
            self.char_idx += 1;
        }

        let mut bytes = self.text.as_bytes().to_owned();
        bytes[this_char] = new_byte;
        Some(String::from_utf8(bytes))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_bit() {
        let a_byte = b'a';
        assert_eq!(a_byte, 0b01100001);
        assert_eq!(flip(a_byte, 0), 0b01100000);
        assert_eq!(flip(a_byte, 1), 0b01100011);
        assert_eq!(flip(a_byte, 2), 0b01100101);
        assert_eq!(flip(a_byte, 3), 0b01101001);
        assert_eq!(flip(a_byte, 4), 0b01110001);
        assert_eq!(flip(a_byte, 5), 0b01000001);
        assert_eq!(flip(a_byte, 6), 0b00100001);
        assert_eq!(flip(a_byte, 7), 0b11100001);
    }

    #[test]
    fn works_with_iter() {
        let flips = "a".iter_flips().collect::<Vec<_>>();
        assert_eq!(flips.len(), 8);

        assert_eq!(flips[0].as_ref().unwrap(), String::from_utf8([0b01100000].to_vec()).unwrap().as_str());
        assert_eq!(flips[1].as_ref().unwrap(), String::from_utf8([0b01100011].to_vec()).unwrap().as_str());
        assert_eq!(flips[2].as_ref().unwrap(), String::from_utf8([0b01100101].to_vec()).unwrap().as_str());
        assert_eq!(flips[3].as_ref().unwrap(), String::from_utf8([0b01101001].to_vec()).unwrap().as_str());
        assert_eq!(flips[4].as_ref().unwrap(), String::from_utf8([0b01110001].to_vec()).unwrap().as_str());
        assert_eq!(flips[5].as_ref().unwrap(), String::from_utf8([0b01000001].to_vec()).unwrap().as_str());
        assert_eq!(flips[6].as_ref().unwrap(), String::from_utf8([0b00100001].to_vec()).unwrap().as_str());
        assert!(flips[7].as_ref().is_err(), "0b11100001 is not valid utf8");
    }
}
