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
    small::small_y,
};

pub fn g(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('が'),
        Some((_, 'i')) => katakana.push('ぎ'),
        Some((_, 'u')) => katakana.push('ぐ'),
        Some((_, 'e')) => katakana.push('げ'),
        Some((_, 'o')) => katakana.push('ご'),
        Some((_, 'y')) => {
            katakana.push('ぎ');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
