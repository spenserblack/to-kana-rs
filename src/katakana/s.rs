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

pub fn s(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('さ'),
        Some((_, 'i')) => katakana.push('し'),
        Some((_, 'u')) => katakana.push('す'),
        Some((_, 'e')) => katakana.push('せ'),
        Some((_, 'o')) => katakana.push('そ'),
        Some((_, 'y')) => {
            katakana.push('し');
            small_y(katakana, characters)?;
        }
        Some((_, 'h')) => {
            katakana.push('し');
            match characters.next() {
                Some((_, 'a')) => katakana.push(small::YA),
                Some((_, 'u')) => katakana.push(small::YU),
                Some((_, 'o')) => katakana.push(small::YO),
                Some((_, 'e')) => katakana.push(small::E),
                Some((_, 'i')) => {},
                Some((i, c)) => return Err(unexpected_char_error(i, c)),
                None => return Err(unexpected_end_of_string()),
            }
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
