use std::iter::{Enumerate, Iterator, Peekable};
use std::str::Chars;

use crate::Error;

use super::{
    small::{self, small_y},
    unexpected_char_error, unexpected_end_of_string,
};

pub fn d(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ダ'),
        Some((_, 'i')) => katakana.push('ヂ'),
        Some((_, 'u')) => katakana.push('ヅ'),
        Some((_, 'e')) => katakana.push('デ'),
        Some((_, 'o')) => katakana.push('ド'),
        Some((_, 'y')) => {
            katakana.push('ヂ');
            small_y(katakana, characters)?;
        }
        Some((_, 'h')) => {
            katakana.push('デ');
            match characters.next() {
                Some((_, 'a')) => katakana.push(small::YA),
                Some((_, 'u')) => katakana.push(small::YU),
                Some((_, 'o')) => katakana.push(small::YO),
                Some((_, 'i')) => katakana.push(small::I),
                Some((_, 'e')) => katakana.push(small::E),
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((_, 'd')) => {
            katakana.push('ッ');
            d(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
