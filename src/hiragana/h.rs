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

pub fn h(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('は'),
        Some((_, 'i')) => hiragana.push('ひ'),
        Some((_, 'u')) => hiragana.push('ふ'),
        Some((_, 'e')) => hiragana.push('へ'),
        Some((_, 'o')) => hiragana.push('ほ'),
        Some((_, 'y')) => {
            hiragana.push('ひ');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
