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

pub fn z(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('ざ'),
        Some((_, 'i')) => hiragana.push('じ'),
        Some((_, 'u')) => hiragana.push('ず'),
        Some((_, 'e')) => hiragana.push('ぜ'),
        Some((_, 'o')) => hiragana.push('ぞ'),
        Some((_, 'y')) => {
            hiragana.push('じ');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
