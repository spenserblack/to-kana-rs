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

pub fn s(hiragana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => hiragana.push('さ'),
        Some((_, 'i')) => hiragana.push('し'),
        Some((_, 'u')) => hiragana.push('す'),
        Some((_, 'e')) => hiragana.push('せ'),
        Some((_, 'o')) => hiragana.push('そ'),
        Some((_, 'y')) => {
            hiragana.push('し');
            small_y(hiragana, characters)?;
        }
        Some((_, 'h')) => {
            hiragana.push('し');
            match characters.next() {
                Some((_, 'a')) => hiragana.push(small::YA),
                Some((_, 'u')) => hiragana.push(small::YU),
                Some((_, 'o')) => hiragana.push(small::YO),
                Some((_, 'e')) => hiragana.push(small::E),
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
