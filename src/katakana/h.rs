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

pub fn h(katakana: &mut String, characters: &mut Peekable<Enumerate<Chars>>) -> Result<(), Error> {
    match characters.next() {
        Some((_, 'a')) => katakana.push('ハ'),
        Some((_, 'i')) => katakana.push('ヒ'),
        Some((_, 'u')) => katakana.push('フ'),
        Some((_, 'e')) => katakana.push('ヘ'),
        Some((_, 'o')) => katakana.push('ホ'),
        Some((_, 'y')) => {
            katakana.push('ヒ');
            small_y(katakana, characters)?;
        }
        Some((i, c)) => return Err(unexpected_char_error(i, c)),
        None => return Err(unexpected_end_of_string()),
    }
    Ok(())
}
