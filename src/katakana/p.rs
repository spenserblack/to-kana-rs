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

pub fn p(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ぱ'),
        Some((_, 'i')) => katakana.push('ぴ'),
        Some((_, 'u')) => katakana.push('ぷ'),
        Some((_, 'e')) => katakana.push('ぺ'),
        Some((_, 'o')) => katakana.push('ぽ'),
        Some((_, 'y')) => {
            katakana.push('ぴ');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
