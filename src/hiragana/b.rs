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

pub fn b(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('ば'),
        Some((_, 'i')) => hiragana.push('び'),
        Some((_, 'u')) => hiragana.push('ぶ'),
        Some((_, 'e')) => hiragana.push('べ'),
        Some((_, 'o')) => hiragana.push('ぼ'),
        Some((_, 'y')) => {
            hiragana.push('び');
            small_y(hiragana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
