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
    assert!(bit <= 7);
    let mask = 1 << bit;
    let flipped = !byte & mask;
    let rest = byte & !mask;
    flipped ^ rest
}

struct FlipIter<'a> {
    text: &'a str,
    char_idx: usize,
    bit: u8,
    buf: Vec<u8>,
}

impl<'a> FlipIter<'a> {
    fn new(text: &'a str) -> Self {
        FlipIter {
            text,
            char_idx: 0,
            bit: 0,
            buf: text.as_bytes().to_owned(),
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

        self.buf.clear();
        self.buf.extend_from_slice(self.text.as_bytes());
        self.buf[this_char] = new_byte;

        Some(String::from_utf8(self.buf.clone()))
    }
}
