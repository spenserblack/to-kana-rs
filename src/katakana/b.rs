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

pub fn b(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ば'),
        Some((_, 'i')) => katakana.push('び'),
        Some((_, 'u')) => katakana.push('ぶ'),
        Some((_, 'e')) => katakana.push('べ'),
        Some((_, 'o')) => katakana.push('ぼ'),
        Some((_, 'y')) => {
            katakana.push('び');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
