use std::iter::{
    Enumerate,
    Iterator,
    Peekable,
};
use std::str::Chars;

use crate::Error;

use super::{
    unexpected_char_error,
    unexpected_end_of_string,
    small::{self, small_y},
};

pub fn d(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('だ'),
        Some((_, 'i')) => katakana.push('ぢ'),
        Some((_, 'u')) => katakana.push('づ'),
        Some((_, 'e')) => katakana.push('で'),
        Some((_, 'o')) => katakana.push('ど'),
        Some((_, 'y')) => {
            katakana.push('ぢ');
            small_y(katakana, characters)?;
        }
        Some((_, 'h')) => {
            katakana.push('で');
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
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
