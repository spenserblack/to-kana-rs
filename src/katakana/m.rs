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

pub fn m(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ま'),
        Some((_, 'i')) => katakana.push('み'),
        Some((_, 'u')) => katakana.push('む'),
        Some((_, 'e')) => katakana.push('め'),
        Some((_, 'o')) => katakana.push('も'),
        Some((_, 'y')) => {
            katakana.push('み');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
