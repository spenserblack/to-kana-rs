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

pub fn z(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ざ'),
        Some((_, 'i')) => katakana.push('じ'),
        Some((_, 'u')) => katakana.push('ず'),
        Some((_, 'e')) => katakana.push('ぜ'),
        Some((_, 'o')) => katakana.push('ぞ'),
        Some((_, 'y')) => {
            katakana.push('じ');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
